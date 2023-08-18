import { AbstractCommand, SerializedCommand, assertSameField, assertSameType } from "./shared";
import { SetStoreFunction } from "solid-js/store";
import { BaseSceneObject } from "../../types/scene";
import { SceneModel } from "../sceneStore";
import { ParentObjectCommand } from "./ParentObjectCommand";
import { CreateObjectCommand } from "./CreateObjectCommand";
import { DeleteObjectCommand } from "./DeleteObjectCommand";
import { DeselectObjectsCommand } from "./DeselectObjectCommand";
import { MoveObjectCommand } from "./MoveObjectCommand";
import { SelectObjectsCommand } from "./SelectObjectCommand";
import { SetSceneObjectFieldCommand } from "./SetSceneObjectFieldCommand";
import { UpdateGraphicsNodeCommand } from "./UpdateGraphicsNodeCommand";
import { batch } from "solid-js";

export {
  ParentObjectCommand as ChangeObjectOrderCommand,
  CreateObjectCommand,
  DeleteObjectCommand,
  DeselectObjectsCommand,
  MoveObjectCommand,
  SelectObjectsCommand,
  SetSceneObjectFieldCommand,
  UpdateGraphicsNodeCommand,
};

type AtomicCommands<TObject extends BaseSceneObject = BaseSceneObject> =
  | ParentObjectCommand<TObject>
  | CreateObjectCommand<TObject>
  | DeleteObjectCommand<TObject>
  | DeselectObjectsCommand<TObject>
  | MoveObjectCommand<TObject>
  | SelectObjectsCommand<TObject>
  | SetSceneObjectFieldCommand<TObject>
  | UpdateGraphicsNodeCommand;

export type Command<TObject extends BaseSceneObject = BaseSceneObject> =
  | MultiCommand<TObject>
  | AtomicCommands<TObject>;
export type CommandType = Command["type"];

export type CommandPrototypeMap = Record<CommandType, AbstractCommand>;
export const _commandPrototypeMap: Record<CommandType, Command> = {
  "CreateObjectCommand": CreateObjectCommand.prototype,
  "DeleteObjectCommand": DeleteObjectCommand.prototype,
  "MoveObjectCommand": MoveObjectCommand.prototype,
  "SelectObjectsCommand": SelectObjectsCommand.prototype,
  "SetSceneObjectFieldCommand": SetSceneObjectFieldCommand.prototype,
  "UpdateGraphicsNodeCommand": UpdateGraphicsNodeCommand.prototype,
};

export class MultiCommand<TObject extends BaseSceneObject>
  extends AbstractCommand {
  public updatable: boolean = true;
  name = "Multi Command";
  type = "MultiCommand";
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

  // @ts-expect-error ; Difficult typing
  updateData(newer: Command<TObject>): void {
    // @ts-expect-error ; Difficult typing
    const n = assertSameType(this, newer) as MultiCommand<TObject>;
    // @ts-expect-error ; Difficult typing
    assertSameField(this, n, length);
    let i = 0;
    for (const cmd of this.commands) {
      const newerCmd = n.commands[i]!;
      // @ts-expect-error ; Difficult typing
      const n2 = assertSameType(cmd, newerCmd) as Command<TObject>;
      // @ts-expect-error ; Difficult typing
      if (cmd.updateData) cmd.updateData(n2);
      i += 1;
    }
  }
}
