import { createResizeObserver } from "@solid-primitives/resize-observer";
import { createEffect, createMemo, splitProps } from "solid-js";
import type { JSX } from "solid-js";
import { createStore } from "solid-js/store";
import * as core from "@pixi/core";
import * as app from "@pixi/app";
import * as layers from "@pixi/layers";

import { useThree } from "./hooks";
import { advance, invalidate } from "./loop";
import { parentChildren } from "./proxy";
import { Lifecycle, Stage, Stages } from "./stages";
import { context, createThreeStore } from "./store";
import { applyProps, calculateDpr, dispose, is, prepare } from "./utils";

import { insert } from "solid-js/web";
import type { Root } from "../three-types";
import { withContext } from "../utils/withContext";
import type { ComputeFunction, EventManager } from "./events";
import type {
  Dpr,
  Frameloop,
  Performance,
  RootState,
  Size,
  Subscription,
} from "./store";
import type { EquConfig } from "./utils";
import { Application, IApplicationOptions } from "@pixi/app";

// TODO: fix type resolve
type Canvas = HTMLCanvasElement | OffscreenCanvas;

export const _roots = new Map<Canvas, Root>();

const shallowLoose = { objects: "shallow", strict: false } as EquConfig;

type Properties<T> = Pick<
  T,
  { [K in keyof T]: T[K] extends (_: any) => any ? never : K }[keyof T]
>;

export type GLProps =
  | Application
  | Partial<Properties<Application> | IApplicationOptions>;

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
  /** An R3F event manager to manage elements' pointer events */
  events?: (store: RootState) => EventManager<HTMLElement>;
  /** Callback after the canvas has rendered (but not yet committed) */
  onCreated?: (state: RootState) => void;
  /** Response for pointer clicks that have missed any target */
  onPointerMissed?: (event: MouseEvent) => void;

  render?: "auto" | "manual";
}

const createRendererInstance = <TCanvas extends Canvas>(
  gl: GLProps | undefined,
  canvas: TCanvas,
): app.Application => {
  return new app.Application({
    powerPreference: "high-performance",
    view: canvas,
    antialias: true,
    ...gl,
  });
};

// const createStages = (stages: Stage[] | undefined, store: RootState) => {
//   let subscribers: Subscription[];
//   let subscription: Subscription;
//
//   const _stages = stages ?? Lifecycle;
//
//   if (!_stages.includes(Stages.Update)) {
//     throw "The Stages.Update stage is required for R3F.";
//   }
//   if (!_stages.includes(Stages.Render)) {
//     throw "The Stages.Render stage is required for R3F.";
//   }
//
//   store.set("internal", "stages", _stages);
//
//   // Add useFrame loop to update stage
//   const frameCallback = (
//     state: RootState,
//     delta: number,
//     frame?: XRFrame | undefined,
//   ) => {
//     subscribers = state.internal.subscribers;
//     for (let i = 0; i < subscribers.length; i++) {
//       subscription = subscribers[i];
//       subscription.ref(subscription.store, delta, frame);
//     }
//   };
//   Stages.Update.add(frameCallback, store);
//
//   // Add render callback to render stage
//   const renderCallback = (state: RootState) => {
//     if (state.internal.render === "auto" && state.gl.render) {
//       state.gl.render();
//     }
//   };
//   Stages.Render.add(renderCallback, store);
// };

export interface ReconcilerRoot<TCanvas extends Canvas> {
  configure: (config?: RenderProps) => ReconcilerRoot<TCanvas>;
  // s3f    solid-three has to pass the element to render as { children: JSX.Element }
  //        otherwise we would have to do .render(props.children) inside Canvas
  //        which would cause the children to be resolved too early.
  render: (props: { children: JSX.Element }) => RootState;
  unmount: () => void;
}

function computeInitialSize(canvas: Canvas, size?: Size): Size {
  if (!size && canvas instanceof HTMLCanvasElement && canvas.parentElement) {
    const { width, height, top, left } = canvas.parentElement
      .getBoundingClientRect();
    return { width, height, top, left };
  } else if (
    !size && typeof OffscreenCanvas !== "undefined" &&
    canvas instanceof OffscreenCanvas
  ) {
    return {
      width: canvas.width,
      height: canvas.height,
      top: 0,
      left: 0,
    };
  }

  return { width: 0, height: 0, top: 0, left: 0, ...size };
}

export function createRoot<TCanvas extends Canvas>(
  canvas: TCanvas,
): ReconcilerRoot<TCanvas> {
  // Check against mistaken use of createRoot
  const prevRoot = _roots.get(canvas);
  const prevStore = prevRoot?.store;

  if (prevRoot) console.warn("R3F.createRoot should only be called once!");

  // Create store
  const store = prevStore || createThreeStore(invalidate, advance);
  // Map it
  if (!prevRoot) _roots.set(canvas, { store });

  // Locals
  let onCreated: ((state: RootState) => void) | undefined;
  let configured = false;

  return {
    configure(props: RenderProps = {}): ReconcilerRoot<TCanvas> {
      let {
        gl: glConfig,
        size: propsSize,
        events,
        onCreated: onCreatedCallback,
        frameloop = "always",
        dpr = [1, 2],
        performance,
        onPointerMissed,
      } = props;

      // Set up renderer (one time only!)
      let gl = store.gl;
      if (!store.gl) {
        store.set("gl", gl = createRendererInstance(glConfig, canvas));
      }

      store.set("scene", gl.stage);

      // Set gl props
      if (
        glConfig && !is.fun(glConfig) &&
        !is.equ(glConfig, gl, shallowLoose)
      ) {
        applyProps(gl, glConfig as any);
      }
      // Store events internally
      if (events && !store.events.handlers) store.set("events", events(store));

      // Check size, allow it to take on container bounds initially
      const size = computeInitialSize(canvas, propsSize);
      if (!is.equ(size, store.size, shallowLoose)) {
        store.setSize(size.width, size.height, size.top, size.left);
      }
      // Check pixelratio
      if (dpr && store.viewport.dpr !== calculateDpr(dpr)) store.setDpr(dpr);
      // Check frameloop
      if (store.frameloop !== frameloop) store.setFrameloop(frameloop);
      // Check pointer missed
      if (!store.onPointerMissed) store.set("onPointerMissed", onPointerMissed);
      // Check performance
      if (
        performance && !is.equ(performance, store.performance, shallowLoose)
      ) store.set("performance", performance);

      // Set locals
      onCreated = onCreatedCallback;
      configured = true;

      return this;
    },
    render(props) {
      // The root has to be configured before it can be rendered
      if (!configured) this.configure();

      // s3f:  this code will break when used in a worker.
      if (!(canvas instanceof OffscreenCanvas)) {
        createResizeObserver(
          () => canvas.parentElement!,
          ({ width, height }) => {
            store.setSize(width, height);
          },
        );
      }

      // s3f    children of <Canvas/> are being attached to the Instance<typeof store.scene>
      const memo = createMemo(
        withContext(() => props.children, context, store),
      );
      parentChildren(() => store.scene, {
        get children() {
          return memo();
        },
      });

      // s3f:  this code will break when used in a worker.
      if (!(canvas instanceof OffscreenCanvas)) {
        insert(
          canvas.parentElement!,
          () => (
            <Provider store={store} rootElement={canvas} onCreated={onCreated}>
              {[canvas]}
            </Provider>
          ),
        );
      }

      return store;
    },
    unmount() {
      unmountComponentAtNode(canvas);
    },
  };
}

export function render<TCanvas extends Canvas>(
  children: JSX.Element,
  canvas: TCanvas,
  config: RenderProps,
): RootState {
  console.warn(
    "R3F.render is no longer supported in React 18. Use createRoot instead!",
  );
  const root = createRoot(canvas);
  root.configure(config);
  return root.render({ children });
}

interface ProviderProps<TCanvas extends Canvas> {
  onCreated?: (state: RootState) => void;
  store: RootState;
  children: JSX.Element;
  rootElement: TCanvas;
}

function Provider<TCanvas extends Canvas>(
  props: ProviderProps<TCanvas>,
): JSX.Element {
  // Flag the canvas active, rendering will now begin
  props.store.set("internal", "active", true);
  // Notifiy that init is completed, the scene graph exists, but nothing has yet rendered
  if (props.onCreated) props.onCreated(props.store);
  // Connect events to the targets parent, this is done to ensure events are registered on
  // a shared target, and not on the canvas itself
  if (!props.store.events.connected) {
    props.store.events.connect?.(props.rootElement);
  }
  return (
    <context.Provider value={props.store}>{props.children}</context.Provider>
  );
}

export function unmountComponentAtNode<TCanvas extends Canvas>(
  canvas: TCanvas,
  callback?: (canvas: TCanvas) => void,
): void {
  const root = _roots.get(canvas);
  const state = root?.store;
  if (state) {
    state.internal.active = false;
    setTimeout(() => {
      try {
        state.events.disconnect?.();
        state.gl.destroy();
        // state.gl?.renderLists?.dispose?.()
        // state.gl?.forceContextLoss?.()
        // if (state.gl?.xr) state.xr.disconnect()
        dispose(state.scene);
        _roots.delete(canvas);
        if (callback) callback(canvas);
      } catch (e) {
        /* ... */
      }
    }, 500);
  }
}

export type InjectState = Partial<
  Omit<RootState, "events"> & {
    events?: {
      enabled?: boolean;
      priority?: number;
      compute?: ComputeFunction;
      connected?: any;
    };
  }
>;

export function createPortal(
  children: JSX.Element,
  container: layers.Stage,
  state?: InjectState,
): JSX.Element {
  return <Portal children={children} container={container} state={state} />;
}

interface PortalProps {
  children: JSX.Element;
  state?: InjectState;
  container: layers.Stage;
}

export function Portal(props: PortalProps) {
  const [state, rest] = splitProps(props.state || {}, ["events", "size"]);
  const previousRoot = useThree();
  const pointer = new core.Point();

  const store = useThree();
  const scene = prepare(props.container || store.scene, store, "", {});

  const inject = (rootState: RootState, injectState: RootState) => {
    let viewport;
    if (state.size) {
      viewport = rootState.viewport.getCurrentViewport(state.size);
    }

    return {
      // The intersect consists of the previous root state
      ...rootState,
      set: injectState.set,
      // Portals have their own scene
      scene: props.container as layers.Stage,
      pointer,
      mouse: pointer,
      // Their previous root is the layer before it
      previousRoot,
      // Events, size and viewport can be overridden by the inject layer
      events: { ...rootState.events, ...injectState.events, ...state.events },
      size: { ...rootState.size, ...state.size },
      viewport: { ...rootState.viewport, ...viewport },
      // Layers are allowed to override events
      setEvents: (events: Partial<EventManager<any>>) =>
        injectState.set((state) => ({
          ...state,
          events: { ...state.events, ...events },
        })),
    } as RootState;
  };

  // SOLID-THREE-NOTE:  I am unsure if this will work in solid since the original code
  //                    relied on subscribing aka deep-tracking rootState
  const usePortalStore = createMemo(() => {
    //@ts-ignore
    const set = (...args) => setStore(...args);
    const [store, setStore] = createStore<RootState>(
      { ...rest, set } as RootState,
    );
    const onMutate = (prev: RootState) =>
      store.set((state) => inject(prev, state));
    createEffect(() => onMutate(previousRoot));
    return store;
  });

  const memo = createMemo(
    withContext(() => props.children, context, usePortalStore()),
  );

  parentChildren(() => scene.object, {
    get children() {
      return memo();
    },
  });

  return <></>;
}
