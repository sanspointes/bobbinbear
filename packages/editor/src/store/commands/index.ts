import { SetStoreFunction } from "solid-js/store";
import { ObjectMapData, SceneStoreModel } from "../sceneStore";
import { CreateObjectCommand, DeleteObjectCommand, MoveObjectCommand, SceneCommands } from "./object";
import { Uuid } from "../../utils/uuid";
import { SceneObject } from "../../types/scene";
export * from './object';

export type Command = SceneCommands;
export type CommandType = Command['type']

let commandPrototypeMap: Record<CommandType, AbstractCommand> = undefined as unknown as Record<CommandType, AbstractCommand>;
export const initialiseCommandPrototypeMap = () => {
commandPrototypeMap = {
  "CreateObjectCommand": CreateObjectCommand.prototype,
  "DeleteObjectCommand": DeleteObjectCommand.prototype,
  "MoveObjectCommand": MoveObjectCommand.prototype,
}
  }

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

  abstract perform(store: SceneStoreModel, setStore: SetStoreFunction<SceneStoreModel>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void;
  abstract undo(store: SceneStoreModel, setStore: SetStoreFunction<SceneStoreModel>, objMap: Map<Uuid<SceneObject>, ObjectMapData>): void;

  toObject(object: Record<string, unknown>) {
    object.type = this.type;
    object.name = this.name;
  }
  abstract fromObject<T extends CommandType>(object: SerializedCommand<T>): void;
  static fromObject<T extends CommandType>(object: SerializedCommand<T>): typeof commandPrototypeMap[T] {
    const cmd = Object.create(commandPrototypeMap[object.type]) as Command;
    cmd.type = object.type;
    cmd.name = object.name as Command['name'];
    cmd.final = object.final;
    cmd.fromObject(object);

    return cmd;
  }

  updateData?(newer: AbstractCommand): void;

  getName(): string {
    return this.name;
  }

  getType(): string {
    return this.type;
  }
}

