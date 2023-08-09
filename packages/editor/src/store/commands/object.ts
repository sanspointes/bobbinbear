/* eslint-disable solid/reactivity */
import { Point } from "@pixi/core";
import { createStore, produce, SetStoreFunction } from "solid-js/store";

import { Uuid } from "../../utils/uuid";
import { arrayRemove, arrayRemoveEl } from "../../utils/array";
import { CommandType } from "../commands";
import { SceneObject } from "../../types/scene";
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
  setStore: SetStoreFunction<SceneModel>,
  objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  newObjectData: SceneObject,
) => {
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
  } else {
    setStore(produce((store) => store.root.push(object)));
  }
};

/**
 * Deletes object and children from store
 */
const deleteObject = (
  setStore: SetStoreFunction<SceneModel>,
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
  } else {
    let success = false;
    setStore(produce((store) => {
      success = arrayRemove(store.root, (child) => child.id === id);
    }));
    return success;
  }
  return false;
};

export class CreateObjectCommand extends AbstractCommand {
  name = "Create Object" as const;
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
    _store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    const success = deleteObject(setStore, objMap, this.object.id);
    if (!success) {
      console.warn(
        `CreateObjectCommand (undo) failed to delete ${this.object.id}`,
      );
    }
  }

  toObject(object: Record<string, unknown>): void {
    super.toObject(object);
    object.object = JSON.stringify(this.object);
  }
  fromObject<T extends CommandType>(object: SerializedCommand<T>): void {
    this.object = JSON.parse(object.object as string) as SceneObject;
  }
}

export class DeleteObjectCommand extends AbstractCommand {
  name = "Delete Object" as const;
  type = "DeleteObjectCommand" as const;
  constructor(private object: SceneObject) {
    super();
  }

  perform(
    _store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    const success = deleteObject(setStore, objMap, this.object.id);
    if (!success) {
      console.warn(`DeleteObjectCommand failed to delete ${this.object.id}`);
    }
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
  name = "Move Object" as const;
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
  name = "Select Objects" as const;
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
        if (!result) {
          throw new Error(
            `DeselectObjectsCommand: Could not get object ${id} to select`,
          );
        }
        if (result.object.selected) this.toDeselect.push(id);
        result.set("selected", true);
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
        if (!result) {
          throw new Error(
            `DeselectObjectsCommand: (undo) Could not get object ${id} to select`,
          );
        }
        result.set("selected", false);
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
  name = "Deselect Objects" as const;
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
        if (!result) {
          throw new Error(
            `DeselectObjectsCommand: Could not get object ${id} to select`,
          );
        }
        if (result.object.selected) this.toDeselect.push(id);
        result.set("selected", false);
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
        if (!result) {
          throw new Error(
            `DeselectObjectsCommand: (undo) Could not get object ${id} to select`,
          );
        }
        result.set("selected", true);
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
  | MoveObjectCommand
  | SelectObjectsCommand
  | DeselectObjectsCommand
;
