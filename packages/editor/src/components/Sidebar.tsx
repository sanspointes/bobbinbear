import { Show, createMemo, useContext } from "solid-js"
import { AppContext } from "../store"
import { arrayFirst } from "../utils/array"
import { SidebarTransform } from "./Sidebar.Transform";
import { Accordion } from "./generics/Accordian";
import { SidebarDebug } from "./Sidebar.Debug";
import { SidebarSceneObject } from "./Sidebar.SceneObject";

export function Sidebar () {
  const { sceneStore } = useContext(AppContext)
  const firstObject = createMemo(() => arrayFirst(sceneStore.selectedObjects));

  return (
    <div class="p-4 bg-yellow-400 border-l border-yellow-500 border-solid w-[400px] box-border">
      <div class="grid grid-cols-2 gap-4">
        <Accordion class="col-span-2" multiple collapsible>
          <Show when={firstObject()}>
            {(obj) => (
              <>
                <SidebarSceneObject object={obj()} />
                <SidebarTransform object={obj()} />
              </>
            )}
          </Show>
          <SidebarDebug />
        </Accordion>
      </div>
    </div>
  )
}
