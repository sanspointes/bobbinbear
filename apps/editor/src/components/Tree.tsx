import { Index, Show, useContext } from "solid-js";
import { TbChevronDown, TbEye, TbEyeClosed } from "solid-icons/tb";
import { Collapsible as KCollapsible } from "@kobalte/core";

import { AllMessages, AppContext, GeneralHandler } from "../store";
import { BaseSceneObject, SceneObject } from "../types/scene";
import {
  DeselectObjectsCommand,
  SelectObjectsCommand,
  SetSceneObjectFieldCommand,
} from "../store/commands";
import { Button } from "./generics/Button";
import { SceneModel } from "../store/sceneStore";
import { MultiCommand } from "../store/commands";
import { Uuid, uuid } from "../utils/uuid";

const toggleVisibility = (
  object: BaseSceneObject,
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

  const cmd = new MultiCommand(deselectOthersCmd, selectThisCommand);
  cmd.name = `Select ${objectId}`;
  dispatch(
    "scene:do-command",
    cmd,
  );
};

type TreeNodeProps = {
  object: BaseSceneObject;
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
        <Index each={props.object.children}>
          {(child) => {
            // eslint-disable-next-line solid/reactivity
            const obj = sceneStore.objects.get(child());

            if (!obj) return <span> Error getting {child()} </span>;
            return <TreeNode object={obj} indent={props.indent + 1} />
          }}
        </Index>
      </KCollapsible.Content>
    </KCollapsible.Root>
  );
}

export function Tree() {
  const { sceneStore } = useContext(AppContext);

  const root = sceneStore.objects.get(uuid('root'));

  return (
    <div class="bg-yellow-200 w-[400px] h-full overflow-y-scroll">
      <Index each={root!.children}>
        {(child) => {
          // eslint-disable-next-line solid/reactivity
          const obj = sceneStore.objects.get(child());

          if (!obj) return <span> Error getting {child()} </span>;
          return <TreeNode object={obj} indent={0} />
        }}
      </Index>
    </div>
  );
}
