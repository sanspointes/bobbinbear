import { AbstractCommand, SerializedCommand } from "./shared";
import {
  CreateObjectCommand,
  DeleteObjectCommand,
  MoveObjectCommand,
  SceneCommands,
  SelectObjectsCommand,
  SetSceneObjectFieldCommand,
} from "./object";
import { SetStoreFunction } from "solid-js/store";
import { SceneObject } from "../../types/scene";
import { Uuid } from "../../utils/uuid";
import { ObjectMapData, SceneModel } from "../sceneStore";
import { UpdateGraphicsNodeCommand } from "./GraphicsNodesCommands";

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
  "SetSceneObjectFieldCommand": SetSceneObjectFieldCommand.prototype,
  "UpdateGraphicsNodeCommand": UpdateGraphicsNodeCommand.prototype,
};

export class MultiCommand extends AbstractCommand {
  public updatable: boolean = true;
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

  updateData(newer: MultiCommand): void {
    if (this.commands.length !== newer.commands.length) 
      throw new Error(`MultiCommand.updateData() Cant update MultiCommand as newer version has a different number of commands.  Expected ${this.commands.length}, found ${newer.commands.length}.`);
    let i = 0;
    for (const cmd of this.commands) {
      // @ts-ignore-error; Typescript pains.
      if (cmd.updateData) cmd.updateData(newer.commands[i]);
      i += 1;
    }
  }
}
