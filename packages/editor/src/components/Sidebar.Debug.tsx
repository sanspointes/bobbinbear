import { useContext } from "solid-js"
import { SceneObject } from "../types/scene"
import { AccordionItem } from "./generics/Accordian"
import { AppContext } from "../store"

export function SidebarDebug() {
  const { toolStore, inputStore, sceneStore } = useContext(AppContext);

  const { boxTool, selectTool } = toolStore;

  return (
    <AccordionItem value="debug" header="Debug">
      <h3>SelectTool</h3>
      <p>State: {selectTool.state().toString()}</p>
      <p>Is selecting: {selectTool.isSelecting}</p>
      <div class="border-b border-solid border-yellow-500"></div>
      <h3>BoxTool</h3>
      <p>State: {boxTool.state().toString()}</p>

    </AccordionItem>
  )
}
