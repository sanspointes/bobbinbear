import {
  Canvas,
  PContainer,
  SolixiState,
  useSolixi,
} from "@bearbroidery/solixi";
import { AppContext, createAppStore } from "./store";
import {
    ErrorBoundary,
  createSignal,
  onMount,
  useContext,
} from "solid-js";
import { Toolbar } from "./components/Toolbar";
import { Cursor } from "./store/toolStore";
import { SceneObjectChildren } from "./sxi-components/general";
import { Viewport } from "./sxi-components/Viewport";
import { CursorTest } from "./sxi-components/CursorTest";
import { SelectBox } from "./sxi-components/SelectBox";
import { preventDefault } from "@solid-primitives/event-listener";
import { Sidebar } from "./components/Sidebar";
import { Tree } from "./components/Tree";

const EditorView = () => {
  const { sceneStore, dispatch } = useContext(AppContext);
  const pixi = useSolixi();

  onMount(() => {
    dispatch("input:set-source", {
      pointer: pixi.app.view as unknown as HTMLCanvasElement,
    });
  });

  return (
    <>
      <CursorTest />
      <SelectBox />
      <Viewport>
        <SceneObjectChildren children={[sceneStore.root]} />
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
  })

  const onWheel = preventDefault(() => {});

  return (
    <ErrorBoundary fallback={err => err}>
      <div ref={wrapperEl} tabindex={0} class="flex flex-col items-stretch w-full h-full" onWheel={onWheel}>
        <AppContext.Provider value={contextModel}>
          <Toolbar />
          <div
            class="flex-grow flex"
            classList={{
              "cursor-grab": toolStore.currentCursor === Cursor.Grab,
              "cursor-grabbing":
                toolStore.currentCursor === Cursor.Grabbing,
              "cursor-pointer":
                toolStore.currentCursor === Cursor.Point,
              "cursor-crosshair": 
                toolStore.currentCursor === Cursor.Cross,
            }}
          >
            <Tree />
            <Canvas devtools={true} onCreated={onCreated} app={{backgroundColor: 0xE1E1E1}}>
              <EditorView />
            </Canvas>
            <Sidebar />
          </div>
        </AppContext.Provider>
      </div>
    </ErrorBoundary>
  );
};
