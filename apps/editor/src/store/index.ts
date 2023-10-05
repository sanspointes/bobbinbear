import { Accessor, createContext } from 'solid-js';
import { SceneStoreMessages, SceneModel, createSceneStore } from './sceneStore';
import {
    Tool,
    ToolModel,
    ToolStoreMessage,
    createToolStore,
} from './toolStore';

/**
 * TYPE DEFS
 */
import { SetStoreFunction, createStore } from 'solid-js/store';
import { InputMessages, InputModel, createInputStore } from './inputStore';
import { SolixiState } from '@bearbroidery/solixi';
import {
    ViewportMessage,
    ViewportModel,
    createViewportStore,
} from './viewportStore';
import {
    SettingsMessage,
    SettingsModel,
    createSettingsStore,
} from './settingsStore';
import {
    DocumentMessage,
    DocumentModel,
    createDocumentStore,
} from './documentStore';

type BaseMessages = Record<string, unknown>;
type BaseModel = Record<string, unknown>;

type BaseHandlerResponder = <K extends keyof AllMessages>(
    type: K,
    message: AllMessages[K],
) => void;
export type GeneralHandler<
    TMessages extends BaseMessages,
    K extends keyof TMessages = keyof TMessages,
    M extends TMessages[K] = TMessages[K],
> = (type: K, message: M, responder?: BaseHandlerResponder) => void;

type BaseHandler<TStore, TMessage> = (
    store: TStore,
    set: SetStoreFunction<TStore>,
    message: TMessage,
    responder?: BaseHandlerResponder,
) => void;
export type BaseMessageHandlers<TStore, TMessage extends BaseMessages> = {
    [K in keyof TMessage]: BaseHandler<TStore, TMessage[K]>;
};

export type BaseStore<TModel, TMessages extends BaseMessages> = {
    store: TModel;
    handle: GeneralHandler<TMessages>;
};

/**
 * Generates a store that adheres to the dispatch model.
 *
 * @template TModel extends BaseModel - Model of store
 * @template TMessages extends BaseMessages - Message types
 * @param model - initial state of model
 * @param handlers - handlers for message types
 * @returns
 */
export function generateStore<
    TModel extends BaseModel,
    TMessages extends BaseMessages,
>(
    model: TModel,
    handlers:
        | BaseMessageHandlers<TModel, TMessages>
        | GeneralHandler<TMessages>,
): BaseStore<TModel, TMessages> {
    const resolvedModel: TModel =
        typeof model === 'function' ? (model as () => TModel)() : model;

    const [store, setStore] = createStore(resolvedModel);

    const handle: GeneralHandler<TMessages> =
        typeof handlers === 'function'
            ? (handlers as GeneralHandler<TMessages>)
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

export type AllMessages = SceneStoreMessages &
    ToolStoreMessage &
    InputMessages &
    ViewportMessage &
    SettingsMessage &
    DocumentMessage;

type EditorModel = {
    temp: 1;
};

export type AppDispatcher = GeneralHandler<AllMessages>;

export const createAppStore = (solixi: Accessor<SolixiState | undefined>) => {
    // Pre-declare handlers so they can be referenced by the main store.
    // let sceneStore: SceneModel,
    //     sceneHandler: GeneralHandler<SceneStoreMessages>;
    // let inputStore: InputModel, inputHandler: GeneralHandler<InputMessages>;
    // let toolStore: ToolModel, toolHandler: GeneralHandler<ToolStoreMessage>;
    // let viewportStore: ViewportModel,
    //     viewportHandler: GeneralHandler<ViewportMessage>;

    const model: EditorModel = {
        temp: 1,
    };

    const appStoreResult = generateStore<EditorModel, AllMessages>(
        model,
        (type, message) => {
            const responses = [
                {
                    type,
                    message,
                },
            ];

            const handleResponse = (
                type: keyof AllMessages,
                message: AllMessages[keyof AllMessages],
            ) => {
                responses.push({
                    type,
                    message,
                });
            };

            for (const { type, message } of responses) {
                if (type.startsWith('scene')) {
                    // @ts-expect-error; Can't be bothered typing.
                    sceneHandler(type, message, handleResponse);
                } else if (type.startsWith('tool')) {
                    // @ts-expect-error; Can't be bothered typing.
                    toolHandler(type, message, handleResponse);
                } else if (type.startsWith('input')) {
                    // @ts-expect-error; Can't be bothered typing.
                    inputHandler(type, message, handleResponse);
                } else if (type.startsWith('viewport')) {
                    // @ts-expect-error; Can't be bothered typing.
                    viewportHandler(type, message, handleResponse);
                } else if (type.startsWith('settings')) {
                    // @ts-expect-error; Can't be bothered typing.
                    settingsHandler(type, message, handleResponse);
                } else if (type.startsWith('document')) {
                    // @ts-expect-error; Can't be bothered typing.
                    documentHandler(type, message, handleResponse);
                } else {
                    throw new Error(
                        `EditorStore: Unable to dispatch message to correct store.  Not store with prefix for ${type}.`,
                    );
                }
            }
        },
    );

    // Assign handlers
    const { store: sceneStore, handle: sceneHandler } = createSceneStore();
    const { store: inputStore, handle: inputHandler } = createInputStore(
        appStoreResult.handle,
    );
    const { store: toolStore, handle: toolHandler } = createToolStore(
        appStoreResult.handle,
        solixi,
        inputStore,
        sceneStore,
    );
    const { store: viewportStore, handle: viewportHandler } =
        createViewportStore(appStoreResult.handle);
    const { store: settingsStore, handle: settingsHandler } =
        createSettingsStore();
    const { store: documentStore, handle: documentHandler } =
        createDocumentStore(appStoreResult.handle, sceneStore);

    // Setup initial state
    appStoreResult.handle('tool:switch', Tool.Select);

    const [finalStore, _set] = createStore({
        inputStore,
        sceneStore,
        toolStore,
        viewportStore,
        settingsStore,
        documentStore,
        dispatch: appStoreResult.handle,
    });
    return finalStore;
};

type AppContextModel = {
    inputStore: InputModel;
    sceneStore: SceneModel;
    toolStore: ToolModel;
    viewportStore: ViewportModel;
    settingsStore: SettingsModel;
    documentStore: DocumentModel;
    dispatch: GeneralHandler<AllMessages>;
};
export const AppContext = createContext({
    app_context: true,
} as unknown as AppContextModel);
