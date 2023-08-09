import {
  Canvas,
  PContainer,
  SolixiState,
  useSolixi,
} from "@bearbroidery/solixi";
import { AppContext, createAppStore } from "./store";
import {
  createSignal,
  onMount,
  Show,
  useContext,
} from "solid-js";
import { Toolbar } from "./components/Toolbar";
import { Cursor } from "./store/toolStore";
import { SceneObjectChildren } from "./sxi-components/general";
import { Viewport } from "./sxi-components/Viewport";
import { CursorTest } from "./sxi-components/CursorTest";
import { SelectBox } from "./sxi-components/SelectBox";
import { preventDefault } from "@solid-primitives/event-listener";

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
        <Show when={sceneStore.root}>
          <SceneObjectChildren children={sceneStore.root} />
        </Show>
      </Viewport>
    </>
  );
};

export const Editor = () => {
  const [solixi, setSolixi] = createSignal<SolixiState>();
  const onCreated = (state: SolixiState) => {
    console.log("Created", state);
    setSolixi(state);
  };

  const contextModel = createAppStore(solixi);
  let wrapperEl: HTMLDivElement | undefined;
  onMount(() => {
    contextModel.dispatch("input:set-source", {
      keys: wrapperEl,
    });
  })

  const onWheel = preventDefault(() => {});

  return (
    <div ref={wrapperEl} tabindex={0} class="flex flex-col items-stretch w-full h-full" onWheel={onWheel}>
      <AppContext.Provider value={contextModel}>
        <Toolbar />
        <div
          class="flex-grow"
          classList={{
            "cursor-grab": contextModel.toolStore.currentCursor === Cursor.Grab,
            "cursor-grabbing":
              contextModel.toolStore.currentCursor === Cursor.Grabbing,
            "cursor-pointer":
              contextModel.toolStore.currentCursor === Cursor.Point,
          }}
        >
          <Canvas devtools={true} onCreated={onCreated}>
            <EditorView />
          </Canvas>
        </div>
      </AppContext.Provider>
    </div>
  );
};
