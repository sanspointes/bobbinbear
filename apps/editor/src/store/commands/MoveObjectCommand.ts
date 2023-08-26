import { Point } from '@pixi/core';
import { SetStoreFunction, produce } from 'solid-js/store';
import { BaseSceneObject, BasicGraphicsNode, GraphicNodeTypes, GraphicSceneObject, GraphicsNode, NodeSceneObject } from "../../types/scene";
import { SceneModel, getObject, getObjectSetter } from "../sceneStore";
import { AbstractCommand, SerializedCommand, assertSameType } from "./shared";
import { Command } from '.';
import { Uuid } from '../../utils/uuid';
import { arrayGetCircular, arraySetCircular } from '../../utils/array';

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

      const index = graphicObject.shape.findIndex(node => node.id === currentNode.id);
      const setGraphics = getObjectSetter(store, nodeObject.relatesTo)!;
      setGraphics(produce(obj => {
        const graphic = obj as GraphicSceneObject;
        if (currentNode.type === GraphicNodeTypes.Point || currentNode.type === GraphicNodeTypes.Jump) {
          MoveObjectCommand.handleMovePointNode(graphic, currentNode, index, this.newPosition);
        } else if (currentNode.type === GraphicNodeTypes.Control) {
          MoveObjectCommand.handleMoveControlNode(graphic, currentNode, index, this.newPosition)
        }
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

  static handleMoveControlNode(graphicObject: GraphicSceneObject, node: BasicGraphicsNode, index: number, newPosition: Point) {
    const diffx = newPosition.x - node.x;
    const diffy = newPosition.y - node.y;

    let lookForward = false;
    let owningNode: BasicGraphicsNode | undefined;
    const nextNode = arrayGetCircular<BasicGraphicsNode>(graphicObject.shape, index + 1);
    if (nextNode?.ownsPrev) {
      owningNode = nextNode;
      lookForward = true;
    } else {
      owningNode = arrayGetCircular<BasicGraphicsNode>(graphicObject.shape, index - 1);
    }
    const needsMoveControlNode = owningNode?.isControlPaired;

    if (needsMoveControlNode) {
      const otherIndex = lookForward ? index + 2 : index - 2;
      const otherNode = arrayGetCircular(graphicObject.shape, otherIndex);
      if (otherNode?.type === GraphicNodeTypes.Control) {
        arraySetCircular(graphicObject.shape, otherIndex, {
          ...otherNode,
          x: otherNode.x - diffx,
          y: otherNode.y - diffy,
        });
      } else {
        console.warn(`MoveObject: Attempted to move other control node but not found ${index} + ${lookForward ? 2 : -2}.`)
      }
    }

    graphicObject.shape.splice(index, 1, {
      ...node,
      x: newPosition.x,
      y: newPosition.y,
    });
  }

  static handleMovePointNode(graphicObject: GraphicSceneObject, node: BasicGraphicsNode, index: number, newPosition: Point) {
    const diffx = newPosition.x - node.x;
    const diffy = newPosition.y - node.y;

    if ((node as BasicGraphicsNode).ownsPrev) {
      const preNode = arrayGetCircular(graphicObject.shape, index - 1);
      if (preNode?.type === GraphicNodeTypes.Control) {
        arraySetCircular(graphicObject.shape, index - 1, {
          ...preNode,
          x: preNode.x + diffx,
          y: preNode.y + diffy,
        })
      }
    }

    if ((node as BasicGraphicsNode).ownsNext) {
      const nextNode = arrayGetCircular(graphicObject.shape, index + 1);
      if (nextNode?.type === GraphicNodeTypes.Control) {
        arraySetCircular(graphicObject.shape, index + 1, {
          ...nextNode,
          x: nextNode.x + diffx,
          y: nextNode.y + diffy,
        })
      }
    }

    graphicObject.shape.splice(index, 1, {
      ...node,
      x: newPosition.x,
      y: newPosition.y,
    });
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
