import { SetStoreFunction, produce } from 'solid-js/store';
import { BaseSceneObject } from "../../types/scene";
import { SceneModel, getObject, getObjectSetter } from "../sceneStore";
import { AbstractCommand, SerializedCommand, assertNotUndefined, assertSameType } from "./shared";
import { Command } from '.';
import { Uuid } from '../../utils/uuid';
import { batch } from 'solid-js';
import { arrayRemove, arrayRemoveEl } from '../../utils/array';

export class SelectObjectsCommand<TObject extends BaseSceneObject> extends AbstractCommand {
  public updatable: boolean = true;

  name = "Select Objects";
  type = 'SelectObjectsCommand' as const;

  toSelect: Uuid<TObject>[] = [];
  toDeselect: Uuid<TObject>[] = [];


  constructor(...objectIds: Uuid<TObject>[]) {
    super();
    this.toSelect = objectIds;
    this.name = `Select ${objectIds.join(', ')}`
  }
  perform(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
  ): void {
    batch(() => {
      this.toDeselect = [];
      for (const id of this.toSelect) {
        const object = getObject(store, id);
        if (assertNotUndefined(this, object, "object")) {
          if (object.selected) this.toDeselect.push(id);
          const set = getObjectSetter<BaseSceneObject>(store, id)!;
          set('selected', true);
        }
        setStore(produce((store) => {
          store.selectedIds.push(id)
          if (object) store.selectedObjects.push(object);
        }));
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
        if (assertNotUndefined(this, object, "object")) {
          const set = getObjectSetter<BaseSceneObject>(store, object.id)!;
          set('selected', true);
        }
        setStore(produce((store) => {
          arrayRemoveEl(store.selectedIds, id)
          arrayRemove(store.selectedObjects, o => o.id === id)
        }));
      }
    });
  }

  fromObject(object: SerializedCommand<SelectObjectsCommand<TObject>>): void {
    this.toSelect = object['toSelect'];
  }

  toObject(object: Record<string, unknown>): void {
    object['toSelect'] = this.toSelect;
  }

  updateData(newer: Command): void {
    const n = assertSameType(this, newer);
    this.toSelect = n.toSelect;
  }
}

