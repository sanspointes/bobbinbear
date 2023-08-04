import { SceneStoreMessages, createSceneStore } from "./sceneStore";
import { ToolStoreMessage, createToolStore } from "./toolStore";

/**
 * TYPE DEFS
 */
import { SetStoreFunction, createStore } from "solid-js/store";

type BaseMessages = Record<string, unknown>
type BaseModel = Record<string, unknown>

type GeneralHandler<
  TMessages extends BaseMessages,
  K extends keyof TMessages = keyof TMessages,
  M extends TMessages[K] = TMessages[K]
> = (type: K, message: M) => void;

type BaseHandler<TStore, TMessage> = (store: TStore, set: SetStoreFunction<TStore>, message: TMessage) => void;

export type BaseMessageHandlers<TStore, TMessage extends BaseMessages> = {[K in keyof TMessage]: BaseHandler<TStore, TMessage[K]>}

type BaseStore<TModel, TMessages extends BaseMessages> = {
  store: TModel,
  handle: GeneralHandler<TMessages>;
}

/**
 * Generates a store that adheres to the dispatch model.
 *
 * @template TModel extends BaseModel - Model of store
 * @template TMessages extends BaseMessages - Message types
 * @param model - initial state of model
 * @param handlers - handlers for message types
 * @returns 
 */
export function generateStore<TModel extends BaseModel, TMessages extends BaseMessages>(
  model: TModel|(() => TModel),
  handlers: BaseMessageHandlers<TModel, TMessages> | GeneralHandler<TMessages>
): BaseStore<TModel, TMessages> {
  const resolvedModel: TModel = typeof(model) === 'function' ? (model as () => TModel)() : model;

  const [store, setStore] = createStore(resolvedModel);

  const handle: GeneralHandler<TMessages> = typeof(handlers) === 'function' 
    ? handlers as GeneralHandler<TMessages>
    : (type, message) => {
      handlers[type](store, setStore, message);
    };

  return {
    store,
    handle,
  } as BaseStore<TModel, TMessages>;
}

/**
 * Intialising Store
 */

type AllMessages = SceneStoreMessages & ToolStoreMessage;

type EditorModel = {
  temp: 1
}

export const createEditorStore = () => {
  const { store: sceneStore, handle: sceneHandler } = createSceneStore();
  const { store: toolStore, handle: toolHandle } = createToolStore();


  const res = generateStore<EditorModel, AllMessages>({
  temp: 1,
}, (type, message) => {
      if (type.startsWith('scene')) {
        // @ts-expect-error; Can't be bothered typing. 
        sceneHandler(type, message)
      } else if (type.startsWith('tool')) {
        // @ts-expect-error; Can't be bothered typing. 
        toolHandle(type, message);
      } else {
        throw new Error(`EditorStore: Unable to dispatch message to correct store.  Not store with prefix for ${type}.`)
      }
  });

  return {
    sceneStore,
    toolStore,
    dispatch: res.handle,
  }
}

export const { sceneStore, toolStore, dispatch } = createEditorStore()
