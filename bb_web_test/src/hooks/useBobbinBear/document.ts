import { DetailedObject, Effect, SceneApi, SelectedApi } from 'bb_core';
import { batch, createMemo, createSignal } from 'solid-js';
import { ReactiveMap } from '@solid-primitives/map';

export type BBDocument = ReturnType<typeof useBBDocument>;

export function useBBDocument() {
    const sceneApi = new SceneApi();
    const selectedApi = new SelectedApi();

    const objects = new ReactiveMap<string, DetailedObject>();
    const [selectedObjectUid, setSelectedObjectUid] = createSignal<
        string | undefined
    >(undefined);
    const selectedObject = createMemo(() => {
        const uid = selectedObjectUid();
        if (uid) return objects.get(uid);
        else return undefined;
    });

    const handleEffect = (effect: Effect) => {
        console.log(effect);
        if (
            effect.tag === 'EntitiesSpawned' ||
            effect.tag === 'EntitiesChanged'
        ) {
            for (const uid of effect.value) {
                sceneApi.describe_object(uid).then((obj) => {
                    if (obj) objects.set(uid, obj);
                });
            }
        } else if (effect.tag === 'EntitiesDespawned') {
            for (const uid of effect.value) {
                objects.delete(uid);
            }
        } else if (effect.tag === 'SelectionChanged') {
            const [first] = effect.value;
            setSelectedObjectUid(first);
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
    const inspect = (uid: string) => {
        return sceneApi.inspect(uid);
    };
    const uninspect = () => {
        return sceneApi.uninspect();
    };

    const selectSingle = (uid: string) => {
        return selectedApi.deselect_all_set_object_selected_js(uid, true);
    };

    return {
        objects,
        selectedObjectUid,
        selectedObject,

        setVisible,
        setName,
        setPosition,
        inspect,
        uninspect,

        selectSingle,
    };
}
