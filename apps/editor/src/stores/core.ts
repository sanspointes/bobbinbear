import {
    JsApiEffectMsg,
    type BBTool,
    BBCursor,
} from '@bearbroidery/bobbinbear-core';
import { createStore } from 'solid-js/store';
import { BBStore } from '.';
import { CANVAS_ID } from '@/constants';
import { coreManager } from '@/core';
import { match } from 'ts-pattern';

export type CoreState = {
    isInit: boolean;
    cursor: BBCursor;
    currentTool: BBTool;
};
export type CoreApi = {
    initEditor(): Promise<void>;

    setTool(tool: BBTool): Promise<boolean>;
};
const defaultCoreState = (): CoreState => ({
    isInit: false,
    cursor: 'Default',
    currentTool: 'Select',
});

export const createCoreStore = (): BBStore<CoreState, CoreApi> => {
    const [store, set] = createStore(defaultCoreState());

    const handleMsgEffect = (msg: JsApiEffectMsg) =>
        match(msg)
            .with({ tag: 'SetCurrentTool' }, ({ value }) =>
                set('currentTool', value),
            )
            .with({ tag: 'SetCursor' }, ({ value }) => set('cursor', value));

    const api: CoreApi = {
        async initEditor() {
            await coreManager.start(`#${CANVAS_ID}`);
            coreManager.addEffectCallback(handleMsgEffect);
        },

        async setTool(tool: BBTool) {
            const response = await coreManager.setTool(tool);
            const success = response.some((r) => r.tag === 'Success');
            return success;
        },
    };

    return [store, api];
};
