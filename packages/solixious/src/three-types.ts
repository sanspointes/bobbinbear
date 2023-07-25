import { JSX } from 'solid-js/jsx-runtime'
import * as core from '@pixi/core'
import { AttachType, InstanceProps } from './core'
import { EventHandlers } from './core/events'
import { Args, ConstructorRepresentation } from './core/proxy'
import { RootState } from './core/store'
import { Mutable } from './utils/utility-types'
import { ColorSource } from '@pixi/core'

export type Root<TStore = RootState, T = {}> = T & { store: TStore }

export type NonFunctionKeys<T> = { [K in keyof T]: T[K] extends Function ? never : K }[keyof T]
export type Overwrite<T, O> = Omit<T, NonFunctionKeys<O>> & O

export interface MathRepresentation {
  set(...args: number[]): any
}

export type Matrix = core.Matrix | Parameters<core.Matrix['set']> | Readonly<core.Matrix['set']>

/**
 * Turn an implementation of THREE.Vector in to the type that an r3f component would accept as a prop.
 */
export type MathType<T extends MathRepresentation | core.Color> = T extends core.Color
  ? ConstructorParameters<typeof core.Color> | core.Color | core.ColorSource
  : T extends MathRepresentation
  ? T | Parameters<T['set']> | number
  : T;

export type Point = ConstructorParameters<typeof core.Point> | core.Point;
export type Color = ConstructorParameters<typeof core.Color> | core.Color | ColorSource;

type WithMathProps<P> = { [K in keyof P]: P[K] extends MathRepresentation ? MathType<P[K]> : P[K] }

type EventProps = Partial<EventHandlers>

export interface NodeProps<T> {
  attach?: AttachType
  /** Constructor arguments */
  args?: Args<T>
  children?: JSX.Element
  ref?: ((value: T) => void) | T
  key?: string
  onUpdate?: (self: T) => void
}

export type ElementProps<T extends ConstructorRepresentation, P = InstanceType<T>> = Partial<
  Overwrite<WithMathProps<P>, NodeProps<P> & EventProps>
>

export type PixiElement<T extends ConstructorRepresentation> = Mutable<
  Overwrite<ElementProps<T>, Omit<InstanceProps<InstanceType<T>, T>, 'object'>>
>

export type ExtendedColors<T> = { [K in keyof T]: T[K] extends THREE.Color | undefined ? Color : T[K] }
export type Node<T> = ExtendedColors<Overwrite<Partial<T>, NodeProps<T>>>

export type ContainerNode<T> = Overwrite<
  Node<T>,
  {
    position?: Point
    dispose?: (() => void) | null
  }
> &
  EventHandlers
