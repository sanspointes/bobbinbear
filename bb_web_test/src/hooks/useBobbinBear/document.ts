import {
    DescribedObject,
    DetailedObject,
    Effect,
    SceneApi,
    SelectedApi,
} from 'bb_core';
import { createSignal } from 'solid-js';

export type BBDocument = ReturnType<typeof useBBDocument>;

export function useBBDocument() {
    const sceneApi = new SceneApi();
    const selectedApi = new SelectedApi();
    const [objects, setObjects] = createSignal<DescribedObject[]>([]);
    const [selectedObject, setSelectedObject] = createSignal<
        DetailedObject | undefined
    >(undefined);

    async function refresh() {
        const objects = await sceneApi.describe_document();
        console.log(objects);
        setObjects(objects);
    }

    const handleEffect = (effect: Effect) => {
        if (effect.tag === 'DocumentChanged') {
            refresh();
        } else if (effect.tag === 'SelectionChanged') {
            const [first] = effect.value;
            console.log(first);
            sceneApi.describe_object(first).then((object) => {
                console.log(object);
                setSelectedObject(object);
            });
        }
    };
    // @ts-expect-error: untyped...
    window.receiveRustEvents = handleEffect;

    const setVisible = (uid: string, visible: boolean) => {
        return sceneApi.set_visible(uid, visible);
    };
    const setName = (uid: string, name: string) => {
        return sceneApi.set_name(uid, name);
    };
    const setPosition = (uid: string, x: number, y: number) => {
        return sceneApi.set_position(uid, x, y);
    };

    const selectSingle = (uid: string) => {
        return selectedApi.deselect_all_set_object_selected_js(uid, true);
    };

    return {
        objects,
        selectedObject,

        setVisible,
        setName,
        setPosition,

        selectSingle,
    };
}
