import { SetStoreFunction } from "solid-js/store";
import { ObjectMapData, SceneModel } from "../sceneStore";
import { Uuid } from "../../utils/uuid";
import { SceneObject } from "../../types/scene";
import { type Command, type CommandPrototypeMap, type CommandType } from ".";
export * from './object';

export type SerializedCommand<T extends CommandType> = {
  type: T,
  name: string,
  final: boolean,
} & Record<string, unknown>;

export abstract class AbstractCommand {
  public final = true;

  public readonly abstract name: string;
  public readonly abstract type: string;
  constructor() {

  }

  abstract perform(store: SceneModel, setStore: SetStoreFunction<SceneModel>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void;
  abstract undo(store: SceneModel, setStore: SetStoreFunction<SceneModel>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void;

  toObject(object: Record<string, unknown>) {
    object.type = this.type;
    object.name = this.name;
  }
  static fromObject<T extends CommandType>(prototypeMap: CommandPrototypeMap, object: SerializedCommand<T>): CommandPrototypeMap[T] {
    const cmd = Object.create(prototypeMap[object.type]) as Command;
    cmd.type = object.type;
    cmd.name = object.name as Command['name'];
    cmd.final = object.final;
    cmd.fromObject(object);

    return cmd;
  }
  protected abstract fromObject<T extends CommandType>(object: SerializedCommand<T>): void;

  updateData?(newer: AbstractCommand): void;

  getName(): string {
    return this.name;
  }

  getType(): string {
    return this.type;
  }
}
