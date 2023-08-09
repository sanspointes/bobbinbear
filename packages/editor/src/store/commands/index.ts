import { AbstractCommand, SerializedCommand } from "./shared";
import {
  CreateObjectCommand,
  DeleteObjectCommand,
  MoveObjectCommand,
  SceneCommands,
  SelectObjectsCommand,
} from "./object";
import { SetStoreFunction } from "solid-js/store";
import { SceneObject } from "../../types/scene";
import { Uuid } from "../../utils/uuid";
import { ObjectMapData, SceneModel } from "../sceneStore";

export { CreateObjectCommand, DeleteObjectCommand, MoveObjectCommand };

export type AtomicCommands = SceneCommands;

export type Command = MultiCommand | AtomicCommands;
export type CommandType = Command["type"];

export type CommandPrototypeMap = Record<CommandType, AbstractCommand>;
export const _commandPrototypeMap: Record<CommandType, AbstractCommand> = {
  "CreateObjectCommand": CreateObjectCommand.prototype,
  "DeleteObjectCommand": DeleteObjectCommand.prototype,
  "MoveObjectCommand": MoveObjectCommand.prototype,
  "SelectObjectsCommand": SelectObjectsCommand.prototype,
};

export class MultiCommand extends AbstractCommand {
  name = "Multi Command";
  type = "MultiCommand";
  commands: Command[];
  constructor(...commands: Command[]) {
    super();

    this.commands = commands;
  }

  perform(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    for (const cmd of this.commands) {
      cmd.perform(store, setStore, objMap);
    }
  }

  undo(
    store: SceneModel,
    setStore: SetStoreFunction<SceneModel>,
    objMap: Map<Uuid<SceneObject>, ObjectMapData>,
  ): void {
    for (var i = this.commands.length - 1; i >= 0; i--) {
      const cmd = this.commands[i];
      cmd.undo(store, setStore, objMap);
    }
  }

  fromObject<T extends string>(object: SerializedCommand<T>): void {
    let i = 0;
    for (const cmd of this.commands) {
      const cmdObject = object[i] as Record<string, unknown>;
      cmd.fromObject(cmdObject as SerializedCommand<AtomicCommands['type']>);
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
}
