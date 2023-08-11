import { For, Show, useContext } from "solid-js";
import { TbChevronDown, TbEye, TbEyeClosed } from "solid-icons/tb";
import { Collapsible as KCollapsible } from "@kobalte/core";

import { AllMessages, AppContext, GeneralHandler } from "../store";
import { SceneObject } from "../types/scene";
import {
  DeselectObjectsCommand,
  SelectObjectsCommand,
  SetSceneObjectFieldCommand,
} from "../store/commands/object";
import { Button } from "./generics/Button";
import { SceneModel } from "../store/sceneStore";
import { MultiCommand } from "../store/commands";
import { Uuid } from "../utils/uuid";

const toggleVisibility = (
  object: SceneObject,
  dispatch: GeneralHandler<AllMessages>,
) => {
  const newValue = !object.visible;
  const cmd = new SetSceneObjectFieldCommand(object.id, "visible", newValue);
  dispatch("scene:do-command", cmd);
};

const selectObject = (
  objectId: Uuid<SceneObject>,
  sceneModel: SceneModel,
  dispatch: GeneralHandler<AllMessages>,
) => {
  const deselectOthersCmd = new DeselectObjectsCommand(
    ...sceneModel.selectedIds,
  );
  const selectThisCommand = new SelectObjectsCommand(objectId);

  dispatch(
    "scene:do-command",
    new MultiCommand(deselectOthersCmd, selectThisCommand),
  );
};

type TreeNodeProps = {
  object: SceneObject;
  indent: number;
};
export function TreeNode(props: TreeNodeProps) {
  const { dispatch, sceneStore } = useContext(AppContext);
  return (
    <KCollapsible.Root
      style={{ "margin-left": `${props.indent * 20}px` }}
      classList={{
        "bg-yellow-400": props.object.selected,
      }}
    >
      <div
        class="flex justify-between select-none hover:outline hover:outline-1 hover:outline-yellow-400"
        onClick={() => selectObject(props.object.id, sceneStore, dispatch)}
      >
        <div class="flex gap-2 items-center">
          <Button
            size="small"
            class="bg-transparent bg-opacity-50 hover:bg-yellow-50"
            onClick={() => toggleVisibility(props.object, dispatch)}
          >
            <Show when={props.object.visible} fallback={<TbEyeClosed />}>
              <TbEye />
            </Show>
          </Button>
          {props.object.id} {props.object.name}
        </div>

        <KCollapsible.Trigger>
          <Show when={props.object.children.length > 0}>
            <Button size="small">
              <TbChevronDown class="ml-auto transition-transform transform kb-expanded:rotate-180" />
            </Button>
          </Show>
        </KCollapsible.Trigger>
      </div>
      <KCollapsible.Content>
        <For each={props.object.children}>
          {(child) => <TreeNode object={child} indent={props.indent + 1} />}
        </For>
      </KCollapsible.Content>
    </KCollapsible.Root>
  );
}

export function Tree() {
  const { sceneStore } = useContext(AppContext);

  return (
    <div class="bg-yellow-200 w-[400px]">
      <For each={sceneStore.root.children}>
        {(child) => <TreeNode object={child} indent={0} />}
      </For>
    </div>
  );
}
