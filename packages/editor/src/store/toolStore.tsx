import { produce } from "solid-js/store";
import { AllMessages, GeneralHandler, generateStore } from ".";
import { arrayLast } from "../utils/array";
import { SelectToolStore, createSelectToolStore } from "./tools/select";
import { ToolInputMessage } from "./tools/shared";

type SubToolStores = SelectToolStore;

export enum Tool {
  Select,
}

export enum Cursor {
  Default,
  Grab,
  Grabbing,
  Point,
}

export const TOOL_TO_DEFAULT_CURSOR_MAP: Record<Tool, Cursor> = {
  [Tool.Select]: Cursor.Default,
}

export type ToolStoreMessage = {
  'tool:switch': Tool,
  'tool:push-cursor': Cursor,
  'tool:pop-cursor': Cursor|undefined,
  'tool:clear-cursor': void,
  'tool:input': ToolInputMessage,
}

export type ToolModel = {
  tool: Tool, // Base / fallback tool
  cursorStack: Cursor[], // Stack to overlay tools ontop of base tool
  currentCursor: Cursor, // Resolved current tool (stack overlayed on base)
}

export type ToolHandler = GeneralHandler<ToolStoreMessage>;

export const createToolStore = (dispatch: GeneralHandler<AllMessages>) => {
  const toolStore =  generateStore<ToolModel, ToolStoreMessage>({
    tool: Tool.Select,
    cursorStack: [],
    get currentCursor() {
      const last = arrayLast<Cursor>(this.cursorStack);
      return last ?? TOOL_TO_DEFAULT_CURSOR_MAP[this.tool as Tool];
    }
  }, {
    'tool:push-cursor': (store, set, message) => {
      set(produce(store => {
        store.cursorStack.push(message)
      }));
      store.currentCursor
    },
    'tool:pop-cursor': (_, set, message) => {
      set(produce(store => {
        const lastTool = arrayLast(store.cursorStack);
        if (message && (lastTool === undefined || lastTool === message))
          store.cursorStack.pop();
        else if (!message) {
          store.cursorStack.pop()
        } else {
          console.warn(`tool:pop: Did not pop tool as provided ${message} and current is ${lastTool}.`)
        }
      }))
    },
    'tool:clear-cursor': (_, set) => {
      set(produce(store => store.cursorStack = []))
    },
    'tool:switch': (_, set, message) => {
      set(produce(store => store.tool = message));
    },
    'tool:input': (store, _2, message) => {
      TOOL_TO_STORE_MAP[store.tool].handle('input', message, dispatch);
    }
  })
  const TOOL_TO_STORE_MAP: Record<Tool, SubToolStores> = {
    [Tool.Select]: createSelectToolStore(toolStore),
  }
  
  return toolStore;
}
