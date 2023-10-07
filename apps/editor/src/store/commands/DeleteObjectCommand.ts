import { SetStoreFunction, produce } from 'solid-js/store';
import { SceneModel, getObject } from '../sceneStore';
import {
    AbstractCommand,
    addObject,
    deleteObject,
} from './shared';
import { batch } from 'solid-js';
import { arrayRemoveEl } from '../../utils/array';
import { Uuid } from '../../utils/uuid';
import { EmbObject } from '@/emb-objects';

export class DeleteObjectCommand<
    TObject extends EmbObject,
> extends AbstractCommand {
    public updatable: boolean = false;

    name = 'Delete Object';
    type = 'DeleteObjectCommand' as const;
    constructor(private objectId: Uuid) {
        super();
    }

    deletedObject: TObject | undefined;

    perform(store: SceneModel, setStore: SetStoreFunction<SceneModel>): void {
        batch(() => {
            if (store.selectedIds.includes(this.objectId)) {
                setStore(
                    produce((store) =>
                        arrayRemoveEl(store.selectedIds, this.objectId),
                    ),
                );
            }
            const object = getObject(store, this.objectId);
            this.deletedObject = object as TObject;
            const success = deleteObject(store, setStore, this.objectId);
            if (!success) {
                console.warn(
                    `DeleteObjectCommand failed to delete ${this.objectId}`,
                );
            }
        });
    }
    undo(store: SceneModel, setStore: SetStoreFunction<SceneModel>): void {
        if (!this.deletedObject)
            throw new Error(
                'DeleteObjectCommand: (undo) No object to restore.',
            );
        addObject(store, setStore, this.deletedObject);
    }

    // toObject(object: Record<string, unknown>): void {
    //     super.toObject(object);
    //     object['objectId'] = this.objectId;
    //     object['deletedObject'] = this.deletedObject;
    // }
    // fromObject<T extends Command>(object: SerializedCommand<T>): void {
    //     this.objectId = object['objectId'] as Uuid;
    //     this.deletedObject = object['deleteObject'] as TObject;
    // }
}
