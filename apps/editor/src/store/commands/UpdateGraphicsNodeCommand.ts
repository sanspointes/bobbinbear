import { produce, SetStoreFunction } from "solid-js/store";
import {
  EmbVector,
  VectorNode,
  EmbObject,
} from "../../types/scene";
import { Uuid } from "../../utils/uuid";
import { getObject, getObjectSetter, SceneModel } from "../sceneStore";
import { AbstractCommand, assertSameField, assertSameType, SerializedCommand } from "./shared";
import { Command } from ".";

export class UpdateGraphicsNodeCommand extends AbstractCommand {
  public updatable: boolean = true;

  name = "Update Graphics Node";
  type = "UpdateGraphicsNodeCommand" as const;

  oldData: VectorNode | undefined;
  constructor(
    public objectId: Uuid<EmbVector>,
    public index: number,
    public node: VectorNode,
  ) {
    super();
  }
  perform(
    store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
  ): void {
    const object = getObject(store, this.objectId);

    if (!object) {
      throw new Error(
        `UpdateGraphicsNodeCommand: Provided object id (${this.objectId}) is not found.`,
      );
    } else if (object.type !== "graphic") {
      throw new Error(
        `UpdateGraphicsNodeCommand: Provided object is not a graphic.  Instead found ${object.type}.`,
      );
    }
    const set = getObjectSetter(store, object.id)!;

    this.oldData = (object as EmbVector).shape[this.index];
    set(produce((object) => {
      const obj = object as EmbVector;
      obj.shape.splice(this.index, 1, this.node);
    }));
  }

  undo(
    store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
  ): void {
    const object = store.objects.get(this.objectId) as EmbVector | undefined;

    if (!object) {
      throw new Error(
        `UpdateGraphicsNodeCommand: Provided object id (${this.objectId}) is not found.`,
      );
    } else if (object.type !== "graphic") {
      throw new Error(
        `UpdateGraphicsNodeCommand: Provided object is not a graphic.  Instead found ${object.type}.`,
      );
    }
    const set = getObjectSetter(store, object.id)!;

    set(produce((object) => {
      const obj = object as EmbVector;
      obj.shape.splice(this.index, 1, this.node);
    }));
  }

  fromObject<T extends Command>(object: SerializedCommand<T>): void {
    this.objectId = object["objectId"] as Uuid<EmbVector>;
    this.index = object["index"] as number;
    this.node = object["node"] as VectorNode;
  }

  toObject(object: Record<string, unknown>): void {
    object["objectId"] = this.objectId as Uuid<EmbObject>;
    object["index"] = this.index as number;
    object["node"] = this.node as VectorNode;
  }

  updateData(newer: Command): void {
    const n = assertSameType(this, newer);
    assertSameField(this, n, 'objectId');
    assertSameField(this, n, 'index');
    this.node = n.node;
  }
}
