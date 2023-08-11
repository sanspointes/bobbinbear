import { produce, SetStoreFunction } from "solid-js/store";
import {
  GraphicSceneObject,
  GraphicsNode,
  SceneObject,
} from "../../types/scene";
import { Uuid } from "../../utils/uuid";
import { ObjectMapData, SceneModel } from "../sceneStore";
import { AbstractCommand, SerializedCommand } from "./shared";
import { CommandType } from ".";

export class UpdateGraphicsNodeCommand extends AbstractCommand {
  public updatable: boolean = true;

  name = "Update Graphics Node";
  type = "UpdateGraphicsNode" as const;

  oldData: GraphicsNode | undefined;
  constructor(
    private objectId: Uuid<SceneObject>,
    private index: number,
    private node: GraphicsNode,
  ) {
    super();
  }
  perform(
    _store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    const result = objMap.get(this.objectId);

    if (!result) {
      throw new Error(
        `UpdateGraphicsNodeCommand: Provided object id (${this.objectId}) is not found.`,
      );
    } else if (result.object.type !== "graphic") {
      throw new Error(
        `UpdateGraphicsNodeCommand: Provided object is not a graphic.  Instead found ${result.object.type}.`,
      );
    }

    this.oldData = (result.object as GraphicSceneObject).shape[this.index];
    result.set(produce((object) => {
      const obj = object as GraphicSceneObject;
      obj.shape.splice(this.index, 1, this.node);
    }));
  }

  undo(
    _store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    const result = objMap.get(this.objectId);

    if (!result) {
      throw new Error(
        `UpdateGraphicsNodeCommand: Provided object id (${this.objectId}) is not found.`,
      );
    } else if (result.object.type !== "graphic") {
      throw new Error(
        `UpdateGraphicsNodeCommand: Provided object is not a graphic.  Instead found ${result.object.type}.`,
      );
    }
    result.set(produce((object) => {
      const obj = object as GraphicSceneObject;
      obj.shape.splice(this.index, 1, this.node);
    }));
  }

  fromObject<T extends CommandType>(object: SerializedCommand<T>): void {
    this.objectId = object.objectId as Uuid<SceneObject>;
    this.index = object.index as number;
    this.node = object.node as GraphicsNode;
  }

  toObject(object: Record<string, unknown>): void {
    object.objectId = this.objectId as Uuid<SceneObject>;
    object.index = this.index as number;
    object.node = this.node as GraphicsNode;
  }

  updateData(newer: UpdateGraphicsNodeCommand): void {
    if (newer.objectId !== this.objectId) {
      throw new Error(
        `UpdateGraphicsNodeCommand: updateData 'objectId' is different.`,
      );
    }
    if (newer.index !== this.index) {
      throw new Error(
        `UpdateGraphicsNodeCommand: updateData 'index' is different.`,
      );
    }
    this.node = newer.node;
  }
}
