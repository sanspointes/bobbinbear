import { Point } from '@pixi/core';
import { produce, SetStoreFunction } from 'solid-js/store';
import { getObject, getObjectSetter, SceneModel } from '../sceneStore';
import { AbstractCommand, assertSameType, SerializedCommand } from './shared';
import { Command } from '.';
import { Uuid } from '../../utils/uuid';
import { EmbNode } from '../../emb-objects';
import { isEmbNode } from '../../emb-objects/utils';

export class MoveObjectCommand extends AbstractCommand {
    public updatable: boolean = true;

    name = 'Move Object';
    type = 'MoveObjectCommand' as const;

    oldPosition?: Point;

    constructor(
        private objectId: Uuid,
        private newPosition: Point,
    ) {
        super();
    }
    perform(store: SceneModel, _setStore: SetStoreFunction<SceneModel>): void {
        const object = getObject(store, this.objectId);
        if (!object) {
            throw new Error(
                `MoveObjectCommand: Could not get object ${this.objectId} to move`,
            );
        }
        if (object.disableMove) {
            console.warn('MoveObjectCommand: Moving a non-movable object.');
        }

        if (!this.oldPosition) this.oldPosition = object.position;

        // If moving a node, update the mesh of the graphic.
        if (isEmbNode(object)) {
            MoveObjectCommand.handleMoveNode(store, object, this.newPosition);
        }

        // Update node position
        const set = getObjectSetter(store, this.objectId)!;
        set(
            produce((object) => {
                object.position = this.newPosition.clone();
            }),
        );
    }

    undo(store: SceneModel, _setStore: SetStoreFunction<SceneModel>): void {
        const object = store.objects.get(this.objectId);
        if (!object) {
            throw new Error(
                `MoveObjectCommand (undo): Could not get object ${this.objectId} to move`,
            );
        }
        if (!this.oldPosition) {
            throw new Error(
                `MoveObjectCommand (undo): Could not get old position of ${this.objectId} to move`,
            );
        }

        const set = getObjectSetter(store, this.objectId)!;

        set(produce((object) => (object.position = this.oldPosition!.clone())));
    }

    /**
     * Special logic for moving a node object
     */
    static handleMoveNode(
        store: SceneModel,
        object: EmbNode,
        newPosition: Point,
    ) {
        const setNode = getObjectSetter<EmbNode>(store, object.id);
        if (!setNode)
            throw new Error(
                `MoveObjectCommand.handleMoveNode:  Cannot get setter for ${object.id}.`,
            );
        setNode('node', 'x', newPosition.x);
        setNode('node', 'y', newPosition.y);
    }
    //
    // fromObject<T extends Command>(object: SerializedCommand<T>): void {
    //     this.objectId = object['objectId'] as Uuid;
    //     this.oldPosition = object['oldPosition'] as Point | undefined;
    // }
    //
    // toObject(object: Record<string, unknown>): void {
    //     object['objectId'] = this.objectId;
    //     object['oldPosition'] = this.oldPosition;
    // }

    updateData(newer: Command): void {
        const n = assertSameType(this, newer);
        this.newPosition = n.newPosition;
    }
}
