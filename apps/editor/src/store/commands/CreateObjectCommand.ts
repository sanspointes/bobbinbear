import { produce, SetStoreFunction } from "solid-js/store";
import { EmbBase } from "../../emb-objects/shared";
import { SceneModel } from "../sceneStore";
import {
  AbstractCommand,
  addObject,
  assertSameType,
  deleteObject,
  InsertPosition,
  SerializedCommand,
} from "./shared";
import { batch } from "solid-js";
import { arrayRemoveEl } from "../../utils/array";
import { Command } from ".";

export class CreateObjectCommand<TObject extends EmbBase>
  extends AbstractCommand {
  public updatable: boolean = false;

  name = "Create Object";
  type = "CreateObjectCommand" as const;
  constructor(
    private object: TObject,
    private insertPosition: InsertPosition = "last",
  ) {
    super();
  }

  perform(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
  ): void {
    addObject(store, setStore, this.object, this.insertPosition);
  }
  undo(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
  ): void {
    batch(() => {
      if (store.selectedIds.includes(this.object.id)) {
        setStore(
          produce((store) => arrayRemoveEl(store.selectedIds, this.object.id)),
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

  toObject(object: Record<string, unknown>): void {
    super.toObject(object);
    object["object"] = JSON.stringify(this.object);
    object["insertPosition"] = this.insertPosition;
  }
  fromObject<T extends Command>(object: SerializedCommand<T>): void {
    this.object = JSON.parse(object["object"] as string) as TObject;
    this.insertPosition = object["insertPosition"] as InsertPosition;
  }

  updateData(newer: Command): void {
    const n = assertSameType(this, newer);
    this.object = n.object;
  }
}
