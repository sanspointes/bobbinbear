import { createSignal, Show, useContext } from "solid-js";
import { TbChevronDown, TbEye, TbEyeClosed } from "solid-icons/tb";
import { Collapsible as KCollapsible } from "@kobalte/core";

import { AllMessages, AppContext, GeneralHandler } from "../store";
import { Uuid, uuid } from "../utils/uuid";
import { Tree } from "./generics/Tree";
import { BaseSceneObject } from "../types/scene";
import { SceneModel } from "../store/sceneStore";
import { Button } from "./generics/Button";
import {
  DeselectObjectsCommand,
  MultiCommand,
  SelectObjectsCommand,
  SetSceneObjectFieldCommand,
} from "../store/commands";
import { TextInput } from "./generics/TextInput";
import { useClickOutside } from "../composables/useClickOutside";

/**
 * Mutation Helpers
 */
const toggleVisibility = (
  object: BaseSceneObject,
  dispatch: GeneralHandler<AllMessages>,
) => {
  const newValue = !object.visible;
  const cmd = new SetSceneObjectFieldCommand(object.id, "visible", newValue);
  dispatch("scene:do-command", cmd);
};

const selectObject = (
  objectId: Uuid<BaseSceneObject>,
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

const setObjectName = (
  objectId: Uuid<BaseSceneObject>,
  name: string,
  dispatch: GeneralHandler<AllMessages>,
  final?: boolean,
) => {
  const cmd = new SetSceneObjectFieldCommand(objectId, "name", name);
  cmd.final = final ?? true;
  dispatch("scene:do-command", cmd);
};

/**
 * Tree related helpers
 */
const childResolver = (sceneStore: SceneModel, node: BaseSceneObject) => {
  const children = node.children
    .map((id) => sceneStore.objects.get(id))
    // .filter(o => !o?.shallowLocked)
    .filter(
      (o): o is BaseSceneObject => o !== undefined,
    );
  return children;
};

export function SceneTree() {
  const { dispatch, sceneStore } = useContext(AppContext);

  const root = sceneStore.objects.get(uuid("root"));

  const [currentlyRenaming, setCurrentlyRenaming] = createSignal<string>();
  const [currentClickOutsideTarget, setCurrentClickOutsideTarget] = createSignal<HTMLElement>();
  useClickOutside(currentClickOutsideTarget, () => {
    setCurrentlyRenaming(undefined);
    setCurrentClickOutsideTarget(undefined);
  })

  return (
    <div class="overflow-y-scroll h-full bg-orange-50 fill-orange-800 stroke-orange-50 text-orange-800 w-[400px]">
      <Tree
        root={root as BaseSceneObject}
        childResolver={(node) => childResolver(sceneStore, node)}
        onDragEnd={(e) => console.log(e)}
        droppableTemplate={(active) => (
          <div
            class="z-20 w-full h-2 bg-orange-200 bg-opacity-20 b-1"
            classList={{
              "invisible": !active,
              "visible": active,
            }}
          />
        )}
        nodeTemplate={(node, children) => (
          <KCollapsible.Root
            classList={{
              "bg-orange-400": node.selected,
            }}
          >
            <div
              class="flex items-center w-full h-8"
              onClick={() => selectObject(node.id, sceneStore, dispatch)}
            >
              <KCollapsible.Trigger
                classList={{
                  "invisible pointer-events-none": node.children.length === 0,
                }}
              >
                <Button size="small" link={true} inverted={true}>
                  <TbChevronDown class="ml-auto transition-transform transform kb-expanded:rotate-180" />
                </Button>
              </KCollapsible.Trigger>
              <Show
                when={currentlyRenaming() !== node.id}
                fallback={
                  <TextInput
                    ref={el => setCurrentClickOutsideTarget(el)}
                    autofocus
                    label={`Rename "${node.name}"`}
                    value={node.name}
                    onChange={(v) => setObjectName(node.id, v, dispatch, false)}
                    onBlur={(e) => {
                      setObjectName(node.id, e.target.value, dispatch, true);
                      setCurrentlyRenaming(undefined);
                    }}
                  />
                }
              >
                <Button
                  size="small"
                  link={true}
                  inverted={true}
                  onClick={() => toggleVisibility(node, dispatch)}
                >
                  <Show when={node.visible} fallback={<TbEyeClosed />}>
                    <TbEye />
                  </Show>
                </Button>
                <span
                  class="ml-2 h-6 select-none"
                  onDblClick={() => setCurrentlyRenaming(node.id)}
                >
                  {node.name}
                </span>
              </Show>
            </div>
            <Show when={node.children.length > 0}>
              <KCollapsible.Content>
                {children()}
              </KCollapsible.Content>
            </Show>
          </KCollapsible.Root>
        )}
      />
    </div>
  );
}
