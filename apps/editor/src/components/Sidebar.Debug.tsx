import { useContext } from "solid-js";
import { SceneObject } from "../types/scene";
import { AccordionItem } from "./generics/Accordian";
import { AppContext } from "../store";
import { Cursor } from "../store/toolStore";

export function SidebarDebug() {
  const { toolStore, sceneStore } = useContext(AppContext);

  const { boxTool, selectTool } = toolStore;

  return (
    <AccordionItem value="debug" header="Debug">
      <p>tool: {toolStore.tool.toString()}</p>
      <p>
        cursor: {toolStore.cursorStack.map((c) => <span>{Cursor[c]}</span>)}
      </p>
      <h3>SelectTool: {selectTool.state().toString()}</h3>
      <h3>BoxTool: {boxTool.state().toString()}</h3>
      <div class="border-b border-yellow-800 border-solid" />
      <h3>Scene</h3>
      <p>Inspecting: {sceneStore.inspecting}</p>
      <p>Selected IDs: {sceneStore.selectedIds.join(",")}</p>
    </AccordionItem>
  );
}
