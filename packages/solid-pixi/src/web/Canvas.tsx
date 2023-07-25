import { extend, createPixiRoot, RenderProps } from "../core";
// import { createPointerEvents } from "./events";
import { RootState, ThreeContext } from "../core/store";
import { Accessor, onCleanup, JSX, mergeProps } from "solid-js";
import { insert } from "../renderer";
import { Instance } from "../core/renderer";
import { StoreApi } from "zustand/vanilla";
import { EventManager } from "../core/events";
import { log } from "../solid";
import { pixiReconciler } from "..";
import { Attribute, Container, Mesh, MeshGeometry, MeshMaterial, PlaneGeometry, Sprite } from "pixi.js";

extend({
  Container: Container,
  Mesh: Mesh,
  Sprite: Sprite,
  MeshGeometry: MeshGeometry,
  MeshMaterial: MeshMaterial,
  PlaneGeometry: PlaneGeometry,
  Attribute: Attribute,
});

export interface Props extends Omit<RenderProps<HTMLCanvasElement>, "size" | "events"> {
  // ,
  //   HTMLAttributes<HTMLDivElement>
  children?: JSX.Element;
  fallback?: JSX.Element;
  // resize?: ResizeOptions
  events?: (store: StoreApi<RootState>) => EventManager<any>;
  id?: string;
  class?: string;
  height?: string;
  width?: string;
  tabIndex?: number;
  // style?: CSSProperties;
}

// type SetBlock = false | Promise<null> | null;

// const CANVAS_PROPS: Array<keyof Props> = [
//   "gl",
//   "events",
//   "shadows",
//   "linear",
//   "flat",
//   "orthographic",
//   "frameloop",
//   "dpr",
//   "performance",
//   "clock",
//   "raycaster",
//   "camera",
//   "onPointerMissed",
//   "onCreated",
// ];

export function Canvas(props: Props) {
  props = mergeProps(
    {
      height: "100vh",
      width: "100vw"
    },
    props
  );

  let canvas: HTMLCanvasElement = (<canvas style={{ height: "100%", width: "100%" }} />) as any;
  let containerRef: HTMLDivElement = (
    <div
      id={props.id}
      class={props.class}
      style={{
        height: props.height,
        width: props.width,
        position: "relative",
        overflow: "hidden"
      }}
      tabIndex={props.tabIndex}
    >
      {canvas}
    </div>
  ) as any;

  const root = createPixiRoot(canvas, {
    // events: createPointerEvents,
    size: containerRef.getBoundingClientRect(),
    onPointerMissed: props.onPointerMissed
    // TODO: add the rest of the canvas props!
  });

  new ResizeObserver(entries => {
    if (entries[0]?.target !== containerRef) return;
    root.getState().setSize(entries[0].contentRect.width, entries[0].contentRect.height);
  }).observe(containerRef);

  insert(
    root.getState().app.stage as unknown as Instance,
    (
      (
        <ThreeContext.Provider value={root}>{props.children}</ThreeContext.Provider>
      ) as unknown as Accessor<Instance[]>
    )()
  );

  onCleanup(() => {
    log("three", "cleanup");
    pixiReconciler.removeRecursive(
      root.getState().app.stage.children as any,
      root.getState().app.stage as any,
      true
    );
    // root.getState().app.destroy();
  });

  return containerRef;
}
