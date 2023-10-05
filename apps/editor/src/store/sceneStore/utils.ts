import { EmbObject, EmbState, EmbStatePersistable } from '@/emb-objects';
import { SceneModel } from '.';

function extractPersistableFields(
    object: EmbObject & EmbState,
): EmbObject & EmbStatePersistable {
    const { disableMove: _1, inspecting: _2, hovered: _3, ...data } = object;
    return data as EmbObject & EmbStatePersistable;
}

type ObjectsMap = Record<string, EmbObject & EmbStatePersistable>;
export type SceneStoreSerialisable = {
    objects: ObjectsMap;
    selectedIds: SceneModel['selectedIds'];
};

export const SceneStoreUtils = {
    toSerialisable(store: SceneModel) {
        const result: SceneStoreSerialisable = {
            selectedIds: [...store.selectedIds],
            objects: {},
        };

        for (const obj of store.objects.values()) {
            result.objects[obj.id] = extractPersistableFields(obj);
        }

        return result;
    },
};
