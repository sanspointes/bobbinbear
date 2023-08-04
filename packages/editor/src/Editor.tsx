import { Canvas } from "@bearbroidery/solixi"
import { Viewport } from "./components/Viewport"
import { sceneStore } from "./store"
import { Show, onMount } from 'solid-js';
import { SceneObjectChildren } from './components/scene/general';
import { Toolbar } from './components/Toolbar';
import { initialiseCommandPrototypeMap } from './store/commands';

const EditorView = () => {
  return <Viewport drag={true} pinch={true}>
    <Show when={sceneStore.root}>
      <SceneObjectChildren children={sceneStore.root} />
    </Show>
  </Viewport>
}

export const Editor = () => {
  onMount(() => {
    initialiseCommandPrototypeMap();
  })

  return (
    <div class="flex flex-col items-stretch w-full h-full">
      <Toolbar />
      <div class="flex-grow">
        <Canvas devtools={true}>
          <EditorView />
        </Canvas>
      </div>
    </div>
  )
}
