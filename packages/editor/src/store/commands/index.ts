import { AbstractCommand } from "./shared";
import { SceneCommands, CreateObjectCommand, DeleteObjectCommand, MoveObjectCommand } from './object';

export { CreateObjectCommand, DeleteObjectCommand, MoveObjectCommand };

export type Command = SceneCommands;
export type CommandType = Command['type']

export type CommandPrototypeMap = Record<CommandType, AbstractCommand>
export const _commandPrototypeMap: Record<CommandType, AbstractCommand> = {
  "CreateObjectCommand": CreateObjectCommand.prototype,
  "DeleteObjectCommand": DeleteObjectCommand.prototype,
  "MoveObjectCommand": MoveObjectCommand.prototype,
};

