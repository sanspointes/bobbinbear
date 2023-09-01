import { Canvas, SolixiState, useSolixi } from "@bearbroidery/solixi";
import {
  ErrorBoundary,
  createRenderEffect,
  createSignal,
  onMount,
  useContext,
} from "solid-js";

import { preventDefault } from "@solid-primitives/event-listener";
import {
  DragDropProvider,
  DragDropSensors,
} from "@thisbeyond/solid-dnd";

import { AppContext, createAppStore } from "./store";
import { Cursor } from "./store/toolStore";

import { ErrorView } from "./components/ErrorView";
import { SidebarLeft } from "./components/SceneTree";
import { Sidebar } from "./components/Sidebar";
import { Toolbar } from "./components/Toolbar";

import { SceneObjectChildren } from "./emb-objects";
import { CursorTest } from "./sxi-components/CursorTest";
import { SelectBox } from "./sxi-components/SelectBox";
import { Viewport } from "./sxi-components/Viewport";

import { uuid } from "./utils/uuid";

export const [appError, setAppError] = createSignal<Error>();

const EditorView = () => {
  // Report errors
  createRenderEffect(() => {
    if (appError()) {
      throw new Error("An Async error occured: ", { cause: appError() });
    }
  });

  const { sceneStore, dispatch } = useContext(AppContext);
  const pixi = useSolixi();

  onMount(() => {
    dispatch("input:set-source", {
      pointer: pixi.app.view as unknown as HTMLCanvasElement,
    });
  });

  const rootObject = sceneStore.objects.get(uuid("root"));

  return (
    <>
      <CursorTest />
      <SelectBox />
      <Viewport>
        <SceneObjectChildren children={rootObject!.children} />
      </Viewport>
    </>
  );
};

export const Editor = () => {
  const [solixi, setSolixi] = createSignal<SolixiState>();
  const onCreated = (state: SolixiState) => {
    setSolixi(state);
  };

  const contextModel = createAppStore(solixi);
  const { toolStore } = contextModel;
  let wrapperEl: HTMLDivElement | undefined;
  onMount(() => {
    contextModel.dispatch("input:set-source", {
      keys: wrapperEl,
    });
  });

  const onWheel = preventDefault(() => {});

  return (
      <div
        ref={wrapperEl}
        tabindex={0}
        class="flex flex-col items-stretch w-full h-full text-orange-50 fill-orange-50 stroke-orange-50"
        onWheel={onWheel}
      >
        <ErrorBoundary fallback={error => <ErrorView error={error} stack={contextModel.sceneStore.undoStack} />}>
          <AppContext.Provider value={contextModel}>
            <Toolbar />
            <div
              class="flex flex-grow"
            >
              <DragDropProvider>
                <DragDropSensors>
                  <SidebarLeft />
                </DragDropSensors>
              </DragDropProvider>
              <Canvas
                classList={{
                  "cursor-grab": toolStore.currentCursor === Cursor.Grab,
                  "cursor-grabbing": toolStore.currentCursor === Cursor.Grabbing,
                  "cursor-pointer": toolStore.currentCursor === Cursor.Point,
                  "cursor-crosshair": toolStore.currentCursor === Cursor.Cross,
                }}
                devtools={true}
                onCreated={onCreated}
                app={{ backgroundColor: 0xE1E1E1, resolution: window.devicePixelRatio }}
              >
                <EditorView />
              </Canvas>
              <Sidebar />
            </div>
          </AppContext.Provider>
        </ErrorBoundary>
      </div>
  );
};
