import { produce } from "solid-js/store";
import { AllMessages, GeneralHandler, generateStore } from ".";
import { arrayLast } from "../utils/array";
import {
  createSelectToolStore,
  SelectToolModel,
  SelectToolStore,
} from "./tools/select";
import { ToolInputMessage } from "./tools/shared";
import { Accessor } from "solid-js";
import { SolixiState } from "@bearbroidery/solixi";
import { SceneModel } from "./sceneStore";
import { InputModel } from "./inputStore";
import { BoxToolModel, BoxToolStore, createBoxToolStore } from "./tools/box";

type SubToolStores = SelectToolStore | BoxToolStore;

export enum Tool {
  None = 'None',
  Select = 'Select',
  Box = 'Box',
}

export enum Cursor {
  Default,
  Grab,
  Grabbing,
  Point,
  Cross,
}

export const TOOL_TO_DEFAULT_CURSOR_MAP: Record<Tool, Cursor> = {
  [Tool.None]: Cursor.Default,
  [Tool.Select]: Cursor.Default,
  [Tool.Box]: Cursor.Cross,
};

export type ToolStoreMessage = {
  "tool:switch": Tool;
  "tool:push-cursor": Cursor;
  "tool:pop-cursor": Cursor | undefined;
  "tool:clear-cursor": Cursor | Cursor[] | undefined;
  "tool:input": ToolInputMessage;
};

export type ToolModel = {
  tool: Tool; // Base / fallback tool
  cursorStack: Cursor[]; // Stack to overlay tools ontop of base tool
  currentCursor: Cursor; // Resolved current tool (stack overlayed on base)

  selectTool: SelectToolModel;
  boxTool: BoxToolModel;
};

export type ToolHandler = GeneralHandler<ToolStoreMessage>;

export const createToolStore = (
  dispatch: GeneralHandler<AllMessages>,
  solixi: Accessor<SolixiState | undefined>,
  inputModel: InputModel,
  sceneModel: SceneModel,
) => {
  const TOOL_TO_STORE_MAP: Record<Tool, SubToolStores | undefined> = {
    [Tool.None]: undefined,
    [Tool.Select]: createSelectToolStore(dispatch, solixi, inputModel, sceneModel),
    [Tool.Box]: createBoxToolStore(dispatch),
  };

  const model: ToolModel = {
    tool: Tool.None,
    cursorStack: [],
    get currentCursor() {
      const last = arrayLast<Cursor>(this.cursorStack);
      return last ?? TOOL_TO_DEFAULT_CURSOR_MAP[this.tool as Tool];
    },

    selectTool: TOOL_TO_STORE_MAP[Tool.Select]!.store as SelectToolModel,
    boxTool: TOOL_TO_STORE_MAP[Tool.Box]!.store as BoxToolModel,
  };

  const toolStore = generateStore<ToolModel, ToolStoreMessage>(model, {
    "tool:push-cursor": (store, set, message) => {
      set(produce((store) => {
        store.cursorStack.push(message);
      }));
      store.currentCursor;
    },
    "tool:pop-cursor": (_, set, message) => {
      set(produce((store) => {
        const lastTool = arrayLast(store.cursorStack);
        if (message && (lastTool === undefined || lastTool === message)) {
          store.cursorStack.pop();
        } else if (!message) {
          store.cursorStack.pop();
        } else {
          console.warn(
            `tool:pop: Did not pop tool as provided ${message} and current is ${lastTool}.`,
          );
        }
      }));
    },
    "tool:clear-cursor": (_, set, toClear) => {
      set(produce((store) => {
        if (toClear === undefined) store.cursorStack = [];
        else if (!Array.isArray(toClear)) {
          store.cursorStack = store.cursorStack.filter((c) => c !== toClear);
        } else {
          store.cursorStack = store.cursorStack.filter((c) =>
            !toClear.includes(c)
          );
        }
      }));
    },
    "tool:switch": (_, set, newTool) => {
      set(produce((store) => {
        if (store.tool !== newTool) {
          const oldToolStore = TOOL_TO_STORE_MAP[store.tool];
          if (oldToolStore) {
            oldToolStore.handle("deactivate", undefined, dispatch);
          }

          store.tool = newTool;

          const newToolStore = TOOL_TO_STORE_MAP[store.tool];
          if (newToolStore) {
            newToolStore.handle("activate", undefined, dispatch);
          }
        }
      }));
    },
    "tool:input": (store, _2, message) => {
      const toolStore = TOOL_TO_STORE_MAP[store.tool];
      if (toolStore) toolStore.handle("input", message, dispatch);
    },
  });
  return toolStore;
};
