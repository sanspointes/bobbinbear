/* eslint-disable solid/reactivity */
import { SetStoreFunction, produce } from "solid-js/store";
import { batch } from "solid-js";
import { Uuid } from "../utils/uuid";
import { Command } from "./commands";
import { generateStore } from ".";
import { arrayLast } from "../utils/array";
import { SceneObject } from "../types/scene";

export type ObjectMapData = {
  object: SceneObject,
  set: SetStoreFunction<SceneObject>,
}

export type SceneStoreMessages = {
  'scene:do-command': Command;
  'scene:undo': void,
  'scene:redo': void,
}

export type SceneModel = {
  undoStack: Command[],
  redoStack: Command[],
  root: SceneObject[],
}

export const createSceneStore = () => {

  const objMap = new Map<Uuid<SceneObject>, ObjectMapData>();

  return generateStore<SceneModel, SceneStoreMessages>({
    undoStack: [],
    redoStack: [],
    root: [],
  }, {
    'scene:do-command': (store, set, command) => {
      const lastCommand = arrayLast(store.undoStack);
      if (lastCommand) {
        const sameType = lastCommand.type === command.type;
        const needsPush = lastCommand.final;
        const needsUpdate = !lastCommand.final && sameType;

        // Error if not an update of previous or a new command entirely
        if (!needsPush && !needsUpdate) {
          throw new Error('perform-command: Invalid lastCommand/command.  Maybe you forgot to finalize the previous command?');
        }

        if (needsUpdate) {
          if (!lastCommand.updateData) throw new Error(`perform-command: Last Command marked as non final but no update method for ${lastCommand.type}:${lastCommand.name}`);

          // @ts-expect-error; Type is asserted to be same by the `sameType` check above.
          lastCommand.updateData(command);
        }

        // Perform the command
        const commandToPerform = needsUpdate ? lastCommand : command;
        commandToPerform.perform(store, set, objMap);

        // Push undo stack, clear redo stack
        if (needsPush) {
          set(produce(store => {
            store.undoStack.push(command)
            store.redoStack = []
          }));
        }
      } else {
        command.perform(store, set, objMap);

        set(produce(store => {
          store.undoStack.push(command)
          store.redoStack = []
        }));
      }
        },
        'scene:undo': (store, set) => {
          batch(() => {
            let command: Command|undefined;  
            set(produce(store => {
              command = store.undoStack.pop();
            }))
            if (command) {
              command.undo(store, set, objMap);

              set(produce(store => {
                store.redoStack.push(command!);
              }))
            }
          })
        },
        'scene:redo': (store, set) => {
      batch(() => {
        let command: Command|undefined;  
        set(produce(store => {
          command = store.redoStack.pop();
        }))
        if (command) {
          command.perform(store, set, objMap);

          set(produce(store => {
            store.undoStack.push(command!);
          }))
        }
      })
    }
  })
}
