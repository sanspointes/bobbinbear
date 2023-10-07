import { produce, SetStoreFunction } from 'solid-js/store';
import { SceneModel } from '../sceneStore';
import {
    AbstractCommand,
    addObject,
    assertSameType,
    deleteObject,
    InsertPosition,
} from './shared';
import { batch } from 'solid-js';
import { arrayRemoveEl } from '../../utils/array';
import { Command } from '.';
import { EmbObject } from '@/emb-objects';

export class CreateObjectCommand<
    TObject extends EmbObject,
> extends AbstractCommand {
    public updatable: boolean = false;

    name = 'Create Object';
    type = 'CreateObjectCommand' as const;
    constructor(
        private object: TObject,
        private insertPosition: InsertPosition = 'last',
    ) {
        super();
    }

    perform(store: SceneModel, setStore: SetStoreFunction<SceneModel>): void {
        addObject(store, setStore, this.object, this.insertPosition);
    }
    undo(store: SceneModel, setStore: SetStoreFunction<SceneModel>): void {
        batch(() => {
            if (store.selectedIds.includes(this.object.id)) {
                setStore(
                    produce((store) =>
                        arrayRemoveEl(store.selectedIds, this.object.id),
                    ),
                );
            }
            const success = deleteObject(store, setStore, this.object.id);
            if (!success) {
                console.warn(
                    `CreateObjectCommand (undo) failed to delete ${this.object.id}`,
                );
            }
        });
    }

    updateData(newer: Command): void {
        const n = assertSameType(this, newer);
        this.object = n.object;
    }
}
