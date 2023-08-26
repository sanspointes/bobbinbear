import { produce, SetStoreFunction } from "solid-js/store";
import { BaseSceneObject } from "../../types/scene";
import { Uuid } from "../../utils/uuid";
import { SceneModel } from "../sceneStore";
import { AbstractCommand, assertDefined, assertSameType, SerializedCommand } from "./shared";
import { Command } from ".";

export class SetInspectingCommand extends AbstractCommand {
  public updatable: boolean = false;
  public name: string;
  public type: string = "InspectObjectCommand" as const;

  private oldValue: Uuid<BaseSceneObject> | undefined = undefined;
  constructor(private objectId: Uuid<BaseSceneObject> | undefined) {
    super();
    this.name = `Inspect ${objectId}`;
  }

  perform(store: SceneModel, set: SetStoreFunction<SceneModel>) {
    if (this.objectId) {
      const obj = store.objects.get(this.objectId);
      if (!assertDefined(this, obj, "object to inspect")) return;
    }
    if (!this.oldValue) this.oldValue = store.inspecting;
    set(produce((obj) => {
      obj.inspecting = this.objectId;
    }));
  }

  undo(store: SceneModel, set: SetStoreFunction<SceneModel>) {
    if (this.objectId) {
      const obj = store.objects.get(this.objectId);
      if (!assertDefined(this, obj, "object to inspect")) return;
    }
    set(produce((scene) => {
      scene.inspecting = this.oldValue;
    }));
  }

  fromObject<T extends Command>(object: SerializedCommand<T>): void {
    const other = assertSameType(this, object);
    this.objectId = other['objectId'];
    this.oldValue = other['oldValue'];
  }

  toObject(object: Record<string, unknown>): void {
    object['objectId'] = this.objectId;
    object['oldValue'] = this.oldValue;
  }
}
