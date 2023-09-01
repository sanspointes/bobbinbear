import { createStore, produce, SetStoreFunction } from "solid-js/store";
import { getObjectSetter, SceneModel } from "../sceneStore";
import { type Command, type CommandPrototypeMap } from ".";
import { EmbBase } from "../../emb-objects/shared";
import { Uuid } from "../../utils/uuid";
import { arrayRemove } from "../../utils/array";
import { batch } from "solid-js";

export type SerializedCommand<TCommand extends Command> = {
  type: TCommand["type"];
  name: string;
  final: boolean;
  updatable: boolean;
} & TCommand;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type SideEffectExecuter<T extends any[]> = (
  store: SceneModel,
  setStore: SetStoreFunction<SceneModel>,
  ...args: T
) => void;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export class SideEffectsManager<T extends any[]> {
  private sideEffects: SideEffectExecuter<T>[] = [];

  public register(executor: SideEffectExecuter<T>) {
    this.sideEffects.push(executor);
  }

  public execute(store: SceneModel, set: SetStoreFunction<SceneModel>, ...args: T) {
    for (let i = 0; i < this.sideEffects.length; i++) {
      const effect = this.sideEffects[i]!;
      effect(store, set, ...args);
    }
  }
}

/**
 * Base class of all commands.
 */
export abstract class AbstractCommand {
  public final = true;
  public abstract updatable: boolean;

  public abstract readonly name: string;
  public abstract readonly type: string;

  public error: Error | undefined;

  constructor() {
  }

  /*
   * Side effects.  Allows components to hook into and extend behaviour of commands. 
   * To be run in each command execution / undo.
   */
  abstract perform(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
  ): void;
  abstract undo(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
  ): void;

  toObject(object: Record<string, unknown>) {
    object["type"] = this.type;
    object["name"] = this.name;
  }
  static fromObject(
    prototypeMap: CommandPrototypeMap,
    object: SerializedCommand<Command>,
  ): CommandPrototypeMap[typeof object["type"]] {
    const prototype = prototypeMap[object.type];
    if (!prototype) {
      throw new Error(
        `AbstractCommand: fromObject() Attempting to get prototype for ${object.type} but none found.`,
      );
    }
    const cmd = Object.create(prototype) as Command;
    cmd.type = object.type;
    cmd.name = object.name as Command["name"];
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

export class MultiCommand<TObject extends EmbBase> extends AbstractCommand {
  public updatable: boolean = true;
  name = "Multi Command";
  type: "MultiCommand" | string = "MultiCommand";
  commands: Command<TObject>[];
  constructor(...commands: Command<TObject>[]) {
    super();

    this.commands = commands;
  }

  perform(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
  ): void {
    batch(() => {
      for (const cmd of this.commands) {
        cmd.perform(store, setStore);
      }
    });
  }

  undo(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
  ): void {
    batch(() => {
      for (let i = this.commands.length - 1; i >= 0; i--) {
        const cmd = this.commands[i];
        if (!cmd) {
          throw new Error(
            `MultiCommand: (undo) Cannot get command at the ${i}th index.`,
          );
        }
        cmd.undo(store, setStore);
      }
    });
  }

  fromObject<T extends Command>(object: SerializedCommand<T>): void {
    let i = 0;
    for (const cmd of this.commands) {
      const cmdObject = object[i] as Record<string, unknown>;
      cmd.fromObject(cmdObject as SerializedCommand<T>);
      i++;
    }
  }

  toObject(object: Record<string, unknown>): void {
    let i = 0;
    for (const cmd of this.commands) {
      const cmdObject: Record<string, unknown> = {};
      cmd.toObject(cmdObject);
      object[i] = cmdObject;
      i++;
    }
  }

  updateData(newer: Command<TObject>): void {
    const n = assertSameType(this, newer) as MultiCommand<TObject>;
    // assertSameField(this, newer, 'length');
    let i = 0;
    for (const cmd of this.commands) {
      const newerCmd = n.commands[i]!;
      const n2 = assertSameType(cmd, newerCmd) as Command<TObject>;
      if (cmd.updateData) cmd.updateData(n2);
      i += 1;
    }
  }
}

/** Helpers **/

export const traverse = <T extends EmbBase>(
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

export type InsertPosition = "first" | "last";
/**
 * Adds object and children to store
 */
export const addObject = (
  store: SceneModel,
  _1: SetStoreFunction<SceneModel>,
  newObjectData: EmbBase,
  insertPosition: InsertPosition = "last",
) => {
  const objMap = store.objects;
  if (objMap.has(newObjectData.id)) {
    const set = getObjectSetter(store, newObjectData.id);
    if (!set) {
      throw new Error(
        `addObject: Attempted to get ${newObjectData.id} but no setter in store.`,
      );
    }
    set(newObjectData);
  } else {
    // Add all children to store
    traverse(store, newObjectData, (obj) => {
      const [object, setObject] = createStore(obj);
      store.objects.set(object.id, object);
      store.objectSetters.set(object.id, setObject);
    });

    const object = store.objects.get(newObjectData.id);
    // Attach to parent or add to root
    if (object?.parent) {
      const set = getObjectSetter(store, newObjectData.parent);
      if (set) {
        set(produce((parent) => {
          if (insertPosition === "first") {
            parent.children.splice(0, 0, object.id);
          } else if (insertPosition === "last") {
            parent.children.push(object.id);
          }
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
  id: Uuid<EmbBase>,
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

type AssertionInfo = string | { type: string };
export const assertSameType = <
  TObject1 extends Command,
  TObject2 extends Command,
>(self: TObject1, other: TObject2): TObject1 => {
  if (self.type === other.type) {
    return other as unknown as TObject1;
  }
  throw new Error(`${self.type}: Other is of different type ${other.type}.`);
};

export const assertSameField = <TCommand extends Command>(
  self: TCommand,
  other: TCommand,
  field: keyof TCommand,
) => {
  if (self[field] !== other[field]) {
    throw new Error(
      `${self.type}: Expected field '${field.toString()}' to be equal. Expected ${
        self[field]
      }, found ${other[field]}`,
    );
  }
};

export const assertDefined = <TValue>(
  self: AssertionInfo,
  value: TValue | undefined,
  valueName: string,
): value is TValue => {
  if (value !== undefined) return true;
  throw new Error(
    `${
      typeof self === "string" ? self : self.type
    }: Failed checking that "${valueName}" variable was not undefined.`,
  );
};
