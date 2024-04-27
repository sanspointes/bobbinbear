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
            const results = effect.value.map((uid) =>
                sceneApi
                    .describe_object(uid)
                    .then((obj) => [uid, obj] as const),
            );

            Promise.allSettled(results).then((results) => {
                batch(() => {
                    for (const result of results) {
                        if (result.status === 'rejected') continue;

                        const [uid, obj] = result.value;
                        if (obj) objects.set(uid, obj);
                    }
                });
            });
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
    window.receiveRustEvents = (effects: Effect[]) => {
        batch(() => {
            console.debug(`Received ${effects.length} effects to handle.`);
            for (const eff of effects) {
                handleEffect(eff);
            }
        });
    };

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
        return selectedApi.deselect_all_set_object_selected(uid, 'Selected');
    };
    const deleteObject = (uid: string) => {
        return sceneApi.delete(uid);
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
        deleteObject,

        selectSingle,
    };
}
