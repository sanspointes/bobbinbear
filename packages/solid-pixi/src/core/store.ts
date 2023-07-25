import create, {
  GetState,
  SetState,
  StoreApi as UseStore,
} from "zustand/vanilla";
import { prepare, Instance, InstanceProps } from "./renderer";
import {
  DomEvent,
  EventManager,
  PointerCaptureTarget,
  ThreeEvent,
} from "./events";
import { calculateDpr } from "./utils";
import { createContext } from "solid-js";
import { subscribeWithSelector } from "zustand/middleware";
import { Application, Container, ICanvas, Point, Ticker, TickerCallback } from "pixi.js";

// export interface Intersection extends THREE.Intersection {
//   eventObject: THREE.Object3D;
// }

export type Subscription = {
  ref: RenderCallback;
  priority: number;
};

export type Dpr = number | [min: number, max: number];
export type Size = { width: number; height: number };
export type Viewport = Size & {
  initialDpr: number;
  dpr: number;
  factor: number;
  distance: number;
  aspect: number;
};

// export type Camera = THREE.OrthographicCamera | THREE.PerspectiveCamera;
// export type Raycaster = THREE.Raycaster & {
//   enabled: boolean;
//   filter?: FilterFunction;
//   computeOffsets?: ComputeOffsetsFunction;
// };

export type RenderCallback = TickerCallback<RootState>;

export type Performance = {
  current: number;
  min: number;
  max: number;
  debounce: number;
  regress: () => void;
};

// export type Renderer = {
//   render: (scene: THREE.Scene, camera: THREE.Camera) => any;
// };

// export const isRenderer = (def: Renderer) => !!def?.render;
// export const isOrthographicCamera = (
//   def: THREE.Camera
// ): def is THREE.OrthographicCamera =>
//   def && (def as THREE.OrthographicCamera).isOrthographicCamera;

export type InternalState = {
  active: boolean;
  priority: number;
  frames: number;
  lastProps: StoreProps;
  lastEvent: { current: DomEvent | null };

  // interaction: THREE.Object3D[];
  hovered: Map<string, ThreeEvent<DomEvent>>;
  subscribers: Subscription[];
  capturedMap: Map<number, Map<Container, PointerCaptureTarget>>;
  initialClick: [x: number, y: number];
  // initialHits: THREE.Object3D[];

  subscribe: (callback: RenderCallback, priority?: number) => () => void;
};

type Clock = {
  oldTime: number;
  elapsedTime: number;
}

export type RootState = {
  app: Application<ICanvas>;
  mouse: Point;
  ticker: Ticker;

  frameloop: "always" | "demand" | "never";
  performance: Performance;

  size: Size;
  clock: Clock;

  set: SetState<RootState>;
  get: GetState<RootState>;
  invalidate: () => void;
  advance: (timestamp: number, runGlobalEffects?: boolean) => void;
  setSize: (width: number, height: number) => void;
  // setDpr: (dpr: Dpr) => void;
  setFrameloop: (frameloop?: "always" | "demand" | "never") => void;
  onPointerMissed?: (event: MouseEvent) => void;

  events: EventManager<any>;
  internal: InternalState;
};

export type ComputeOffsetsFunction = (
  event: any,
  state: RootState
) => { offsetX: number; offsetY: number };

export type StoreProps = {
  app: Application<ICanvas>;
  size: Size;
  frameloop?: "always" | "demand" | "never";
  performance?: Partial<Omit<Performance, "regress">>;
  dpr?: Dpr;
  ticker?: Ticker;
  onPointerMissed?: (event: MouseEvent) => void;
};

export type ApplyProps = (instance: Instance, newProps: InstanceProps) => void;

const ThreeContext = createContext<UseStore<RootState>>(null!);

const createPixiStore = (
  applyProps: ApplyProps,
  invalidate: (state?: RootState) => void,
  advance: (
    timestamp: number,
    runGlobalEffects?: boolean,
    state?: RootState
  ) => void,
  props: StoreProps
): UseStore<RootState> => {
  const {
    app,
    size,
    frameloop = "always",
    dpr = [1, 2],
    performance,
    ticker = new Ticker(),
    onPointerMissed,
  } = props;

  // clock.elapsedTime is updated using advance(timestamp)
  if (frameloop === "never") {
    app.ticker.stop();
    app.ticker.autoStart = false;
  } else {
    app.ticker.start();
  }

  const rootState = create<RootState>(
    subscribeWithSelector((set, get) => {
      const initialDpr = calculateDpr(dpr);

      let performanceTimeout: ReturnType<typeof setTimeout> | undefined =
        undefined;
      const setPerformanceCurrent = (current: number) =>
        set((state) => ({ performance: { ...state.performance, current } }));

      return {
        app,

        set,
        get,
        invalidate: () => invalidate(get()),
        advance: (timestamp: number, runGlobalEffects?: boolean) =>
          advance(timestamp, runGlobalEffects, get()),

        stage: prepare(app.stage),
        ticker,
        mouse: new Point(),

        clock: {
          oldTime: 0,
          elapsedTime: 0,
        },

        frameloop,
        onPointerMissed,

        performance: {
          current: 1,
          min: 0.5,
          max: 1,
          debounce: 200,
          ...performance,
          regress: () => {
            const state = get();
            // Clear timeout
            if (performanceTimeout) clearTimeout(performanceTimeout);
            // Set lower bound performance
            if (state.performance.current !== state.performance.min)
              setPerformanceCurrent(state.performance.min);
            // Go back to upper bound performance after a while unless something regresses meanwhile
            performanceTimeout = setTimeout(
              () => setPerformanceCurrent(get().performance.max),
              state.performance.debounce
            );
          },
        },

        size: { width: 800, height: 600 },
        viewport: {
          initialDpr,
          dpr: initialDpr,
          width: 0,
          height: 0,
          aspect: 0,
          distance: 0,
          factor: 0,
        },

        setSize: (width: number, height: number) => {
          const size = { width, height };
          set((state) => ({
            size,
          }));
        },
        // setDpr: (dpr: Dpr) =>
        //   set((state) => ({
        //     viewport: { ...state.viewport, dpr: calculateDpr(dpr) },
        //   })),

        setFrameloop: (frameloop: "always" | "demand" | "never" = "always") =>
          set(() => ({ frameloop })),

        events: { connected: false },
        internal: {
          active: false,
          priority: 0,
          frames: 0,
          lastProps: props,
          lastEvent: { current: null },

          interaction: [],
          hovered: new Map<string, ThreeEvent<DomEvent>>(),
          subscribers: [],
          initialClick: [0, 0],
          initialHits: [],
          capturedMap: new Map(),

          subscribe: (ref: RenderCallback, priority = 0) => {
            ticker.add(ref, state, priority);
            return () => {
              set(({ internal }) => ({
                internal: {
                  ...internal,
                  // Decrease manual flag if this subscription had a priority
                  priority: internal.priority - (priority > 0 ? 1 : 0),
                  // Remove subscriber from list
                  subscribers: internal.subscribers.filter(
                    (s) => s.ref !== ref
                  ),
                },
              }));
            };
          },
        },
      };
    })
  );

  const state = rootState.getState();

  // Resize camera and renderer on changes to size and pixelratio
  let oldSize = state.size;
  rootState.subscribe(() => {
    const { size } = rootState.getState();
    if (size !== oldSize /*|| dpr !== oldDpr*/) {
      // https://github.com/pmndrs/react-three-fiber/issues/92
      // Do not mess with the camera if it belongs to the user
      // Update renderer
      // app.renderer.resolution = dpr;
      app.renderer.resize(size.width, size.height);

      oldSize = size;
      // oldDpr = viewport.dpr;
    }
  });

  // Update size
  if (size) state.setSize(size.width, size.height);

  // Invalidate on any change
  rootState.subscribe((state) => invalidate(state));

  // Return root state
  return rootState;
};

export { createPixiStore as createThreeStore, ThreeContext };
