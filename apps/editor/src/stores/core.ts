import * as BobbinBearCore from '@bearbroidery/bobbinbear-core';
import { type BBTool, EditorApi } from '@bearbroidery/bobbinbear-core';
import { createStore } from 'solid-js/store';
import { BBStore } from '.';
import { CANVAS_ID } from '@/constants';

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

    let editorApi: EditorApi | undefined;
    const handleCoreInit = (instance: EditorApi) => {
        editorApi = instance;
    };

    const api: CoreApi = {
        async initEditor() {
            await BobbinBearCore.default();
            BobbinBearCore.main_web(`#${CANVAS_ID}`, handleCoreInit);
        },

        async setTool(tool: BBTool) {
            if (!editorApi) throw new Error('no editorApi.');
            return await editorApi.set_tool(tool);
        },
    };

    return [store, api];
};
