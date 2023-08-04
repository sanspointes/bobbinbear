/* eslint-disable solid/reactivity */
import { SetStoreFunction, createStore, produce } from "solid-js/store";
import { ObjectMapData, SceneObject, SceneStore, traverse } from ".";
import { Uuid } from "../../utils/uuid";
import { arrayRemove } from "../../utils/array";
import { AbstractCommand, CommandType, SerializedCommand } from "../commands";
import { Point } from "pixi.js";

/**
 * HELPERS
 */

/**
 * Adds object and children to store
 */
const addObject = (store: SceneStore, setStore: SetStoreFunction<SceneStore>, objMap: Map<Uuid<SceneObject>, ObjectMapData>, newObjectData: SceneObject) => {
  // Add all children to store
  traverse(newObjectData, (obj) => {
    const [object, setObject] = createStore(obj);
    objMap.set(object.id, {
      object,
      set: setObject,
    });
  })

  const object = objMap.get(newObjectData.id)!.object;
  // Attach to parent or add to root
  if (newObjectData.parent) {
    const parentStore = objMap.get(newObjectData.parent);
    if (parentStore) {
      parentStore.set(produce(parent => {
        parent.children.push(object);
      }))
    }
  } else {
    setStore(produce(store => store.root.push(object)));
  }
}

/**
 * Deletes object and children from store
 */
const deleteObject = (store: SceneStore, setStore: SetStoreFunction<SceneStore>, objMap: Map<Uuid<SceneObject>, ObjectMapData>, id: Uuid<SceneObject>): boolean => {
  const result = objMap.get(id);
  if (!result) return false;

  const { object } = result;
  traverse(object, (obj) => {
    objMap.delete(obj.id);
  });
  if (object.parent) {
    const parentStore = objMap.get(object.parent);
    if (parentStore) {
      parentStore.set(produce(parent => arrayRemove(parent.children, ({id}) => id === object.id)))
      return true;
    }
  } else {
    let success = false;
    setStore(produce(store => {
      success = arrayRemove(store.root, child => child.id === id)
    }))
    return success;
  }
  return false;
}

abstract class SceneCommand extends AbstractCommand {
  handler: string = 'scene' as const;

  constructor() {
    super()
  }

  abstract perform(store: SceneStore, setStore: SetStoreFunction<SceneStore>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void;
  abstract undo(store: SceneStore, setStore: SetStoreFunction<SceneStore>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void;
}

export class CreateObjectCommand extends SceneCommand {
  name = 'Create Object' as const;
  type = 'CreateObjectCommand' as const;
  constructor(private object: SceneObject) {
    super();
  }

  perform(store: SceneStore, setStore: SetStoreFunction<SceneStore>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void {
    addObject(store, setStore, objMap, this.object);
  }
  undo(store: SceneStore, setStore: SetStoreFunction<SceneStore>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void {
    const success = deleteObject(store, setStore, objMap, this.object.id);
    if (!success) {
      console.warn(`CreateObjectCommand (undo) failed to delete ${this.object.id}`);
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

export class DeleteObjectCommand extends SceneCommand {
  name = 'Delete Object' as const;
  type = 'DeleteObjectCommand' as const;
  constructor(private object: SceneObject) {
    super();
  }

  perform(store: SceneStore, setStore: SetStoreFunction<SceneStore>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void {
    const success = deleteObject(store, setStore, objMap, this.object.id);
    if (!success) {
      console.warn(`DeleteObjectCommand failed to delete ${this.object.id}`);
    }
  }
  undo(store: SceneStore, setStore: SetStoreFunction<SceneStore>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void {
    addObject(store, setStore, objMap, this.object);
  }

  toObject(object: Record<string, unknown>): void {
    super.toObject(object);
    object.object = JSON.stringify(this.object);
  }
  fromObject<T extends CommandType>(object: SerializedCommand<T>): void {
    this.object = JSON.parse(object.object as string) as SceneObject;
  }
}

export class MoveObjectCommand extends SceneCommand {
  name = 'Move Object' as const;
  type = 'MoveObjectCommand' as const;

  oldPosition?: Point;

  constructor(private objectId: Uuid<SceneObject>, private newPosition: Point) {
    super();
  }
  perform(store: SceneStore, setStore: SetStoreFunction<SceneStore>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void {
    const result = objMap.get(this.objectId);
    if (!result) throw new Error(`MoveObjectCommand: Could not get object ${this.objectId} to move`);

    if (!this.oldPosition) this.oldPosition = result.object.position;

    result.set(produce(object => object.position = this.newPosition));
  }

  undo(store: SceneStore, setStore: SetStoreFunction<SceneStore>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void {
    const result = objMap.get(this.objectId);
    if (!result) throw new Error(`MoveObjectCommand (undo): Could not get object ${this.objectId} to move`);
    if (!this.oldPosition) throw new Error(`MoveObjectCommand (undo): Could not get old position of ${this.objectId} to move`);

    result.set(produce(object => object.position = this.oldPosition!));
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

export type SceneCommands = CreateObjectCommand | DeleteObjectCommand | MoveObjectCommand;
