import { AbstractCommand, MultiCommand } from './shared';
import { ParentObjectCommand } from './ParentObjectCommand';
import { CreateObjectCommand } from './CreateObjectCommand';
import { DeleteObjectCommand } from './DeleteObjectCommand';
import { DeselectObjectsCommand } from './DeselectObjectCommand';
import { MoveObjectCommand } from './MoveObjectCommand';
import { SelectObjectsCommand } from './SelectObjectCommand';
import { SetEmbObjectFieldCommand } from './SetSceneObjectFieldCommand';
import { MutateSceneObjectArrayFieldCommand } from './MutateSceneObjectArrayFieldCommand';
import { SetInspectingCommand } from './SetInspectingCommand';
import { EmbObject } from '@/emb-objects';

export {
    ParentObjectCommand as ChangeObjectOrderCommand,
    CreateObjectCommand,
    DeleteObjectCommand,
    DeselectObjectsCommand,
    MoveObjectCommand,
    SelectObjectsCommand,
    SetEmbObjectFieldCommand as SetSceneObjectFieldCommand,
    MutateSceneObjectArrayFieldCommand,
};

type AtomicCommands<TObject extends EmbObject> =
    | ParentObjectCommand
    | CreateObjectCommand<TObject>
    | DeleteObjectCommand<TObject>
    | DeselectObjectsCommand
    | MoveObjectCommand
    | SelectObjectsCommand
    | SetEmbObjectFieldCommand<TObject>
    | MutateSceneObjectArrayFieldCommand<TObject>
    | SetInspectingCommand;

export type Command<TObject extends EmbObject = EmbObject> =
    | MultiCommand<TObject>
    | AtomicCommands<TObject>;
export type CommandType = Command['type'];

export type CommandPrototypeMap = Record<CommandType, AbstractCommand>;

export const _commandPrototypeMap: Record<CommandType, Command> = {
    CreateObjectCommand: CreateObjectCommand.prototype,
    DeleteObjectCommand: DeleteObjectCommand.prototype,
    MoveObjectCommand: MoveObjectCommand.prototype,
    SelectObjectsCommand: SelectObjectsCommand.prototype,
    SetSceneObjectFieldCommand: SetEmbObjectFieldCommand.prototype,
    MultiCommand: MultiCommand.prototype,
    DeselectObjectsCommand: DeselectObjectsCommand.prototype,
    ParentObjectCommand: ParentObjectCommand.prototype,
    SetInspectingCommand: SetInspectingCommand.prototype,
};
