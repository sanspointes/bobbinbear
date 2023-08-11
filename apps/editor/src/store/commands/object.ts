/* eslint-disable solid/reactivity */
import { Point } from "@pixi/core";
import { createStore, produce, SetStoreFunction } from "solid-js/store";

import { Uuid } from "../../utils/uuid";
import {
  arrayMoveElToIndex,
  arrayRemove,
  arrayRemoveEl,
} from "../../utils/array";
import { CommandType } from "../commands";
import { BaseSceneObject, GraphicSceneObject, SceneObject } from "../../types/scene";
import { ObjectMapData, SceneModel } from "../sceneStore";
import { AbstractCommand, SerializedCommand } from "./shared";
import { batch } from "solid-js";

/**
 * HELPERS
 */
export const traverse = (
  obj: SceneObject,
  handler: (obj: SceneObject) => void,
) => {
  handler(obj);
  if (obj.children) {
    for (const child of obj.children) {
      traverse(child, handler);
    }
  }
};

/**
 * Adds object and children to store
 */
const addObject = (
  _1: SetStoreFunction<SceneModel>,
  objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  newObjectData: SceneObject,
) => {
  if (objMap.has(newObjectData.id)) {
    const data = objMap.get(newObjectData.id) as ObjectMapData;
    data.set(newObjectData);
  } else {
    // Add all children to store
    traverse(newObjectData, (obj) => {
      const [object, setObject] = createStore(obj);
      objMap.set(object.id, {
        object,
        set: setObject,
      });
    });

    const object = objMap.get(newObjectData.id)!.object;
    // Attach to parent or add to root
    if (newObjectData.parent) {
      const parentStore = objMap.get(newObjectData.parent);
      if (parentStore) {
        parentStore.set(produce((parent) => {
          parent.children.push(object);
        }));
      }
    }
  }
};

/**
 * Deletes object and children from store
 */
const deleteObject = (
  _1: SetStoreFunction<SceneModel>,
  objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  id: Uuid<SceneObject>,
): boolean => {
  const result = objMap.get(id);
  if (!result) return false;

  const { object } = result;
  traverse(object, (obj) => {
    objMap.delete(obj.id);
  });
  if (object.parent) {
    const parentStore = objMap.get(object.parent);
    if (parentStore) {
      parentStore.set(
        produce((parent) =>
          arrayRemove(parent.children, ({ id }) => id === object.id)
        ),
      );
      return true;
    }
  }
  return false;
};

export class CreateObjectCommand extends AbstractCommand {
  public updatable: boolean = false;

  name = "Create Object";
  type = "CreateObjectCommand" as const;
  constructor(private object: SceneObject) {
    super();
  }

  perform(
    _store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    addObject(setStore, objMap, this.object);
  }
  undo(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    batch(() => {
      if (store.selectedIds.includes(this.object.id)) {
        setStore(
          produce((store) => arrayRemoveEl(store.selectedIds, this.object.id)),
        );
      }
      const success = deleteObject(setStore, objMap, this.object.id);
      if (!success) {
        console.warn(
          `CreateObjectCommand (undo) failed to delete ${this.object.id}`,
        );
      }
    });
  }

  toObject(object: Record<string, unknown>): void {
    super.toObject(object);
    object.object = JSON.stringify(this.object);
  }
  fromObject<T extends CommandType>(object: SerializedCommand<T>): void {
    this.object = JSON.parse(object.object as string) as SceneObject;
  }

  updateData(newer: CreateObjectCommand): void {
    this.object = newer.object;
  }
}

export class ChangeObjectOrderCommand extends AbstractCommand {
  public updatable: boolean = true;
  name = "Change Object Order";
  type = "ChangeObjectOrderCommand" as const;

  oldIndex: number | undefined;

  constructor(
    private objectId: Uuid<SceneObject>,
    private strategy: "first" | "last" | "offset" | "absolute",
    private index?: number,
  ) {
    super();
  }

  perform(
    _1: SceneModel,
    _2: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    const result = objMap.get(this.objectId);
    if (!result) {
      throw new Error(
        `getObjectSiblings: Could not get object ${this.objectId} to change order of.`,
      );
    }
    const { object } = result;
    const parentResult = objMap.get(object.parent);
    if (!parentResult) {
      throw new Error(
        `getObjectSiblings: Could not get parent (${result.object.parent}) of object ${this.objectId} to change order of.`,
      );
    }
    const siblings = parentResult.object.children;
    this.oldIndex = siblings.findIndex((el) => el.id === this.objectId);
    if (this.oldIndex < 0) {
      throw new Error(
        `ChangeObjectOrderCommand: Could not get current index of ${this.objectId}`,
      );
    }

    let targetIndex: number | undefined;
    if (this.strategy === "absolute") {
      if (!this.index) {
        throw new Error(
          `ChangeObjectOrderCommand: Move strategy is 'absolute' but no index provided for ${this.objectId}.`,
        );
      }
      targetIndex = this.index;
    } else if (this.strategy === "offset") {
      if (!this.index) {
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

    parentResult.set(produce((parent) => {
      if (targetIndex === undefined) {
        throw new Error(
          "ChangeObjectOrderCommand: Unknown error getting target index to move child to. ",
        );
      }
      arrayMoveElToIndex(parent.children, object, targetIndex);
    }));
  }

  undo(
    _1: SceneModel,
    _2: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    const result = objMap.get(this.objectId);
    if (!result) {
      throw new Error(
        `getObjectSiblings: Could not get object ${this.objectId} to change order of.`,
      );
    }
    const { object } = result;
    const parentResult = objMap.get(object.parent);
    if (!parentResult) {
      throw new Error(
        `getObjectSiblings: Could not get parent (${result.object.parent}) of object ${this.objectId} to change order of.`,
      );
    }
    parentResult.set(produce((parent) => {
      if (this.oldIndex === undefined) {
        throw new Error(
          "ChangeObjectOrderCommand: (undo) Unknown error getting old index to move child to. ",
        );
      }
      arrayMoveElToIndex(parent.children, object, this.oldIndex);
    }));
  }

  protected fromObject<T extends CommandType>(
    object: SerializedCommand<T>,
  ): void {
    this.objectId = object.objectId as Uuid<SceneObject>;
    this.oldIndex = object.oldIndex as number | undefined;
    this.strategy = object.strategy as "first" | "last" | "offset" | "absolute";
    this.index = object.index as number | undefined;
  }

  toObject(object: Record<string, unknown>): void {
    object.objectId = this.objectId as Uuid<SceneObject>;
    object.oldIndex = this.oldIndex as number | undefined;
    object.strategy = this.strategy as "first" | "last" | "offset" | "absolute";
    object.index = this.index as number | undefined;
  }
}

export class SetSceneObjectFieldCommand<
  TObj extends BaseSceneObject = BaseSceneObject,
  K extends keyof TObj = keyof TObj,
> extends AbstractCommand {
  public updatable: boolean = true;

  name = "Set Scene Object Field";
  type = "SetSceneObjectFieldCommand" as const;
  oldValue: TObj[K] | undefined = undefined;
  constructor(
    private objectId: Uuid<SceneObject>,
    private field: K,
    private value: TObj[K],
  ) {
    super();
  }

  perform(
    _store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    // console.debug(`SetSceneObjectFieldCommand: ${this.objectId}.${this.field.toString()} to ${this.value}`);
    const result = objMap.get(this.objectId);
    if (!result) {
      throw new Error(
        `SetSceneObjectFieldCommand: Can not get object ${this.objectId}`,
      );
    }
    this.oldValue = result.object[this.field as keyof SceneObject] as TObj[K];
    // @ts-ignore-error; Complicated typescript
    result.set(this.field, this.value);
  }
  undo(
    _store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    // console.debug(`SetSceneObjectFieldCommand: (undo) ${this.objectId}.${this.field.toString()} to ${this.oldValue}`);
    const result = objMap.get(this.objectId);
    if (!result) {
      throw new Error(
        `SetSceneObjectFieldCommand: (undo) Can not get object ${this.objectId}`,
      );
    }
    // @ts-ignore-error; Complicated typescript
    result.set(this.field, this.oldValue);
  }

  toObject(object: Record<string, unknown>): void {
    super.toObject(object);
    object.objectId = this.objectId;
    object.field = this.field;
    object.value = this.value;
  }
  fromObject<T extends CommandType>(object: SerializedCommand<T>): void {
    this.objectId = object.objectId as Uuid<SceneObject>;
    this.field = object.field as K;
    this.value = object.value as TObj[K];
  }

  updateData(newer: SetSceneObjectFieldCommand<TObj, keyof TObj>): void {
    if (newer.field !== this.field) {
      throw new Error(
        "SetSceneObjectFieldCommand.updateData() Can't update command to use new field.  Make sure field matches previous commands",
      );
    }
    // @ts-ignore-error ; Complicated typescript
    this.value = newer.value;
  }
}

export class DeleteObjectCommand extends AbstractCommand {
  public updatable: boolean = false;

  name = "Delete Object";
  type = "DeleteObjectCommand" as const;
  constructor(private object: SceneObject) {
    super();
  }

  perform(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    batch(() => {
      if (store.selectedIds.includes(this.object.id)) {
        setStore(
          produce((store) => arrayRemoveEl(store.selectedIds, this.object.id)),
        );
      }
      const success = deleteObject(setStore, objMap, this.object.id);
      if (!success) {
        console.warn(`DeleteObjectCommand failed to delete ${this.object.id}`);
      }
    });
  }
  undo(
    _store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    addObject(setStore, objMap, this.object);
  }

  toObject(object: Record<string, unknown>): void {
    super.toObject(object);
    object.object = JSON.stringify(this.object);
  }
  fromObject<T extends CommandType>(object: SerializedCommand<T>): void {
    this.object = JSON.parse(object.object as string) as SceneObject;
  }
}

export class MoveObjectCommand extends AbstractCommand {
  public updatable: boolean = true;

  name = "Move Object";
  type = "MoveObjectCommand" as const;

  oldPosition?: Point;

  constructor(private objectId: Uuid<SceneObject>, private newPosition: Point) {
    super();
  }
  perform(
    _store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    const result = objMap.get(this.objectId);
    if (!result) {
      throw new Error(
        `MoveObjectCommand: Could not get object ${this.objectId} to move`,
      );
    }

    if (!this.oldPosition) this.oldPosition = result.object.position;

    // If moving a node, update the mesh of the graphic.
    if (result.object.type === 'node' && result.object.relatesTo) {
      const currentNode = result.object.node;
      const graphicResult = objMap.get(result.object.relatesTo);
      if (!graphicResult) throw new Error('MoveObjectCommand: Attempting to graphic related to moved node but no graphic found.')
      const { set: setGraphics } = graphicResult;

      const graphicObject = graphicResult.object as GraphicSceneObject;
      const oldIndex = graphicObject.shape.findIndex(node => node.id === currentNode.id);
      console.log(oldIndex);
      setGraphics(produce(obj => {
        const graphic = obj as GraphicSceneObject;

        graphic.shape.splice(oldIndex, 1, {
          ...currentNode,
          x: this.newPosition.x,
          y: this.newPosition.y,
        });
      }));

    }

    result.set(produce((object) => object.position = this.newPosition));
  }

  undo(
    _store: SceneModel,
    _setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    const result = objMap.get(this.objectId);
    if (!result) {
      throw new Error(
        `MoveObjectCommand (undo): Could not get object ${this.objectId} to move`,
      );
    }
    if (!this.oldPosition) {
      throw new Error(
        `MoveObjectCommand (undo): Could not get old position of ${this.objectId} to move`,
      );
    }

    result.set(produce((object) => object.position = this.oldPosition!));
  }

  fromObject<T extends CommandType>(object: SerializedCommand<T>): void {
    this.objectId = object.objectId as Uuid<SceneObject>;
    this.oldPosition = object.oldPosition as Point | undefined;
  }

  toObject(object: Record<string, unknown>): void {
    object.objectId = this.objectId;
    object.oldPosition = this.oldPosition;
  }

  updateData(newer: MoveObjectCommand): void {
    this.newPosition = newer.newPosition;
  }
}

export class SelectObjectsCommand extends AbstractCommand {
  public updatable: boolean = true;

  name = "Select Objects";
  type = "SelectObjectsCommand" as const;

  toSelect: Uuid<SceneObject>[] = [];
  toDeselect: Uuid<SceneObject>[] = [];

  constructor(...objectIds: Uuid<SceneObject>[]) {
    super();
    this.toSelect = objectIds;
  }
  perform(
    _store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    batch(() => {
      for (const id of this.toSelect) {
        const result = objMap.get(id);
        if (result) {
          if (result.object.selected) this.toDeselect.push(id);
          result.set("selected", true);
        }
        setStore(produce((store) => store.selectedIds.push(id)));
      }
    });
  }

  undo(
    _store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    batch(() => {
      for (const id of this.toDeselect) {
        const result = objMap.get(id);
        if (result) {
          result.set("selected", false);
        }
        setStore(produce((store) => arrayRemoveEl(store.selectedIds, id)));
      }
    });
  }

  fromObject<T extends CommandType>(object: SerializedCommand<T>): void {
    this.toSelect = object.toSelect as Uuid<SceneObject>[];
  }

  toObject(object: Record<string, unknown>): void {
    object.toSelect = this.toSelect;
  }

  updateData(newer: SelectObjectsCommand): void {
    this.toSelect = newer.toSelect;
  }
}

export class DeselectObjectsCommand extends AbstractCommand {
  public updatable: boolean = true;

  name = "Deselect Objects";
  type = "DeselectObjectsCommand" as const;

  toSelect: Uuid<SceneObject>[] = [];
  toDeselect: Uuid<SceneObject>[] = [];

  constructor(...objectIds: Uuid<SceneObject>[]) {
    super();
    this.toSelect = objectIds;
  }
  perform(
    _store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    batch(() => {
      for (const id of this.toSelect) {
        const result = objMap.get(id);
        if (result) {
          if (result.object.selected) this.toDeselect.push(id);
          result.set("selected", false);
        }
        setStore(produce((store) => arrayRemoveEl(store.selectedIds, id)));
      }
    });
  }

  undo(
    _store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    batch(() => {
      for (const id of this.toDeselect) {
        const result = objMap.get(id);
        if (result) {
          result.set("selected", true);
        }
        setStore(produce((store) => store.selectedIds.push(id)));
      }
    });
  }

  fromObject<T extends CommandType>(object: SerializedCommand<T>): void {
    this.toSelect = object.toSelect as Uuid<SceneObject>[];
  }

  toObject(object: Record<string, unknown>): void {
    object.toSelect = this.toSelect;
  }

  updateData(newer: SelectObjectsCommand): void {
    this.toSelect = newer.toSelect;
  }
}

export type SceneCommands =
  | CreateObjectCommand
  | DeleteObjectCommand
  | SetSceneObjectFieldCommand
  | ChangeObjectOrderCommand
  | MoveObjectCommand
  | SelectObjectsCommand
  | DeselectObjectsCommand;
