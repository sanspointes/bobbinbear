import { SetStoreFunction, produce } from 'solid-js/store';
import { BaseSceneObject } from "../../types/scene";
import { SceneModel, getObject, getObjectSetter } from "../sceneStore";
import { AbstractCommand, SerializedCommand, assertNotUndefined, assertSameType } from "./shared";
import { Command } from '.';
import { Uuid } from '../../utils/uuid';
import { batch } from 'solid-js';
import { arrayRemoveEl } from '../../utils/array';

export class DeselectObjectsCommand<TObject extends BaseSceneObject> extends AbstractCommand {
  public updatable: boolean = true;

  name = "Deselect Objects";
  type = "DeselectObjectsCommand" as const;

  toDeselect: Uuid<TObject>[] = [];
  toSelect: Uuid<TObject>[] = [];

  constructor(...objectIds: Uuid<TObject>[]) {
    super();
    this.toDeselect = objectIds;
  }
  perform(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
  ): void {
    batch(() => {
      for (const id of this.toDeselect) {
        const object = getObject(store, id);
        if (assertNotUndefined(this, object, "object")) {
          if (object.selected) this.toSelect.push(id);
          const set = getObjectSetter<BaseSceneObject>(store, object)!;
          set('selected', false);
          setStore(produce((store) => arrayRemoveEl(store.selectedIds, id)));
        }
      }
    });
  }

  undo(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
  ): void {
    batch(() => {
      for (const id of this.toDeselect) {
        const object = getObject(store, id);
        if (assertNotUndefined(this, object, "object")) {
          if (object.selected) this.toDeselect.push(id);
          const set = getObjectSetter<BaseSceneObject>(store, object)!;
          set('selected', true);
          setStore(produce((store) => arrayRemoveEl(store.selectedIds, id)));
        }
      }
    });
  }

  fromObject<T extends Command>(object: SerializedCommand<T>): void {
    this.toDeselect = object['toSelect'] as Uuid<TObject>[];
  }

  toObject(object: Record<string, unknown>): void {
    object['toSelect'] = this.toDeselect;
  }

  updateData(newer: Command): void {
    const n = assertSameType(this, newer);
    this.toDeselect = n.toSelect;
  }
}

