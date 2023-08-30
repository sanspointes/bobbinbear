import { SetStoreFunction, produce } from 'solid-js/store';
import { EmbBase } from "../../types/scene";
import { SceneModel, getObject, getObjectSetter } from "../sceneStore";
import { AbstractCommand, SerializedCommand, assertDefined, assertSameType } from "./shared";
import { Command } from '.';
import { Uuid } from '../../utils/uuid';
import { batch } from 'solid-js';
import { arrayRemove, arrayRemoveEl } from '../../utils/array';

export class DeselectObjectsCommand<TObject extends EmbBase> extends AbstractCommand {
  public updatable: boolean = true;

  name = "Deselect Objects";
  type = "DeselectObjectsCommand" as const;

  toDeselect: Uuid<TObject>[] = [];
  toSelect: Uuid<TObject>[] = [];

  constructor(...objectIds: Uuid<TObject>[]) {
    super();
    this.toDeselect = objectIds;
    this.name = `Deselect ${objectIds.join(', ')}`
  }
  perform(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
  ): void {
    batch(() => {
      for (const id of this.toDeselect) {
        const object = getObject(store, id);
        if (assertDefined(this, object, "object")) {
          if (object.selected) this.toSelect.push(id);
          const set = getObjectSetter<EmbBase>(store, id)!;
          set('selected', false);
          setStore(produce((store) => arrayRemoveEl(store.selectedIds, id)));
          setStore(produce((store) => {
            arrayRemoveEl(store.selectedIds, id);
            arrayRemove(store.selectedObjects, o => o.id === id);
          }));
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
        if (assertDefined(this, object, "object")) {
          if (object.selected) this.toDeselect.push(id);
          const set = getObjectSetter<EmbBase>(store, id)!;
          set('selected', true);
          setStore(produce((store) => {
            store.selectedIds.push(id);
            if (object) store.selectedObjects.push(object);
          }));
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

