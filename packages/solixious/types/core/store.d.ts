/// <reference types="webxr" />
import { SetStoreFunction } from 'solid-js/store';
import * as app from '@pixi/app';
import * as core from '@pixi/core';
import * as ticker from '@pixi/ticker';
import * as display from '@pixi/display';
import * as layers from '@pixi/layers';
import { FixedStage, Stage } from './stages';
import type { DomEvent, EventManager, PointerCaptureTarget, ThreeEvent } from './events';
export declare const privateKeys: readonly ["set", "get", "setSize", "setFrameloop", "setDpr", "events", "invalidate", "advance", "size", "viewport"];
export type PrivateKeys = (typeof privateKeys)[number];
export type Subscription = {
    ref: RenderCallback;
    priority: number;
    store: RootState;
};
export type Dpr = number | [min: number, max: number];
export interface Size {
    width: number;
    height: number;
    top: number;
    left: number;
}
export interface Viewport extends Size {
    /** The initial pixel ratio */
    initialDpr: number;
    /** Current pixel ratio */
    dpr: number;
    /** size.width / viewport.width */
    factor: number;
}
export type RenderCallback = (state: RootState, delta: number, frame?: XRFrame) => void;
export type UpdateCallback = RenderCallback;
export type LegacyAlways = 'always';
export type FrameloopMode = LegacyAlways | 'auto' | 'demand' | 'never';
export type FrameloopRender = 'auto' | 'manual';
export type FrameloopLegacy = 'always' | 'demand' | 'never';
export type Frameloop = FrameloopLegacy | {
    mode?: FrameloopMode;
    render?: FrameloopRender;
    maxDelta?: number;
};
export interface Performance {
    /** Current performance normal, between min and max */
    current: number;
    /** How low the performance can go, between 0 and max */
    min: number;
    /** How high the performance can go, between min and max */
    max: number;
    /** Time until current returns to max in ms */
    debounce: number;
    /** Sets current to min, puts the system in regression */
    regress: () => void;
}
export interface Renderer {
    render: (scene: THREE.Scene, camera: THREE.Camera) => any;
}
export declare const isRenderer: (def: any) => boolean;
export type StageTypes = Stage | FixedStage;
export interface InternalState {
    interaction: display.Container[];
    hovered: Map<string, ThreeEvent<DomEvent>>;
    subscribers: Subscription[];
    capturedMap: Map<number, Map<display.Container, PointerCaptureTarget>>;
    initialClick: [x: number, y: number];
    initialHits: display.Container[];
    lastEvent: DomEvent | null;
    active: boolean;
    priority: number;
    frames: number;
    /** The ordered stages defining the lifecycle. */
    stages: StageTypes[];
    /** Render function flags */
    render: 'auto' | 'manual';
    /** The max delta time between two frames. */
    maxDelta: number;
    subscribe: (callback: RenderCallback, priority: number, store: RootState) => () => void;
}
export interface XRManager {
    connect: () => void;
    disconnect: () => void;
}
export interface RootState {
    /** Set current state */
    set: SetStoreFunction<RootState>;
    /** The instance of the renderer */
    gl: app.Application;
    /** Default camera */
    /** Default scene */
    scene: layers.Stage;
    /** Default raycaster */
    /** Default clock */
    ticker: ticker.Ticker;
    /** Event layer interface, contains the event handler and the node they're connected to */
    events: EventManager<any>;
    /** XR interface */
    xr: XRManager;
    /** Currently used controls */
    controls: THREE.EventDispatcher | null;
    /** Normalized event coordinates */
    pointer: core.Point;
    /** @deprecated Normalized event coordinates, use "pointer" instead! */
    mouse: core.Point;
    /** Update frame loop flags */
    frameloop: FrameloopLegacy;
    /** Adaptive performance interface */
    performance: Performance;
    /** Reactive pixel-size of the canvas */
    size: Size;
    /** Reactive size of the viewport in threejs units */
    viewport: Viewport & {
        getCurrentViewport: (size?: Size) => Omit<Viewport, 'dpr' | 'initialDpr'>;
    };
    /** Flags the canvas for render, but doesn't render in itself */
    invalidate: (frames?: number) => void;
    /** Advance (render) one step */
    advance: (timestamp: number, runGlobalEffects?: boolean) => void;
    /** Shortcut to setting the event layer */
    setEvents: (events: Partial<EventManager<any>>) => void;
    /** Shortcut to manual sizing */
    setSize: (width: number, height: number, top?: number, left?: number) => void;
    /** Shortcut to manual setting the pixel ratio */
    setDpr: (dpr: Dpr) => void;
    /** Shortcut to setting frameloop flags */
    setFrameloop: (frameloop: Frameloop) => void;
    /** When the canvas was clicked but nothing was hit */
    onPointerMissed?: (event: MouseEvent) => void;
    /** If this state model is layerd (via createPortal) then this contains the previous layer */
    previousRoot?: RootState;
    /** Internals */
    internal: InternalState;
}
export declare const context: import("solid-js").Context<RootState | undefined>;
declare const createThreeStore: (invalidate: (state?: RootState, frames?: number) => void, advance: (timestamp: number, runGlobalEffects?: boolean, state?: RootState, frame?: XRFrame) => void) => RootState;
export { createThreeStore };
//# sourceMappingURL=store.d.ts.map