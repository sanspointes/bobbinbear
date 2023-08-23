import { Point } from '@pixi/core';
import { SetStoreFunction, produce } from 'solid-js/store';
import { BaseSceneObject, GraphicSceneObject, NodeSceneObject } from "../../types/scene";
import { SceneModel, getObject, getObjectSetter } from "../sceneStore";
import { AbstractCommand, SerializedCommand, assertSameType } from "./shared";
import { Command } from '.';
import { Uuid } from '../../utils/uuid';

export class MoveObjectCommand<TObject extends BaseSceneObject> extends AbstractCommand {
  public updatable: boolean = true;

  name = "Move Object";
  type = "MoveObjectCommand" as const;

  oldPosition?: Point;

  constructor(private objectId: Uuid<TObject>, private newPosition: Point) {
    super();
  }
  perform(
    store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
  ): void {
    const object = getObject(store, this.objectId);
    if (!object) {
      throw new Error(
        `MoveObjectCommand: Could not get object ${this.objectId} to move`,
      );
    }

    if (!this.oldPosition) this.oldPosition = object.position;

    // If moving a node, update the mesh of the graphic.
    const nodeObject = object as unknown as NodeSceneObject;
    if (nodeObject.type === 'node' && nodeObject.relatesTo) {
      const currentNode = nodeObject.node;
      const graphicObject = getObject(store, nodeObject.relatesTo);
      if (!graphicObject) throw new Error('MoveObjectCommand: Attempting to graphic related to moved node but no graphic found.')

      const oldIndex = graphicObject.shape.findIndex(node => node.id === currentNode.id);
      const setGraphics = getObjectSetter(store, nodeObject.relatesTo)!;
      setGraphics(produce(obj => {
        const graphic = obj as GraphicSceneObject;

        graphic.shape.splice(oldIndex, 1, {
          ...currentNode,
          x: this.newPosition.x,
          y: this.newPosition.y,
        });
      }));
    }

    // Update node position 
    const set = getObjectSetter(store, this.objectId)!;
    set(produce((object) => {
      object.position = this.newPosition.clone();
    }));
  }

  undo(
    store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
  ): void {
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

    set(produce((object) => object.position = this.oldPosition!.clone()));
  }

  fromObject<T extends Command>(object: SerializedCommand<T>): void {
    this.objectId = object['objectId'] as Uuid<TObject>;
    this.oldPosition = object['oldPosition'] as Point | undefined;
  }

  toObject(object: Record<string, unknown>): void {
    object['objectId'] = this.objectId;
    object['oldPosition'] = this.oldPosition;
  }

  updateData(newer: Command): void {
    const n = assertSameType(this, newer);
    this.newPosition = n.newPosition;
  }
}
