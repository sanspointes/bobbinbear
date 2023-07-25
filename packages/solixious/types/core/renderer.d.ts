import type { JSX } from "solid-js";
import * as layers from "@pixi/layers";
import type { ComputeFunction, EventManager } from "./events";
import type { Dpr, Frameloop, Performance, Renderer, RootState, Size } from "./store";
import { Application, IApplicationOptions } from "@pixi/app";
type Canvas = HTMLCanvasElement | OffscreenCanvas;
export declare const _roots: Map<Canvas, {
    store: RootState;
}>;
type Properties<T> = Pick<T, {
    [K in keyof T]: T[K] extends (_: any) => any ? never : K;
}[keyof T]>;
export type GLProps = Application | ((canvas: Canvas) => Renderer) | Partial<Properties<Application> | IApplicationOptions>;
export interface RenderProps {
    /** A threejs renderer instance or props that go into the default renderer */
    gl?: GLProps;
    /** Dimensions to fit the renderer to. Will measure canvas dimensions if omitted */
    size?: Size;
    /**
     * R3F's render mode. Set to `demand` to only render on state change or `never` to take control.
     * @see https://docs.pmnd.rs/react-three-fiber/advanced/scaling-performance#on-demand-rendering
     */
    frameloop?: Frameloop;
    /**
     * R3F performance options for adaptive performance.
     * @see https://docs.pmnd.rs/react-three-fiber/advanced/scaling-performance#movement-regression
     */
    performance?: Partial<Omit<Performance, "regress">>;
    /** Target pixel ratio. Can clamp between a range: `[min, max]` */
    dpr?: Dpr;
    /** Props that go into the default raycaster */
    raycaster?: Partial<THREE.Raycaster>;
    /** A `THREE.Scene` instance or props that go into the default scene */
    scene?: THREE.Scene | Partial<THREE.Scene>;
    /** An R3F event manager to manage elements' pointer events */
    events?: (store: RootState) => EventManager<HTMLElement>;
    /** Callback after the canvas has rendered (but not yet committed) */
    onCreated?: (state: RootState) => void;
    /** Response for pointer clicks that have missed any target */
    onPointerMissed?: (event: MouseEvent) => void;
    render?: "auto" | "manual";
}
export interface ReconcilerRoot<TCanvas extends Canvas> {
    configure: (config?: RenderProps) => ReconcilerRoot<TCanvas>;
    render: (props: {
        children: JSX.Element;
    }) => RootState;
    unmount: () => void;
}
export declare function createRoot<TCanvas extends Canvas>(canvas: TCanvas): ReconcilerRoot<TCanvas>;
export declare function render<TCanvas extends Canvas>(children: JSX.Element, canvas: TCanvas, config: RenderProps): RootState;
export declare function unmountComponentAtNode<TCanvas extends Canvas>(canvas: TCanvas, callback?: (canvas: TCanvas) => void): void;
export type InjectState = Partial<Omit<RootState, "events"> & {
    events?: {
        enabled?: boolean;
        priority?: number;
        compute?: ComputeFunction;
        connected?: any;
    };
}>;
export declare function createPortal(children: JSX.Element, container: layers.Stage, state?: InjectState): JSX.Element;
interface PortalProps {
    children: JSX.Element;
    state?: InjectState;
    container: layers.Stage;
}
export declare function Portal(props: PortalProps): JSX.Element;
export {};
//# sourceMappingURL=renderer.d.ts.map