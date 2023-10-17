import { createContext, useContext } from 'solid-js';
import { CoreApi, CoreState, createCoreStore } from './core';

export type BBStore<TState, TApi> = [model: TState, api: TApi];

type AppState = {
    core: CoreState;
};

type AppApi = {
    core: CoreApi;
};

export const createAppStore = (): BBStore<AppState, AppApi> => {
    const [coreState, coreApi] = createCoreStore();

    const state: AppState = {
        core: coreState,
    };
    const api: AppApi = {
        core: coreApi,
    };
    return [state, api];
};

export const AppContext = createContext<BBStore<AppState, AppApi>>();

export const useAppStore = (): BBStore<AppState, AppApi> => {
    const store = useContext(AppContext);
    if (!store) {
        throw new Error(
            'useAppStore: Must be used within an AppContext provider.',
        );
    }
    return store;
};
