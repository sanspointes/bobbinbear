import { AbstractCommand, MultiCommand } from './shared';
import { EmbBase, EmbState } from '../../emb-objects/shared';
import { ParentObjectCommand } from './ParentObjectCommand';
import { CreateObjectCommand } from './CreateObjectCommand';
import { DeleteObjectCommand } from './DeleteObjectCommand';
import { DeselectObjectsCommand } from './DeselectObjectCommand';
import { MoveObjectCommand } from './MoveObjectCommand';
import { SelectObjectsCommand } from './SelectObjectCommand';
import { SetEmbObjectFieldCommand } from './SetSceneObjectFieldCommand';
import { MutateSceneObjectArrayFieldCommand } from './MutateSceneObjectArrayFieldCommand';
import { UpdateGraphicsNodeCommand } from './UpdateGraphicsNodeCommand';
import { SetInspectingCommand } from './SetInspectingCommand';

export {
    ParentObjectCommand as ChangeObjectOrderCommand,
    CreateObjectCommand,
    DeleteObjectCommand,
    DeselectObjectsCommand,
    MoveObjectCommand,
    SelectObjectsCommand,
    SetEmbObjectFieldCommand as SetSceneObjectFieldCommand,
    UpdateGraphicsNodeCommand,
    MutateSceneObjectArrayFieldCommand,
};

type AtomicCommands<TObject extends EmbBase & EmbState> =
    | ParentObjectCommand<TObject>
    | CreateObjectCommand<TObject>
    | DeleteObjectCommand<TObject>
    | DeselectObjectsCommand<TObject>
    | MoveObjectCommand<TObject>
    | SelectObjectsCommand<TObject>
    | SetEmbObjectFieldCommand<TObject>
    | UpdateGraphicsNodeCommand
    | MutateSceneObjectArrayFieldCommand
    | SetInspectingCommand;

export type Command<TObject extends EmbBase & EmbState = EmbBase & EmbState> =
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
    UpdateGraphicsNodeCommand: UpdateGraphicsNodeCommand.prototype,
    MultiCommand: MultiCommand.prototype,
    DeselectObjectsCommand: DeselectObjectsCommand.prototype,
    ParentObjectCommand: ParentObjectCommand.prototype,
    SetInspectingCommand: SetInspectingCommand.prototype,
};
