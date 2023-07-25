import { Accessor, batch, createContext, createEffect, on } from 'solid-js'
import { SetStoreFunction, createStore, produce } from 'solid-js/store'
import * as app from '@pixi/app';
import * as core from '@pixi/core';
import * as ticker from '@pixi/ticker';
import * as display from '@pixi/display';
import * as layers from '@pixi/layers';

import { FixedStage, Stage } from './stages'
import { calculateDpr } from './utils'

import type { DomEvent, EventManager, PointerCaptureTarget, ThreeEvent } from './events'
import { Instance } from './proxy'

// Keys that shouldn't be copied between R3F stores
export const privateKeys = [
  'set',
  'get',
  'setSize',
  'setFrameloop',
  'setDpr',
  'events',
  'invalidate',
  'advance',
  'size',
  'viewport',
] as const

export type PrivateKeys = (typeof privateKeys)[number]

export type Subscription = {
  ref: RenderCallback
  priority: number
  store: RootState
}

export type Dpr = number | [min: number, max: number]
export interface Size {
  width: number
  height: number
  top: number
  left: number
}
export interface Viewport extends Size {
  /** The initial pixel ratio */
  initialDpr: number
  /** Current pixel ratio */
  dpr: number
  /** size.width / viewport.width */
  factor: number
}

export type RenderCallback = (state: RootState, delta: number, frame?: XRFrame) => void
export type UpdateCallback = RenderCallback

export type LegacyAlways = 'always'
export type FrameloopMode = LegacyAlways | 'auto' | 'demand' | 'never'
export type FrameloopRender = 'auto' | 'manual'
export type FrameloopLegacy = 'always' | 'demand' | 'never'
export type Frameloop = FrameloopLegacy | { mode?: FrameloopMode; render?: FrameloopRender; maxDelta?: number }

export interface Performance {
  /** Current performance normal, between min and max */
  current: number
  /** How low the performance can go, between 0 and max */
  min: number
  /** How high the performance can go, between min and max */
  max: number
  /** Time until current returns to max in ms */
  debounce: number
  /** Sets current to min, puts the system in regression */
  regress: () => void
}

export type StageTypes = Stage | FixedStage

export interface InternalState {
  interaction: display.Container[]
  hovered: Map<string, ThreeEvent<DomEvent>>
  subscribers: Subscription[]
  capturedMap: Map<number, Map<display.Container, PointerCaptureTarget>>
  initialClick: [x: number, y: number]
  initialHits: display.Container[]
  lastEvent: DomEvent | null
  active: boolean
  priority: number
  frames: number
  /** Render function flags */
  render: 'auto' | 'manual'
  /** The max delta time between two frames. */
  maxDelta: number
  subscribe: (callback: RenderCallback, priority: number, store: RootState) => () => void
}

export interface XRManager {
  connect: () => void
  disconnect: () => void
}

export interface RootState {
  /** Set current state */
  set: SetStoreFunction<RootState>
  /** The instance of the renderer */
  gl: app.Application
  /** Default camera */
  // camera: Camera
  /** Default scene */
  scene: display.Container
  /** Default raycaster */
  // raycaster: THREE.Raycaster
  /** Default clock */
  ticker: ticker.Ticker
  /** Event layer interface, contains the event handler and the node they're connected to */
  events: EventManager<any>
  /** XR interface */
  xr: XRManager
  /** Currently used controls */
  controls: THREE.EventDispatcher | null
  /** Normalized event coordinates */
  pointer: core.Point
  /** @deprecated Normalized event coordinates, use "pointer" instead! */
  mouse: core.Point
  /** Update frame loop flags */
  frameloop: FrameloopLegacy
  /** Adaptive performance interface */
  performance: Performance
  /** Reactive pixel-size of the canvas */
  size: Size
  /** Reactive size of the viewport in threejs units */
  viewport: Viewport & {
    getCurrentViewport: (
      size?: Size,
    ) => Omit<Viewport, 'dpr' | 'initialDpr'>
  }
  /** Flags the canvas for render, but doesn't render in itself */
  invalidate: (frames?: number) => void
  /** Advance (render) one step */
  advance: (timestamp: number, runGlobalEffects?: boolean) => void
  /** Shortcut to setting the event layer */
  setEvents: (events: Partial<EventManager<any>>) => void
  /** Shortcut to manual sizing */
  setSize: (width: number, height: number, top?: number, left?: number) => void
  /** Shortcut to manual setting the pixel ratio */
  setDpr: (dpr: Dpr) => void
  /** Shortcut to setting frameloop flags */
  setFrameloop: (frameloop: Frameloop) => void
  /** When the canvas was clicked but nothing was hit */
  onPointerMissed?: (event: MouseEvent) => void
  /** If this state model is layerd (via createPortal) then this contains the previous layer */
  previousRoot?: RootState
  /** Internals */
  internal: InternalState
}

export const context = createContext<RootState>(null!)

const createThreeStore = (
  invalidate: (state?: RootState, frames?: number) => void,
  advance: (timestamp: number, runGlobalEffects?: boolean, state?: RootState, frame?: XRFrame) => void,
): RootState => {
  function getCurrentViewport(
    size: Size = rootState.size,
  ): Omit<Viewport, 'dpr' | 'initialDpr'> {
    const { width, height, top, left } = size
    return { width, height, top, left, factor: 1 }
  }

  let performanceTimeout: ReturnType<typeof setTimeout> | undefined = undefined
  const setPerformanceCurrent = (current: number) => set('performance', 'current', current)

  const pointer = new core.Point()

  //@ts-expect-error
  const set: SetStoreFunction<RootState> = (...args: any[]) => setRootState(...args)
  const get: Accessor<RootState> = () => rootState

  const [rootState, setRootState] = createStore<RootState>({
    set,

    // Mock objects that have to be configured
    gl: null as unknown as app.Application,
    events: { priority: 1, enabled: true, connected: false },
    scene: null as unknown as display.Container,
    xr: null as unknown as XRManager,

    invalidate: (frames = 1) => invalidate(get(), frames),
    advance: (timestamp: number, runGlobalEffects?: boolean) => advance(timestamp, runGlobalEffects, get()),

    controls: null,
    ticker: new ticker.Ticker(),
    pointer,
    mouse: pointer,

    frameloop: 'always',
    onPointerMissed: undefined,

    performance: {
      current: 1,
      min: 0.5,
      max: 1,
      debounce: 200,
      regress: () => {
        const state = get()
        // Clear timeout
        if (performanceTimeout) clearTimeout(performanceTimeout)
        // Set lower bound performance
        if (state.performance.current !== state.performance.min) setPerformanceCurrent(state.performance.min)
        // Go back to upper bound performance after a while unless something regresses meanwhile
        performanceTimeout = setTimeout(() => setPerformanceCurrent(get().performance.max), state.performance.debounce)
      },
    },

    size: { width: 0, height: 0, top: 0, left: 0 },
    viewport: {
      initialDpr: 0,
      dpr: 0,
      width: 0,
      height: 0,
      top: 0,
      left: 0,
      factor: 0,
      getCurrentViewport,
    },

    setEvents: (events: Partial<EventManager<any>>) => set('events', events),
    setSize: (width: number, height: number, top: number = 0, left: number = 0) => {
      const size = { width, height, top: top, left: left }
      batch(() => {
        set('viewport', getCurrentViewport(size))
        set('size', size)
      })
    },
    setDpr: (dpr: Dpr) => {
      const resolved = calculateDpr(dpr)
      const state = get()
      return set('viewport', { dpr: resolved, initialDpr: state.viewport.initialDpr || resolved })
    },
    setFrameloop: (frameloop: Frameloop) => {
      const state = get()
      const mode: FrameloopLegacy =
        typeof frameloop === 'string'
          ? frameloop
          : frameloop?.mode === 'auto'
          ? 'always'
          : frameloop?.mode ?? state.frameloop
      const render = typeof frameloop === 'string' ? state.internal.render : frameloop?.render ?? state.internal.render
      const maxDelta =
        typeof frameloop === 'string' ? state.internal.maxDelta : frameloop?.maxDelta ?? state.internal.maxDelta

      const clock = state.ticker
      // if frameloop === "never" clock.elapsedTime is updated using advance(timestamp)
      clock.stop()
      clock.elapsedMS = 0
      clock.lastTime = 0

      if (frameloop !== 'never') {
        clock.start()
        clock.elapsedMS = 0
        clock.lastTime = 0
      }
      batch(() => {
        set('frameloop', mode)
        set('internal', { render, maxDelta })
      })
    },
    previousRoot: undefined,
    internal: {
      // Events
      interaction: [],
      hovered: new Map<string, ThreeEvent<DomEvent>>(),
      subscribers: [],
      initialClick: [0, 0],
      initialHits: [],
      capturedMap: new Map(),
      lastEvent: null,

      // Updates
      active: false,
      frames: 0,
      stages: [],
      render: 'auto',
      maxDelta: 1 / 10,
      priority: 0,
      subscribe: (ref: RenderCallback, priority: number, store: RootState) => {
        const state = get()
        const internal = state.internal
        // If this subscription was given a priority, it takes rendering into its own hands
        // For that reason we switch off automatic rendering and increase the manual flag
        // As long as this flag is positive there can be no internal rendering at all
        // because there could be multiple render subscriptions
        set('internal', 'priority', internal.priority + (priority > 0 ? 1 : 0))
        // We use the render flag and deprecate priority
        if (internal.priority && state.internal.render === 'auto') set('internal', 'render', 'manual')
        set(
          'internal',
          'subscribers',
          produce((arr) => arr.push({ ref, priority, store })),
        )
        // Register subscriber and sort layers from lowest to highest, meaning,
        // highest priority renders last (on top of the other frames)
        set(
          'internal',
          'subscribers',
          produce((subscribers) => subscribers.sort((a, b) => a.priority - b.priority)),
        )
        return () => {
          const state = get()
          const internal = state.internal
          if (internal?.subscribers) {
            // Decrease manual flag if this subscription had a priority
            set('internal', 'priority', internal.priority - (priority > 0 ? 1 : 0))
            // We use the render flag and deprecate priority
            if (!internal.priority && state.internal.render === 'manual') set('internal', 'render', 'auto')
            // Remove subscriber from list
            set(
              'internal',
              'subscribers',
              internal.subscribers.filter((s) => s.ref !== ref),
            )
          }
        }
      },
    },
  })

  createEffect(() => {
    // Resize camera and renderer on changes to size and pixelratio
    // Update camera & renderer
    rootState.gl.renderer.resolution = rootState.viewport.dpr
    rootState.gl.renderer.resize(rootState.size.width, rootState.size.height)
  })

  createEffect(() => {
    // Update viewport once the camera changes
    rootState.set('viewport', rootState.viewport.getCurrentViewport())
  })

  // TODO:  This currently does not deep-track like the original r3f does.
  // Invalidate on any change
  createEffect(
    on(
      () => rootState,
      () => invalidate(rootState),
    ),
  )

  // Return root state
  return rootState
}

export { createThreeStore }
