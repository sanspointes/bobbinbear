/* eslint-disable solid/reactivity */
import { createStore, produce, SetStoreFunction } from "solid-js/store";
import { batch } from "solid-js";
import { uuid, Uuid } from "../utils/uuid";
import { Command } from "./commands";
import { generateStore } from ".";
import { arrayLast } from "../utils/array";
import { GraphicSceneObject, SceneObject } from "../types/scene";
import { Point } from "@pixi/core";

export type ObjectMapData<T extends SceneObject = SceneObject> = {
  object: T;
  set: SetStoreFunction<T>;
};

export type SceneStoreMessages = {
  "scene:hover": Uuid<SceneObject>;
  "scene:unhover": Uuid<SceneObject>;
  "scene:inspect": Uuid<SceneObject>;
  "scene:uninspect": void;
  "scene:do-command": Command;
  "scene:undo": void;
  "scene:redo": void;
};

export type SceneModel = {
  inspecting: Uuid<SceneObject> | undefined;
  selectedIds: Uuid<SceneObject>[];
  selectedObjects: SceneObject[];
  undoStack: Command[];
  redoStack: Command[];
  root: SceneObject;
};

export const createSceneStore = () => {
  const objMap = new Map<Uuid<SceneObject>, ObjectMapData>();

  // Set the root object, this can't be edited
  const [object, set] = createStore<SceneObject>({
    type: 'group',
    hovered: false,
    id: uuid('root'),
    name: 'Root',
    locked: false,
    shallowLocked: true,
    parent: undefined as unknown as Uuid<SceneObject>,
    visible: true,
    children: [],
    position: new Point(0, 0),
    selected: false,
  })
  objMap.set(uuid('root'), {
    object,
    set,
  });

  const result = generateStore<SceneModel, SceneStoreMessages>({
    inspecting: undefined,
    selectedIds: [],
    get selectedObjects() {
      return this.selectedIds.map((id: Uuid<SceneObject>) => {
        const result = objMap.get(id);
        if (!result) {
          throw new Error(
            `sceneStore.selectedObjects could not get object for id ${id}.`,
          );
        }
        return result.object;
      });
    },
    undoStack: [],
    redoStack: [],
    root: object,
  }, {
    "scene:hover": (_1, _2, uuid) => {
      const obj = objMap.get(uuid);
      if (obj) {
        obj.set("hovered", true);
      }
    },
    "scene:unhover": (_1, _2, uuid) => {
      const obj = objMap.get(uuid);
      if (obj) {
        obj.set("hovered", false);
      }
    },
    "scene:inspect": (_1, setStore, uuid) => {
      setStore(produce((store) => store.inspecting = uuid));
      const obj = objMap.get(uuid) as ObjectMapData<GraphicSceneObject>;
      if (obj && obj.object.inspecting !== undefined) {
        obj.set("inspecting", true);
      }
    },
    "scene:uninspect": (store, setStore) => {
      if (store.inspecting) {
        const obj = objMap.get(store.inspecting) as ObjectMapData<
          GraphicSceneObject
        >;
        if (obj && obj.object.inspecting !== undefined) {
          obj.set("inspecting", true);
        }
        setStore(produce((store) => store.inspecting = undefined));
      }
    },
    "scene:do-command": (store, set, command) => {
      const lastCommand = arrayLast(store.undoStack);
      if (lastCommand) {
        const sameType = lastCommand.type === command.type;
        const needsPush = lastCommand.final;
        const needsUpdate = !lastCommand.final && sameType;

        // Error if not an update of previous or a new command entirely
        if (!needsPush && !needsUpdate) {
          throw new Error(
            "perform-command: Invalid lastCommand/command.  Maybe you forgot to finalize the previous command?",
          );
        }

        if (needsUpdate) {
          if (!lastCommand.updateData) {
            throw new Error(
              `perform-command: Last Command marked as non final but no update method for ${lastCommand.type}:${lastCommand.name}`,
            );
          }

          // @ts-expect-error; Type is asserted to be same by the `sameType` check above.
          lastCommand.updateData(command);
          lastCommand.final = command.final;
        }

        // Perform the command
        const commandToPerform = needsUpdate ? lastCommand : command;
        commandToPerform.perform(store, set, objMap);

        // Push undo stack, clear redo stack
        if (needsPush) {
          set(produce((store) => {
            store.undoStack.push(command);
            store.redoStack = [];
          }));
        }
      } else {
        command.perform(store, set, objMap);

        set(produce((store) => {
          store.undoStack.push(command);
          store.redoStack = [];
        }));
      }
    },
    "scene:undo": (store, set) => {
      batch(() => {
        let command: Command | undefined;
        set(produce((store) => {
          command = store.undoStack.pop();
        }));
        if (command) {
          command.undo(store, set, objMap);

          set(produce((store) => {
            store.redoStack.push(command!);
          }));
        }
      });
    },
    "scene:redo": (store, set) => {
      batch(() => {
        let command: Command | undefined;
        set(produce((store) => {
          command = store.redoStack.pop();
        }));
        if (command) {
          command.perform(store, set, objMap);

          set(produce((store) => {
            store.undoStack.push(command!);
          }));
        }
      });
    },
  });

  return {
    ...result,
    objectMap: objMap,
  };
};
