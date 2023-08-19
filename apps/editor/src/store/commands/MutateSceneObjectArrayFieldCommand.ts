import { produce, SetStoreFunction } from "solid-js/store";
import { BaseSceneObject } from "../../types/scene";
import { getObjectSetter, SceneModel } from "../sceneStore";
import {
  AbstractCommand,
  assertSameField,
  assertSameType,
  SerializedCommand,
} from "./shared";
import { Command } from ".";
import { Uuid } from "../../utils/uuid";
import { KeysWithType, PickOfType } from "../../types/utility";

/**
 * Sets a single field on a scene object.
 */
export class MutateSceneObjectArrayFieldCommand<
  TObject extends BaseSceneObject = BaseSceneObject,
  TObjectPicked extends PickOfType<TObject, Array<unknown>> = PickOfType<TObject, Array<unknown>>,
  K extends KeysWithType<TObject, Array<unknown>> = KeysWithType<TObject, Array<unknown>>,
> extends AbstractCommand {
  public updatable: boolean = true;

  name = "Mutate Scene Object Field";
  type = "MutateSceneObjectArrayFieldCommand" as const;
  oldValue: TObjectPicked[K] | undefined = undefined;
  constructor(
    private objectId: Uuid<TObject>,
    private field: K,
    private index: number,
    private toDelete: number,
    private toInsert: TObjectPicked[K],
  ) {
    super();
    this.name = `Mutating array "${field.toString()}" on ${objectId}.  Index ${index}, deleting ${toDelete} and inserting ${toInsert.length}`;
  }

  perform(
    store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
  ): void {
    // console.debug(`SetSceneObjectFieldCommand: ${this.objectId}.${this.field.toString()} to ${this.value}`);
    const object = store.objects.get(this.objectId) as TObject | undefined;
    if (!object) {
      throw new Error(
        `SetSceneObjectFieldCommand: Can not get object for ${this.objectId}`,
      );
    }
    const set = getObjectSetter(store, object.id);
    if (!set) {
      throw new Error(
        `SetSceneObjectFieldCommand: Can not get object setter for ${this.objectId}`,
      );
    }
    set(produce(obj => {
      // @ts-expect-error; Complicated typescript
      const field = obj[this.field] as Array<unknown>;
      // @ts-expect-error; Complicated typescript
      field.splice(this.index, this.toDelete, ...this.toInsert)
    }));
    // set(this.field, this.value);
  }
  undo(
    store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
  ): void {
    // console.debug(`SetSceneObjectFieldCommand: (undo) ${this.objectId}.${this.field.toString()} to ${this.oldValue}`);
    const object = store.objects.get(this.objectId) as TObject | undefined;
    if (!object) {
      throw new Error(
        `SetSceneObjectFieldCommand: (undo) Can not get object ${this.objectId}`,
      );
    }
    const set = getObjectSetter(store, object.id);
    if (!set) {
      throw new Error(
        `SetSceneObjectFieldCommand: (undo) Can not get object setter for ${this.objectId}`,
      );
    }
    // @ts-expect-error; Complicated typescript
    set(this.field, this.oldValue);
  }

  toObject(object: Record<string, unknown>): void {
    super.toObject(object);
    object["objectId"] = this.objectId;
    object["field"] = this.field;
    object["value"] = this.value;
  }
  fromObject<T extends Command>(object: SerializedCommand<T>): void {
    this.objectId = object["objectId"] as Uuid<TObject>;
    this.field = object["field"] as K;
    this.value = object["value"] as TObject[K];
  }

  updateData(newer: Command): void {
    // @ts-expect-error; Difficult to resolve typing
    const n = assertSameType(this, newer) as typeof this;
    // @ts-expect-error; Difficult to resolve typing
    assertSameField(this, n, "field");
    this.value = n.value;
  }
}
