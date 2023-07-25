import { Accessor, createEffect, createRenderEffect, mapArray, on, onCleanup, splitProps } from 'solid-js'
import * as display from '@pixi/display';
import * as core from '@pixi/core';
import * as mesh from '@pixi/mesh';
// import { Falsey } from '../utils/utility-types'
//
// import { useThree, useUpdate } from './hooks'
// import { catalogue } from './proxy'
// import { Stages } from './stages'

import { produce } from 'solid-js/store'
import type { ConstructorRepresentation, Instance } from './proxy'
import type { Dpr, RootState } from './store'
import { Container } from 'pixi.js'

export type NonFunctionKeys<P> = { [K in keyof P]-?: P[K] extends Function ? never : K }[keyof P]
export type Overwrite<P, O> = Omit<P, NonFunctionKeys<O>> & O
export type Properties<T> = Pick<T, NonFunctionKeys<T>>

/**
 * An SSR-friendly useLayoutEffect.
 *
 * React currently throws a warning when using useLayoutEffect on the server.
 * To get around it, we can conditionally useEffect on the server (no-op) and
 * useLayoutEffect elsewhere.
 *
 * @see https://github.com/facebook/react/issues/14927
 */
export const useIsomorphicLayoutEffect =
  typeof window !== 'undefined' && (window.document?.createElement || window.navigator?.product === 'ReactNative')
    ? createRenderEffect
    : createEffect

export interface ObjectMap {
  nodes: { [name: string]: display.Container }
  materials: { [name: string]: core.Shader }
}

export function calculateDpr(dpr: Dpr): number {
  // Err on the side of progress by assuming 2x dpr if we can't detect it
  // This will happen in workers where window is defined but dpr isn't.
  const target = typeof window !== 'undefined' ? window.devicePixelRatio ?? 2 : 1
  return Array.isArray(dpr) ? Math.min(Math.max(dpr[0], target), dpr[1]) : dpr
}

/**
 * Returns instance root state
 */
export const getRootState = (obj: Instance['object']): RootState | undefined => obj.__r3f?.root

export interface EquConfig {
  /** Compare arrays by reference equality a === b (default), or by shallow equality */
  arrays?: 'reference' | 'shallow'
  /** Compare objects by reference equality a === b (default), or by shallow equality */
  objects?: 'reference' | 'shallow'
  /** If true the keys in both a and b must match 1:1 (default), if false a's keys must intersect b's */
  strict?: boolean
}

// A collection of compare functions
export const is = {
  obj: (a: any) => a === Object(a) && !is.arr(a) && typeof a !== 'function',
  fun: (a: any): a is Function => typeof a === 'function',
  str: (a: any): a is string => typeof a === 'string',
  num: (a: any): a is number => typeof a === 'number',
  boo: (a: any): a is boolean => typeof a === 'boolean',
  und: (a: any) => a === void 0,
  arr: (a: any) => Array.isArray(a),
  equ(a: any, b: any, { arrays = 'shallow', objects = 'reference', strict = true }: EquConfig = {}) {
    // Wrong type or one of the two undefined, doesn't match
    if (typeof a !== typeof b || !!a !== !!b) return false
    // Atomic, just compare a against b
    if (is.str(a) || is.num(a)) return a === b
    const isObj = is.obj(a)
    if (isObj && objects === 'reference') return a === b
    const isArr = is.arr(a)
    if (isArr && arrays === 'reference') return a === b
    // Array or Object, shallow compare first to see if it's a match
    if ((isArr || isObj) && a === b) return true
    // Last resort, go through keys
    let i
    // Check if a has all the keys of b
    for (i in a) if (!(i in b)) return false
    // Check if values between keys match
    if (isObj && arrays === 'shallow' && objects === 'shallow') {
      for (i in strict ? b : a) if (!is.equ(a[i], b[i], { strict, objects: 'reference' })) return false
    } else {
      for (i in strict ? b : a) if (a[i] !== b[i]) return false
    }
    // If i is undefined
    if (is.und(i)) {
      // If both arrays are empty we consider them equal
      if (isArr && a.length === 0 && b.length === 0) return true
      // If both objects are empty we consider them equal
      if (isObj && Object.keys(a).length === 0 && Object.keys(b).length === 0) return true
      // Otherwise match them by value
      if (a !== b) return false
    }
    return true
  },
}

function traversePixiObject(object: display.Container, callback: (object: display.Container) => void) {
  // Apply the callback to the current object
  callback(object);

  // If the object has children, apply the callback to each of them, recursively
  if(object.children && object.children.length > 0) {
    for(let i = 0; i < object.children.length; i++) {
      traversePixiObject(object.children[i] as display.Container, callback);
    }
  }
}
// Collects nodes and materials from a THREE.Object3D
export function buildGraph(object: Container): ObjectMap {
  const data: ObjectMap = { nodes: {}, materials: {} }
  if (object) {
    traversePixiObject(object, (obj) => {
      if (obj.name) data.nodes[obj.name] = obj
      if (obj instanceof mesh.Mesh && obj.material && !data.materials[obj.material.name]) data.materials[obj.material.name] = obj.material
    })
  }
  return data
}

export interface Disposable {
  type?: string
  clear?: () => void
}

// Disposes an object and all its properties
export function dispose<T extends Disposable>(obj: T): void {
  console.debug(`Dispose: `, obj);
  if (obj.type !== 'Scene') obj.clear?.()
  for (const p in obj) {
    const prop = obj[p] as Disposable | undefined
    if (prop?.type !== 'Scene') prop?.clear?.()
  }
}

export const INTERNAL_PROPS = ['children', 'ref']

// Gets only instance props from proxy-component
export function getInstanceProps<T = any>(queue: any): Instance<T>['props'] {
  // SOLID-THREE-NOTE:  solid-three has to use splitProps so getters are not resolved
  const [_, props] = splitProps(queue, INTERNAL_PROPS)
  return props
}

// Each object in the scene carries a small LocalState descriptor
export function prepare<T = any>(target: T, root: RootState, type: string, props: Instance<T>['props']): Instance<T> {
  const object = target as unknown as Instance['object']

  // Create instance descriptor
  let instance = object?.__r3f
  if (!instance) {
    instance = {
      root,
      type,
      parent: null,
      children: [],
      props: getInstanceProps(props),
      object,
      eventCount: 0,
      handlers: {},
      isHidden: false,
    }
    if (object) {
      object.__r3f = instance
      if (type) applyProps(object, instance.props)
    }
  }

  return instance
}

export function resolve(root: any, key: string): { root: any; key: string; target: any } {
  let target = root[key]
  if (!key.includes('-')) return { root, key, target }

  // Resolve pierced target
  const chain = key.split('-')
  target = chain.reduce((acc, key) => acc[key], root)
  key = chain.pop()!

  // Switch root if atomic
  if (!target?.set) root = chain.reduce((acc, key) => acc[key], root)

  return { root, key, target }
}

// Checks if a dash-cased string ends with an integer
const INDEX_REGEX = /-\d+$/

export function attach(parent: Instance, child: Instance): void {
  if (is.str(child.props.attach)) {
    // If attaching into an array (foo-0), create one
    if (INDEX_REGEX.test(child.props.attach)) {
      const index = child.props.attach.replace(INDEX_REGEX, '')
      const { root, key } = resolve(parent.object, index)
      if (!Array.isArray(root[key])) root[key] = []
    }

    const { root, key } = resolve(parent.object, child.props.attach)
    child.previousAttach = root[key]
    root[key] = child.object
  } else if (is.fun(child.props.attach)) {
    child.previousAttach = child.props.attach(parent.object, child.object)
  }
}

export function detach(parent: Instance, child: Instance): void {
  if (is.str(child.props.attach)) {
    const { root, key } = resolve(parent.object, child.props.attach)
    const previous = child.previousAttach
    // When the previous value was undefined, it means the value was never set to begin with
    if (previous === undefined) delete root[key]
    // Otherwise set the previous value
    else root[key] = previous
  } else {
    child.previousAttach?.(parent.object, child.object)
  }

  delete child.previousAttach
}

export const RESERVED_PROPS = [
  ...INTERNAL_PROPS,
  // Instance props
  'args',
  'dispose',
  'attach',
  'object',
  // Behavior flags
  'dispose',
]

const INTERNAL_EVENT_PROP_KEYS = {
  'click': true,
  'doublclick': true,
  'contextmenu': true,
  // 'copy': true,
  // 'cut': true,
  'pointercancel': true,
  'pointerdown': true,
  'pointerenter': true,
  'pointerleave': true,
  'pointermove': true,
  'pointerout': true,
  'pointerover': true,
  'pointerup': true,

  'wheel': true,
}
type InternalEventPropKeys = keyof typeof INTERNAL_EVENT_PROP_KEYS;

export const DEFAULTS = new Map()

export const applyProp = (object: Instance<Container>['object'], prop: string, value: any) => {
  const rootState = (object as Instance<Container>['object']).__r3f?.root
  /* If the key contains a hyphen, we're setting a sub property. */
  if (prop.indexOf('-') > -1) {
    const [property, ...rest] = prop.split('-')
    applyProps(object[property], { [rest.join('-')]: value })
    return
  }
  // if (needsUpdate && ((!object[prop] && value) || (object[prop] && !value))) object.needsUpdate = true
  /* If prop is an event-handler */
  if (rootState && INTERNAL_EVENT_PROP_KEYS[prop] && object.__r3f) {
    object.__r3f.handlers[prop as InternalEventPropKeys] = value
    object.__r3f.eventCount = Object.keys(object.__r3f.handlers).length
    const nativeEventName = prop.slice(2).toLowerCase() as InternalEventPropKeys;
    // @ts-expect-error ; TODO Fix typing of object.on
    object.on(nativeEventName, value);

    // Deal with pointer events, including removing them if undefined
    if (rootState.internal) {
      const index = rootState.internal.interaction.indexOf(object as unknown as Container)
      if (object.__r3f.eventCount && index === -1) {
        rootState.set('internal', 'interaction', (arr) => [...arr, object as unknown as Container])
      } else if (!object.__r3f.eventCount && index !== -1) {
        rootState.set(
          'internal',
          'interaction',
          produce((arr) => arr.splice(index, 1)),
        )
      }
    }
  }

  // Ignore setting undefined props
  if (value === undefined) return

  let { root, key, target } = resolve(object, prop)

  // Copy if properties match signatures
  if (target?.copy && target?.constructor === (value as ConstructorRepresentation)?.constructor) {
    target.copy(value)
  }
  // Layers have no copy function, we must therefore copy the mask property
  // else if (target instanceof THREE.Layers && value instanceof THREE.Layers) {
  //   target.mask = value.mask
  // }
  // Set array types
  else if (target?.set && Array.isArray(value)) {
    if (target.fromArray) target.fromArray(value)
    else target.set(...value)
  }
  // Set literal types, ignore undefined
  // https://github.com/pmndrs/react-three-fiber/issues/274
  else if (target?.set && typeof value !== 'object') {
    const isColor = target instanceof core.Color
    // Allow setting array scalars
    if (!isColor && target.setScalar && typeof value === 'number') target.setScalar(value)
    // Otherwise just set ...
    else if (value !== undefined) target.set(value)
  }
  // Else, just overwrite the value
  else {
    root[key] = value
  }
}

// This function prepares a set of changes to be applied to the instance
export const applyProps = (object: Instance['object'], props: { [key: string]: any }) =>
  createRenderEffect(
    mapArray(
      () => Object.keys(props),
      (key) => {
        /* We wrap it in an effect only if a prop is a getter or a function */
        const descriptors = Object.getOwnPropertyDescriptor(props, key)
        const isDynamic = !!(descriptors?.get || typeof descriptors?.value === 'function')
        const update = (value: any) => applyProp(object, key, value)
        isDynamic ? createRenderEffect(on(() => props[key], update)) : update(props[key])
      },
    ),
  )

export function invalidateInstance(instance: Instance): void {
  const state = instance.root
  if (state && state.internal.frames === 0) state.invalidate()
}

/**
 * Get a handle to the current global scope in window and worker contexts if able
 * https://github.com/pmndrs/react-three-fiber/pull/2493
 */
export const globalScope =
  (typeof global !== 'undefined' && global) ||
  (typeof self !== 'undefined' && self) ||
  (typeof window !== 'undefined' && window)
