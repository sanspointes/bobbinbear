import { Canvas, PContainer, useSolixi } from "@bearbroidery/solixi"
import { AppContext, createAppStore} from "./store"
import { Show, createEffect, onMount, useContext } from 'solid-js';
import { Toolbar } from './components/Toolbar';
import { Cursor } from "./store/toolStore";
import { SceneObjectChildren } from "./sxi-components/general";
import { Viewport } from "./sxi-components/Viewport";

const EditorView = () => {
  const { sceneStore, dispatch } = useContext(AppContext);
  const pixi = useSolixi();
  
  onMount(() => {
    dispatch('input:set-source', {
      element: pixi.app.view as unknown as HTMLCanvasElement,
    })
  })

  createEffect(() => {
    console.log(Object.keys(pixi));
  })
  return <Viewport>
    <Show when={sceneStore.root}>
      <SceneObjectChildren children={sceneStore.root} />
    </Show>
  </Viewport>
}

export const Editor = () => {
  const contextModel = createAppStore();
  
  console.log(contextModel);

  return (
    <div class="flex flex-col items-stretch w-full h-full">
              <AppContext.Provider value={contextModel}>
        <Toolbar />
      <div class="flex-grow" classList={{
        'cursor-grab': contextModel.toolStore.currentCursor === Cursor.Grab,
        'cursor-grabbing': contextModel.toolStore.currentCursor === Cursor.Grabbing,
      }}>
        <Canvas devtools={true}>
          <PContainer>
                <EditorView />
          </PContainer>
        </Canvas>
      </div>
            </AppContext.Provider>
    </div>
  )
}
