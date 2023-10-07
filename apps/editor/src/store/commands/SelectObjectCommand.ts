import { SetStoreFunction, produce } from 'solid-js/store';
import { SceneModel, getObject, getObjectSetter } from '../sceneStore';
import { AbstractCommand, assertDefined, assertSameType } from './shared';
import { Command } from '.';
import { Uuid } from '../../utils/uuid';
import { batch } from 'solid-js';
import { arrayRemove, arrayRemoveEl } from '../../utils/array';
import { EmbObject } from '@/emb-objects';

export class SelectObjectsCommand extends AbstractCommand {
    public updatable: boolean = true;

    name = 'Select Objects';
    type = 'SelectObjectsCommand' as const;

    toSelect: Uuid[] = [];
    toDeselect: Uuid[] = [];

    constructor(...objectIds: Uuid[]) {
        super();
        this.toSelect = objectIds;
        this.name = `Select ${objectIds.join(', ')}`;
    }
    perform(store: SceneModel, setStore: SetStoreFunction<SceneModel>): void {
        batch(() => {
            this.toDeselect = [];
            for (const id of this.toSelect) {
                const object = getObject(store, id);
                if (assertDefined(this, object, 'object')) {
                    if (object.selected) this.toDeselect.push(id);
                    const set = getObjectSetter<EmbObject>(store, id)!;
                    set('selected', true);
                }
                setStore(
                    produce((store) => {
                        store.selectedIds.push(id);
                        if (object) store.selectedObjects.push(object);
                    }),
                );
            }
        });
    }

    undo(store: SceneModel, setStore: SetStoreFunction<SceneModel>): void {
        batch(() => {
            for (const id of this.toDeselect) {
                this.toSelect = [];
                const object = getObject(store, id);
                if (assertDefined(this, object, 'object')) {
                    const set = getObjectSetter<EmbObject>(store, object.id)!;
                    set('selected', true);
                }
                setStore(
                    produce((store) => {
                        arrayRemoveEl(store.selectedIds, id);
                        arrayRemove(store.selectedObjects, (o) => o.id === id);
                    }),
                );
            }
        });
    }
    //
    // fromObject(object: SerializedCommand<SelectObjectsCommand<TObject>>): void {
    //     this.toSelect = object['toSelect'];
    // }
    //
    // toObject(object: Record<string, unknown>): void {
    //     object['toSelect'] = this.toSelect;
    // }

    updateData(newer: Command): void {
        const n = assertSameType(this, newer);
        this.toSelect = n.toSelect;
    }
}
