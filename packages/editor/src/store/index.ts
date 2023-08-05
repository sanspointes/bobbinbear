import { createContext } from "solid-js";
import { SceneStoreMessages, SceneModel, createSceneStore } from "./sceneStore";
import { ToolModel, ToolStoreMessage, createToolStore } from "./toolStore";

/**
 * TYPE DEFS
 */
import { SetStoreFunction, createStore } from "solid-js/store";
import { InputMessages, InputModel, createInputStore } from "./inputStore";

type BaseMessages = Record<string, unknown>
type BaseModel = Record<string, unknown>

type BaseHandlerResponder = <K extends keyof AllMessages>(type: K, message: AllMessages[K]) => void;
export type GeneralHandler<
  TMessages extends BaseMessages,
  K extends keyof TMessages = keyof TMessages,
  M extends TMessages[K] = TMessages[K]
> = (type: K, message: M, responder?: BaseHandlerResponder) => void;

type BaseHandler<TStore, TMessage> = (store: TStore, set: SetStoreFunction<TStore>, message: TMessage, responder?: BaseHandlerResponder) => void;
export type BaseMessageHandlers<TStore, TMessage extends BaseMessages> = {[K in keyof TMessage]: BaseHandler<TStore, TMessage[K]>}

export type BaseStore<TModel, TMessages extends BaseMessages> = {
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
  model: TModel,
  handlers: BaseMessageHandlers<TModel, TMessages> | GeneralHandler<TMessages>
): BaseStore<TModel, TMessages> {
  const resolvedModel: TModel = typeof(model) === 'function' ? (model as () => TModel)() : model;

  const [store, setStore] = createStore(resolvedModel);


  const handle: GeneralHandler<TMessages> = typeof(handlers) === 'function' 
    ? handlers as GeneralHandler<TMessages>
    : (type, message, respond) => {
      handlers[type](store, setStore, message, respond);
    };

  return {
    store,
    handle,
  } as BaseStore<TModel, TMessages>;
}

/**
 * Intialising Store
 */

export type AllMessages = SceneStoreMessages & ToolStoreMessage & InputMessages;

type EditorModel = {
  temp: 1
}

export type AppDispatcher = GeneralHandler<AllMessages>;

export const createAppStore = () => {
  let sceneStore: SceneModel,
    sceneHandler: GeneralHandler<SceneStoreMessages>;

  let inputStore: InputModel,
    inputHandler: GeneralHandler<InputMessages>;
  let toolStore: ToolModel,
    toolHandler: GeneralHandler<ToolStoreMessage>;

  const res = generateStore<EditorModel, AllMessages>({
  temp: 1,
}, (type, message) => {

      const responses = [{
        type,
        message,
      }];

      const handleResponse = (type: keyof AllMessages, message: AllMessages[keyof AllMessages]) => {
        responses.push({
          type,
          message,
        })
      }

      for (const {type, message} of responses) {
        if (!type.startsWith('input') && type !== 'tool:input')
          console.debug('Handing message ', type, message)

        if (type.startsWith('scene')) {
          // @ts-expect-error; Can't be bothered typing. 
          sceneHandler(type, message, handleResponse)
        } else if (type.startsWith('tool')) {
          // @ts-expect-error; Can't be bothered typing. 
          toolHandler(type, message, handleResponse);
        } else if (type.startsWith('input')) {
          // @ts-expect-error; Can't be bothered typing. 
          inputHandler(type, message, handleResponse);
        } else {
          throw new Error(`EditorStore: Unable to dispatch message to correct store.  Not store with prefix for ${type}.`)
        }
      }
  });

  const sceneResult = createSceneStore();
  sceneStore = sceneResult.store;
  sceneHandler = sceneResult.handle;
  const inputResult = createInputStore(res.handle);
  inputStore = inputResult.store;
  inputHandler = inputResult.handle;
  const toolResult = createToolStore(res.handle);
  toolStore = toolResult.store;
  toolHandler = toolResult.handle;

  const [finalStore, _set] = createStore({
    inputStore,
    sceneStore,
    toolStore,
    dispatch: res.handle,
  })
  return finalStore;
}

type AppContextModel = {
  inputStore: InputModel,
  sceneStore: SceneModel,
  toolStore: ToolModel,
  dispatch: GeneralHandler<AllMessages>,
}
export const AppContext = createContext({ app_context: true } as unknown as AppContextModel)
