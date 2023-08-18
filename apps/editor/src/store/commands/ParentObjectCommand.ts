import { SetStoreFunction, produce } from 'solid-js/store';
import { SceneModel, getObjectSetter } from "../sceneStore";
import { AbstractCommand, SerializedCommand, assertNotUndefined } from "./shared";
import { Command } from '.';
import { Uuid } from '../../utils/uuid';
import { BaseSceneObject, SceneObject } from '../../types/scene';
import { arrayMoveElToIndex, arrayRemoveEl } from '../../utils/array';


export class ParentObjectCommand<TObject extends BaseSceneObject> extends AbstractCommand {
  public updatable: boolean = true;
  name = "Change Object Order";
  type = "ChangeObjectOrderCommand" as const;

  oldParentId: Uuid<BaseSceneObject> | undefined;
  oldIndex: number | undefined;

  constructor(
    private objectId: Uuid<TObject>,
    private newParentId: Uuid<BaseSceneObject>,
    private strategy: "first" | "last" | "offset" | "absolute",
    private index?: number,
  ) {
    super();
  }

  perform(
    store: SceneModel,
    _2: SetStoreFunction<SceneModel>,
  ): void {
    const object = store.objects.get(this.objectId);
    if (!assertNotUndefined(this, object, 'object')) return;
    this.oldParentId = object.parent;

    let parentobject = store.objects.get(object.parent);
    if (!assertNotUndefined(this, parentobject, 'parentobject')) return;

    const siblings = parentobject.children;
    this.oldIndex = siblings.findIndex((id) => id === this.objectId);
    if (this.oldIndex < 0) {
      throw new Error(
        `ChangeObjectOrderCommand: Could not get current index of ${this.objectId}`,
      );
    }

    // Unparent old parent if different
    if (this.newParentId !== this.oldParentId) {
      const setParent = store.objectSetters.get(parentobject);
      if (assertNotUndefined(this, setParent, 'setParent')) {
        setParent(produce(parent => arrayRemoveEl(parent.children, object.id)));
      }
      parentobject = store.objects.get(this.newParentId);
    }
    if (!assertNotUndefined(this, parentobject, 'parentobject (for new parent)')) return;

    let targetIndex: number | undefined;
    if (this.strategy === "absolute") {
      if (this.index === undefined) {
        throw new Error(
          `ChangeObjectOrderCommand: Move strategy is 'absolute' but no index provided for ${this.objectId}.`,
        );
      }
      targetIndex = this.index;
    } else if (this.strategy === "offset") {
      if (this.index === undefined) {
        throw new Error(
          `ChangeObjectOrderCommand: Move strategy is 'offset' but no offset provided for ${this.objectId}.`,
        );
      }
      targetIndex = this.oldIndex + this.index;
    } else if (this.strategy === "first") {
      targetIndex = 0;
    } else if (this.strategy === "last") {
      targetIndex = siblings.length - 1;
    }

    const parentSetter = getObjectSetter(store, parentobject);

    if (!parentSetter) throw new Error(`ChangeObjectOrderCommand: Could not get parent setter for ${parentobject.id}.`);
    parentSetter(produce((parent) => {
      if (targetIndex === undefined) {
        throw new Error(
          "ChangeObjectOrderCommand: Unknown error getting target index to move child to. ",
        );
      }
      arrayMoveElToIndex(parent.children, object.id, targetIndex);
    }));
  }

  undo(
    store: SceneModel,
    _2: SetStoreFunction<SceneModel>,
  ): void {
    const object = store.objects.get(this.objectId);
    if (!object) {
      throw new Error(
        `ParentObjectCommand: Could not get object ${this.objectId} to change order of.`,
      );
    }
    const parentobject = store.objects.get(object.parent);
    if (!parentobject) {
      throw new Error(
        `ParentObjectCommand: Could not get parent (${object.parent}) of object ${this.objectId} to change order of.`,
      );
    }
    const parentSetter = getObjectSetter(store, parentobject);

    if (!parentSetter) throw new Error(`ChangeObjectOrderCommand: (undo) Could not get parent setter for ${parentobject.id}.`);
    parentSetter(produce((parent) => {
      if (this.oldIndex === undefined) {
        throw new Error(
          "ChangeObjectOrderCommand: (undo) Unknown error getting old index to move child to. ",
        );
      }
      arrayMoveElToIndex(parent.children, object.id, this.oldIndex);
    }));
  }

  fromObject<T extends Command>(
    object: SerializedCommand<T>,
  ): void {
    this.objectId = object['objectId'] as Uuid<TObject>;
    this.oldIndex = object['oldIndex'] as number | undefined;
    this.strategy = object['strategy'] as "first" | "last" | "offset" | "absolute";
    this.index = object['index'] as number | undefined;
  }

  toObject(object: Record<string, unknown>): void {
    object['objectId'] = this.objectId as Uuid<TObject>;
    object['oldIndex'] = this.oldIndex as number | undefined;
    object['strategy'] = this.strategy as "first" | "last" | "offset" | "absolute";
    object['index'] = this.index as number | undefined;
  }
}
