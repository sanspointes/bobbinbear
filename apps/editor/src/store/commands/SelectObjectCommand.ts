import { SetStoreFunction, produce } from 'solid-js/store';
import { BaseSceneObject } from "../../types/scene";
import { SceneModel, getObject, getObjectSetter } from "../sceneStore";
import { AbstractCommand, SerializedCommand, assertSameType } from "./shared";
import { Command } from '.';
import { Uuid } from '../../utils/uuid';
import { batch } from 'solid-js';
import { arrayRemoveEl } from '../../utils/array';

export class SelectObjectsCommand<TObject extends BaseSceneObject> extends AbstractCommand {
  public updatable: boolean = true;

  name = "Select Objects";
  type = "SelectObjectsCommand" as const;

  toSelect: Uuid<TObject>[] = [];
  toDeselect: Uuid<TObject>[] = [];

  constructor(...objectIds: Uuid<TObject>[]) {
    super();
    this.toSelect = objectIds;
  }
  perform(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
  ): void {
    batch(() => {
      this.toDeselect = [];
      for (const id of this.toSelect) {
        const object = getObject(store, id);
        if (object) {
          if (object.selected) this.toDeselect.push(id);
          const set = getObjectSetter<BaseSceneObject>(store, object)!;
          setStore(produce((store) => store.selectedIds.push(id)));
          set('selected', true);
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
        this.toSelect = []
        const object = getObject(store, id);
        if (object) {
          const set = getObjectSetter<BaseSceneObject>(store, object)!;
          setStore(produce((store) => arrayRemoveEl(store.selectedIds, id)));
          set('selected', true);
        }
      }
    });
  }

  fromObject<T extends Command>(object: SerializedCommand<T>): void {
    this.toSelect = object['toSelect'] as Uuid<TObject>[];
  }

  toObject(object: Record<string, unknown>): void {
    object['toSelect'] = this.toSelect;
  }

  updateData(newer: Command): void {
    const n = assertSameType(this, newer);
    this.toSelect = n.toSelect;
  }
}

