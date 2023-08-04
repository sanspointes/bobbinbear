import { produce } from "solid-js/store";
import { generateStore } from ".";
import { arrayLast } from "../utils/array";

export enum Tool {
  Default,
  Grab,
  Grabbing,
}

export type ToolStoreMessage = {
  'tool:switch': Tool,
  'tool:push': Tool,
  'tool:pop': Tool|undefined,
}

type ToolModel = {
  baseTool: Tool, // Base / fallback tool
  toolStack: Tool[], // Stack to overlay tools ontop of base tool
  currentTool: Tool, // Current tool (stack overlayed on base)
}

export const createToolStore = () => {
  return generateStore<ToolModel, ToolStoreMessage>({
    baseTool: Tool.Default,
    toolStack: [],
    get currentTool() {
      const last = arrayLast<Tool>(this.toolStack);
      return last ?? this.baseTool;
    }
  }, {
      'tool:pop': (_, set, message) => {
        set(produce(store => {
          const lastTool = arrayLast(store.toolStack);
          if (message && (lastTool === undefined || lastTool === message))
            store.toolStack.pop();
          else if (!message) {
            store.toolStack.pop()
          } else {
            console.warn(`tool:pop: Did not pop tool as provided ${message} and current is ${lastTool}.`)
          }
        }))
      },
      'tool:push': (_, set, message) => {
        set(produce(store => store.toolStack.push(message)));
      },
      'tool:switch': (_, set, message) => {
        set(produce(store => store.baseTool = message));
      }
    })
}
