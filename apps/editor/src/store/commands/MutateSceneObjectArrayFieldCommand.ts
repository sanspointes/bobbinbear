import { produce, SetStoreFunction } from "solid-js/store";
import { EmbBase } from "../../emb-objects/shared";
import { getObjectSetter, SceneModel } from "../sceneStore";
import {
  AbstractCommand,
  assertSameField,
  assertSameType,
  SerializedCommand,
} from "./shared";
import { Command } from ".";
import { Uuid } from "../../utils/uuid";
import { PickOfType } from "../../types/utility";
import { arrayInsertCircular } from "../../utils/array";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type MutateSceneObjectArrayFieldCommandOptions<T extends any[]> = {
  toDelete: number;
  toInsert: T;
  circularInsert?: boolean;
};
/**
 * Sets a single field on a scene object.
 */
export class MutateSceneObjectArrayFieldCommand<
  TObject extends EmbBase = EmbBase,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  TObjectPicked extends PickOfType<TObject, any[]> = PickOfType<
    TObject,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    any[]
  >,
  K extends keyof TObjectPicked = keyof TObjectPicked,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  KV extends TObjectPicked[K] & any[] = TObjectPicked[K] & any[],
> extends AbstractCommand {
  public updatable: boolean = true;

  name = "Mutate Scene Object Field";
  type = "MutateSceneObjectArrayFieldCommand" as const;
  oldValue: TObjectPicked[K] | undefined = undefined;
  constructor(
    private objectId: Uuid<TObject>,
    private field: K,
    private index: number,
    private opts: MutateSceneObjectArrayFieldCommandOptions<KV>,
  ) {
    super();
    this.name =
      `Mutating array "${field.toString()}" on ${objectId}.  Index ${index}, deleting ${opts.toDelete} and inserting ${opts.toInsert.length}`;
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
    set(produce((obj) => {
      // @ts-expect-error; Complicated typescript
      const field = obj[this.field] as Array<unknown>;
      if (this.opts.circularInsert) {
        field.splice(this.index, this.opts.toDelete);
        arrayInsertCircular(field, this.index, ...this.opts.toInsert);
      } else {
        field.splice(this.index, this.opts.toDelete, ...this.opts.toInsert);
      }
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
    // TODO: Implement
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
