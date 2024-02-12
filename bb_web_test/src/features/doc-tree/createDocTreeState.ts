import { Api } from 'bb_core';
import { createSignal } from 'solid-js';

export function createDocTreeState(api: Api) {
    const [data, setData] = createSignal([]);

    async function refresh() {
        const objects = await api.scene.list_objects();
        setData(objects);
    }

    return [data, { refresh }] as const;
}
