import { type BBTool } from '@bearbroidery/bobbinbear-core';
import { createStore } from 'solid-js/store';
import { BBStore } from '.';
import { CANVAS_ID } from '@/constants';
import { coreManager } from '@/core';

export type CoreState = {
    isInit: boolean;
    currentTool: BBTool;
};
export type CoreApi = {
    initEditor(): Promise<void>;

    setTool(tool: BBTool): Promise<boolean>;
};
const defaultCoreState = (): CoreState => ({
    isInit: false,
    currentTool: 'Select',
});

export const createCoreStore = (): BBStore<CoreState, CoreApi> => {
    const [store, set] = createStore(defaultCoreState());

    const api: CoreApi = {
        async initEditor() {
            coreManager.start(`#${CANVAS_ID}`);
        },

        async setTool(tool: BBTool) {
            const response = await coreManager.setTool(tool);
            const success = response.some((r) => r.tag === 'Success');
            return success;
        },
    };

    return [store, api];
};
