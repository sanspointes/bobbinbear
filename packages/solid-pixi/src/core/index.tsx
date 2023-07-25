import { StoreApi as UseStore } from "zustand/vanilla";
import { applyProps, dispose } from "./utils";
import {
  createThreeStore as createPixiStore,
  RootState,
  Size,
  StoreProps,
  ThreeContext,
} from "./store";
import { extend, Root } from "./renderer";
import { addAfterEffect, addEffect, addTail, createLoop } from "./loop";
import { EventManager } from "./events";
import { createEffect } from "solid-js";
import { Application, IApplicationOptions, ICanvas } from "pixi.js";

export type {
  Camera,
  EventHandlers,
  Intersection,
  IntersectionEvent,
} from "./events";
export { applyProp, attach } from "./utils";
export { catalogue } from "./renderer";
export type { InternalState } from "./store";
export type { DiffSet } from "./utils";

export const roots = new Map<Element, Root>();
const { invalidate, advance } = createLoop(roots);

type Properties<T> = Pick<
  T,
  { [K in keyof T]: T[K] extends (_: any) => any ? never : K }[keyof T]
>;

type GLProps =
  | ((canvas: HTMLCanvasElement) => Renderer)
  | Partial<Properties<Application> | IApplicationOptions>
  | undefined;

export type RenderProps<TCanvas extends Element> =
  & Omit<StoreProps, "gl" | "app" | "events" | "size">
  & {
    gl?: GLProps;
    events?: (store: UseStore<RootState>) => EventManager<TCanvas>;
    size?: Size;
    onCreated?: (state: RootState) => void;
  };

const createRendererInstance = <TElement extends Element>(
  gl: GLProps,
  canvas: TElement,
): Application<ICanvas> => {
  const app = new Application({
    powerPreference: "high-performance",
    view: canvas as unknown as HTMLCanvasElement,
    antialias: true,
    ...gl,
  });
  globalThis.__PIXI_APP__ = app;
  // const renderer = new THREE.WebGLRenderer({
  //   powerPreference: "high-performance",
  //   canvas: canvas as unknown as HTMLCanvasElement,
  //   antialias: true,
  //   alpha: true,
  //   ...gl
  // });

  // Set color management
  // Set gl props
  if (gl) applyProps(app as any, gl as any);

  return app;
};

function createPixiRoot<TCanvas extends HTMLElement>(
  canvas: TCanvas,
  config?: RenderProps<TCanvas>,
) {
  let { gl, size, events, onCreated, ...props } = config || {};
  // Allow size to take on container bounds initially
  if (!size) {
    size = canvas.parentElement?.getBoundingClientRect() ?? {
      width: 0,
      height: 0,
    };
  }

  // if (fiber && state) {
  //   // When a root was found, see if any fundamental props must be changed or exchanged

  //   // Check pixelratio
  //   if (
  //     props.dpr !== undefined &&
  //     state.viewport.dpr !== calculateDpr(props.dpr)
  //   )
  //     state.setDpr(props.dpr);
  //   // Check size
  //   if (
  //     state.size.width !== size.width ||
  //     state.size.height !== size.height
  //   )
  //     state.setSize(size.width, size.height);
  //   // Check frameloop
  //   if (state.frameloop !== props.frameloop)
  //     state.setFrameloop(props.frameloop);

  //   // For some props we want to reset the entire root

  //   // Changes to the color-space
  //   const linearChanged = props.linear !== state.internal.lastProps.linear;
  //   if (linearChanged) {
  //     unmountComponentAtNode(canvas);
  //     fiber = undefined;
  //   }
  // }

  // Create gl
  const appInstance = createRendererInstance(gl, canvas);

  // Create store
  const store = createPixiStore(applyProps, invalidate, advance, {
    app: appInstance,
    size,
    ...props,
  });

  const state = store.getState();

  // Map it
  roots.set(canvas, { store });
  // Store events internally
  if (events) state.set({ events: events(store) });

  createEffect(() => {
    const state = store.getState();
    // Flag the canvas active, rendering will now begin
    state.set((state) => ({ internal: { ...state.internal, active: true } }));
    // Connect events
    state.events.connect?.(canvas);
    // Notifiy that init is completed, the scene graph exists, but nothing has yet rendered
    onCreated?.(state);

    state.invalidate();
  });

  return store;
}

function unmountComponentAtNode<TElement extends Element>(
  canvas: TElement,
  callback?: (canvas: TElement) => void,
) {
  const root = roots.get(canvas);
  // const fiber = root?.fiber;
  // if (fiber) {
  //   const state = root?.store.getState();
  //   if (state) state.internal.active = false;

  //   setTimeout(() => {
  //     try {
  //       state.events.disconnect?.();
  //       state.gl?.renderLists?.dispose?.();
  //       state.gl?.forceContextLoss?.();
  //       if (state.gl?.xr) state.internal.xr.disconnect();
  //       dispose(state);
  //       roots.delete(canvas);
  //       if (callback) callback(canvas);
  //     } catch (e) {
  //       /* ... */
  //     }
  //   }, 500);
  // }
}

// function createPortal(
//   children: React.ReactNode,
//   container: THREE.Object3D
// ): React.ReactNode {
//   return reconciler.createPortal(children, container, null, null);
// }

export {
  addAfterEffect,
  addEffect,
  addTail,
  advance,
  // createPortal,
  applyProps,
  // render,
  createPixiRoot,
  dispose,
  extend,
  invalidate,
  // act,
  roots as _roots,
  ThreeContext as context,
  unmountComponentAtNode,
};
