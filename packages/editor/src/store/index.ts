import { createStore, produce, SetStoreFunction } from "solid-js/store";
import { batch } from "solid-js";
import { Command } from "./commands";
import { createSceneStore } from "./scene";
import { arrayLast } from "../utils/array";

export type WithSetter<T> = T & {
  set: SetStoreFunction<T>
}

type StoreMessages = {
  'perform-command': Command,
  'perform-undo': void,
  'perform-redo': void,
}

type SceneObjectStore = {
  undoStack: Command[],
  redoStack: Command[],
  dispatch: <TEvent extends keyof StoreMessages, TEventModel extends StoreMessages[TEvent]>(event: TEvent, model: TEventModel) => void,
}

// eslint-disable-next-line solid/reactivity
export const SceneObjectStore = createStore<SceneObjectStore>({
  undoStack: [],
  redoStack: [],
  dispatch: (event, model) => {
    MESSAGE_HANDLERS[event](model);
  }
})

export type StoreHandler<TCommands, TStore> = {
  store: TStore,
  performCommand: (history: Command[], command: TCommands) => void;
  undoCommand: (history: Command[], command: TCommands) => void;
}

const sceneStoreHandler = createSceneStore();
export const sceneStore = sceneStoreHandler.store;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const handlers: Record<Command['handler'], StoreHandler<any, any>> = {
  scene: sceneStoreHandler,
}

type MessageHandler<T> = (model: T) => void;
const MESSAGE_HANDLERS: { [K in keyof StoreMessages]: MessageHandler<StoreMessages[K]> } = {
  'perform-command': (command) => {
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
      const handler = handlers[command.handler];
      if (!handler) throw new Error(`perform-command: No handler for command ${command.name} with handler: ${command.handler}`);
      handler.performCommand(store.undoStack, commandToPerform);

      // Push undo stack, clear redo stack
      if (needsPush) {
        setStore(produce(store => {
          store.undoStack.push(command)
          store.redoStack = []
        }));
      }
    } else {
      const handler = handlers[command.handler];
      if (!handler) throw new Error(`perform-command: No handler for command ${command.name} with handler: ${command.handler}`);
      handler.performCommand(store.undoStack, command);

      setStore(produce(store => {
        store.undoStack.push(command)
        store.redoStack = []
      }));
    }
  },
  'perform-undo': () => {
    batch(() => {
      let command: Command|undefined;  
      setStore(produce(store => {
        command = store.undoStack.pop();
      }))
      if (command) {
        const handler = handlers[command.handler];
        if (!handler) throw new Error(`perform-command: No handler for command ${command.name} with handler: ${command.handler}`);

        handler.undoCommand(store.undoStack, command);

        setStore(produce(store => {
          store.redoStack.push(command!);
        }))
      }
    })
  },
  'perform-redo': () => {
    batch(() => {
      let command: Command|undefined;  
      setStore(produce(store => {
        command = store.redoStack.pop();
      }))
      if (command) {
        const handler = handlers[command.handler];
        if (!handler) throw new Error(`perform-command: No handler for command ${command.name} with handler: ${command.handler}`);

        handler.undoCommand(store.undoStack, command);

        setStore(produce(store => {
          store.undoStack.push(command!);
        }))
      }
    })
  }
};

export const [store, setStore] = SceneObjectStore;
