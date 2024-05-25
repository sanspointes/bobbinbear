import { DetailedObject, Effect, SceneApi, SelectedApi } from 'bb_core';
import { batch, createMemo, createSignal, onCleanup, onMount } from 'solid-js';
import { createMutable } from 'solid-js/store';
import { ReactiveMap } from '@solid-primitives/map';
import { EffectEmitter, useEffectEmitter } from '~/utils/effect-emitter';

export type BBDocument = ReturnType<typeof useBBDocument>;

export function useBBDocument(effectEmitter: EffectEmitter) {
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


    const onEntityChanged = (uids: string[]) => {
        const results = uids.map((uid) =>
            sceneApi
                .describe_object(uid)
                .then((obj) => [uid, obj] as const),
        );

        Promise.allSettled(results).then((results) => {
            batch(() => {
                for (const result of results) {
                    if (result.status === 'rejected') continue;

                    const [uid, next] = result.value;

                    const curr = objects.get(uid)
                    if (next && curr) {
                        for (const k in next) {
                            if (k === 'uid') continue;
                            // @ts-expect-error: Untyped keyof DetailedObject
                            curr[k] = next[k];
                        }
                    } else if (next) {
                        objects.set(uid, createMutable(next));
                    } else if (curr) {
                        objects.delete(uid);
                    }
                }
            });
        });
    };

    useEffectEmitter(effectEmitter, 'EntitiesSpawned', onEntityChanged);
    useEffectEmitter(effectEmitter, 'EntitiesDespawned', onEntityChanged);
    useEffectEmitter(effectEmitter, 'EntitiesChanged', onEntityChanged);

    useEffectEmitter(effectEmitter, 'SelectionChanged', (selectedUids) => {
        const [first] = selectedUids;
        if (first) setSelectedObjectUid(first);
    });

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
    const hover = (uid: string) => {
        console.log(`document.hover(uid: ${uid})`);
        return selectedApi.set_object_hovered(uid, 'Hovered');
    };
    const unhover = (uid: string) => {
        console.log(`document.unhover(uid: ${uid})`);
        return selectedApi.set_object_hovered(uid, 'Unhovered');
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

        hover,
        unhover,
    };
}
