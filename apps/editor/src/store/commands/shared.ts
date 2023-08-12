import { SetStoreFunction, createStore, produce } from "solid-js/store";
import { SceneModel, getObjectSetter } from "../sceneStore";
import { type Command, type CommandPrototypeMap, type CommandType } from ".";
import { BaseSceneObject, SceneObject } from "../../types/scene";
import { Uuid } from "../../utils/uuid";
import { arrayRemove } from "../../utils/array";

export type SerializedCommand<TCommand extends Command> = {
  type: TCommand['type'],
  name: string,
  final: boolean,
  updatable: boolean,
} & Record<string, unknown>;

export abstract class AbstractCommand {
  public final = true;
  public abstract updatable: boolean;

  public readonly abstract name: string;
  public readonly abstract type: string;
  constructor() {

  }

  abstract perform(store: SceneModel, setStore: SetStoreFunction<SceneModel>): void;
  abstract undo(store: SceneModel, setStore: SetStoreFunction<SceneModel>): void;

  toObject(object: Record<string, unknown>) {
    object["type"] = this.type;
    object["name"] = this.name;
  }
  static fromObject(prototypeMap: CommandPrototypeMap, object: SerializedCommand<Command>): CommandPrototypeMap[typeof object['type']] {
    const prototype = prototypeMap[object.type];
    if (!prototype) throw new Error(`AbstractCommand: fromObject() Attempting to get prototype for ${object.type} but none found.`)
    const cmd = Object.create(prototype) as Command;
    cmd.type = object.type;
    cmd.name = object.name as Command['name'];
    cmd.final = object.final;
    cmd.fromObject(object);

    return cmd;
  }
  abstract fromObject<T extends Command>(object: SerializedCommand<T>): void;

  updateData?<T extends Command = Command>(newer: T): void;

  getName(): string {
    return this.name;
  }

  getType(): string {
    return this.type;
  }
}

/** Helpers **/

export const traverse = <T extends BaseSceneObject>(
  store: SceneModel,
  obj: T,
  handler: (obj: T) => void,
) => {
  handler(obj);
  if (obj.children) {
    for (const child of obj.children) {
      const obj = store.objects.get(child) as T;
      if (obj) traverse(store, obj, handler);
    }
  }
};

/**
 * Adds object and children to store
 */
export const addObject = (
  store: SceneModel,
  _1: SetStoreFunction<SceneModel>,
  newObjectData: BaseSceneObject,
) => {
  const objMap = store.objects;
  if (objMap.has(newObjectData.id)) {
    const set = getObjectSetter(store, newObjectData.id);
    if (!set) throw new Error(`addObject: Attempted to get ${newObjectData.id} but no setter in store.`)
    set(newObjectData);
  } else {
    // Add all children to store
    traverse(store, newObjectData, (obj) => {
      const [object, setObject] = createStore(obj);
      store.objects.set(object.id, object);
      store.objectSetters.set(object, setObject);
    });

    const object = store.objects.get(newObjectData.id);
    // Attach to parent or add to root
    if (object?.parent) {
      const set = getObjectSetter(store, newObjectData.parent);
      if (set) {
        set(produce((parent) => {
          parent.children.push(object.id);
        }));
      }
    }
  }
};

/**
 * Deletes object and children from store
 */
export const deleteObject = (
  store: SceneModel,
  _1: SetStoreFunction<SceneModel>,
  id: Uuid<BaseSceneObject>,
): boolean => {
  const obj = store.objects.get(id);
  if (!obj) return false;

  traverse(store, obj, (obj) => {
    store.objects.delete(obj.id);
  });
  if (obj.parent) {
    const setParent = getObjectSetter(store, obj.parent);
    if (setParent) {
      setParent(
        produce((parent) =>
          arrayRemove(parent.children, (id) => id === obj.id)
        ),
      );
      return true;
    }
  }
  return false;
};

export const assertSameType = <TObject1 extends Command, TObject2 extends Command>(self: TObject1, other: TObject2): TObject1 => {
  if (self.type === other.type) {
    return other as unknown as TObject1;
  }
  throw new Error(`${self.type}: Other is of different type ${other.type}.`);
}

export const assertSameField = <TCommand extends Command>(self: TCommand, other: TCommand, field: keyof TCommand) => {
  if (self[field] !== other[field]) throw new Error(`${self.type}: Expected field '${field.toString()}' to be equal. Expected ${self[field]}, found ${other[field]}`);
}
