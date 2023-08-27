/* eslint-disable solid/reactivity */
import { createStore, produce, SetStoreFunction } from "solid-js/store";
import { batch } from "solid-js";
import { Point } from "@pixi/core";
import { ReactiveMap } from "@solid-primitives/map";

import { Uuid, uuid } from "../utils/uuid";
import { Command } from "./commands";
import { generateStore } from ".";
import { arrayLast } from "../utils/array";
import {
  BaseSceneObject,
  GroupSceneObject,
  HasInspectSceneObject,
  SceneObject,
} from "../types/scene";

export const getObject = <T extends BaseSceneObject>(
  store: SceneModel,
  uuid: Uuid<T>|undefined,
): T | undefined => {
  if ((uuid) === undefined) return undefined;
  return store.objects.get(uuid) as T | undefined;
};
export const getObjectSetter = <T extends BaseSceneObject>(
  store: SceneModel,
  uuid: Uuid<T> | undefined,
): SetStoreFunction<T> | undefined => {
  if (uuid === undefined) return undefined;
  const setter = store.objectSetters.get(uuid);
  if (!setter) return undefined;
  return setter as SetStoreFunction<T>;
};

export const isInspectable = <T extends BaseSceneObject>(obj: T): obj is T & HasInspectSceneObject => {
  const o = obj as unknown as T & HasInspectSceneObject;
  if (o.inspecting !== undefined) return true;
  return false;
}
/**
 * Keep a flat reference to every object on the scene and its setter function
 */
export type ObjectMapData<T extends SceneObject = SceneObject> = {
  object: T;
  set: SetStoreFunction<T>;
};

export type SceneStoreMessages = {
  "scene:hover": Uuid<BaseSceneObject>;
  "scene:unhover": Uuid<BaseSceneObject>;
  "scene:do-command": Command;
  "scene:undo": void;
  "scene:redo": void;
};

export type SceneModel = {
  /** UUID of object that we're currently inspecting */
  inspecting: Uuid<BaseSceneObject> | undefined;
  /** UUID of inspect root object, used for storing temporary parts of the document i.e. nodes */
  inspectRoot: Uuid<BaseSceneObject> | undefined;
  /* List of selected ids */
  selectedIds: Uuid<BaseSceneObject>[];
  selectedObjects: BaseSceneObject[];
  undoStack: Command[];
  redoStack: Command[];
  objects: ReactiveMap<Uuid<BaseSceneObject>, BaseSceneObject>;
  objectSetters: Map<Uuid<BaseSceneObject>, SetStoreFunction<BaseSceneObject>>;
  root: BaseSceneObject;
};

export const createSceneStore = () => {
  // Set the root object, this can't be edited
  const [object, set] = createStore<GroupSceneObject>({
    type: "group",
    hovered: false,
    id: uuid("root"),
    name: "Root",
    locked: false,
    shallowLocked: true,
    parent: undefined as unknown as Uuid<SceneObject>,
    visible: true,
    children: [],
    position: new Point(0, 0),
    selected: false,
  });

  const result = generateStore<SceneModel, SceneStoreMessages>({
    inspecting: undefined,
    inspectRoot: undefined,
    selectedIds: [],
    selectedObjects: [],
    undoStack: [],
    redoStack: [],
    objects: new ReactiveMap([[uuid("root"), object]]),
    objectSetters: new Map([[uuid("root"), set]]),
    root: object,
  }, {
    "scene:hover": (store, _2, uuid) => {
      const set = getObjectSetter(store, uuid);
      if (set) set("hovered", true);
    },
    "scene:unhover": (store, _2, uuid) => {
      const set = getObjectSetter(store, uuid);
      if (set) set("hovered", false);
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
        commandToPerform.perform(store, set);

        // Push undo stack, clear redo stack
        if (needsPush) {
          set(produce((store) => {
            store.undoStack.push(command);
            store.redoStack = [];
          }));
        }
      } else {
        command.perform(store, set);

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
          command.undo(store, set);

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
          command.perform(store, set);

          set(produce((store) => {
            store.undoStack.push(command!);
          }));
        }
      });
    },
  });

  // @ts-expect-error Debug only inspection
  window.sceneStore = result.store;

  return result;
};
