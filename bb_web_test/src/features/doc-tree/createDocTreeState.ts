import { DescribedObject, SceneApi, UidJs } from 'bb_core';
import { createContext, createSignal, useContext } from 'solid-js';

type DocTreeContextModel = ReturnType<typeof createDocTreeContext>;
export const DocTreeContext = createContext<DocTreeContextModel>(null!);

export function createDocTreeContext() {
    const api = new SceneApi();
    const [data, setData] = createSignal<DescribedObject[]>([]);

    async function refresh() {
        const objects = await api.describe_document();
        console.log(objects);
        setData(objects);
    }

    async function setVisible(uid: UidJs, visible: boolean) {
        await api.set_visible(uid, visible);
        await refresh();
    }

    return [data, { refresh, setVisible }] as const;
}

export function useDocTreeContext() {
    const ctx = useContext(DocTreeContext);
    if (!ctx) throw new Error('Must be used within a DocTreeContext.Provider.');
    return ctx;
}
