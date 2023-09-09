import { For, Show, useContext } from 'solid-js';
import { TbChevronDown, TbEye, TbEyeClosed } from 'solid-icons/tb';
import { Collapsible as KCollapsible } from '@kobalte/core';

import { AllMessages, AppContext, GeneralHandler } from '../store';
import { EmbBase } from '../emb-objects/shared';
import {
    DeselectObjectsCommand,
    SelectObjectsCommand,
    SetSceneObjectFieldCommand,
    MultiCommand,
} from '../store/commands';
import { Button } from './generics/Button';
import { SceneModel } from '../store/sceneStore';
import { Uuid, uuid } from '../utils/uuid';

/**
 * Helpers
 */
const toggleVisibility = (
    object: EmbBase,
    dispatch: GeneralHandler<AllMessages>,
) => {
    const newValue = !object.visible;
    const cmd = new SetSceneObjectFieldCommand(object.id, 'visible', newValue);
    dispatch('scene:do-command', cmd);
};

const selectObject = (
    objectId: Uuid<EmbBase>,
    sceneModel: SceneModel,
    dispatch: GeneralHandler<AllMessages>,
) => {
    const deselectOthersCmd = new DeselectObjectsCommand(
        ...sceneModel.selectedIds,
    );
    const selectThisCommand = new SelectObjectsCommand(objectId);

    const cmd = new MultiCommand(deselectOthersCmd, selectThisCommand);
    cmd.name = `Select ${objectId}`;
    dispatch('scene:do-command', cmd);
};

type TreeNodeProps = {
    object: EmbBase;
    indent: number;
};
export function TreeNode(props: TreeNodeProps) {
    const { dispatch, sceneStore } = useContext(AppContext);
    return (
        <KCollapsible.Root
            style={{ 'margin-left': `${props.indent * 20}px` }}
            classList={{
                'bg-orange-400': props.object.selected,
            }}
        >
            <div
                class="flex justify-between select-none hover:outline hover:outline-1 hover:outline-orange-600"
                onClick={() =>
                    selectObject(props.object.id, sceneStore, dispatch)
                }
            >
                <div class="flex gap-2 items-center">
                    <Button
                        size="small"
                        class="bg-transparent bg-opacity-50 hover:bg-orange-50"
                        onClick={() => toggleVisibility(props.object, dispatch)}
                    >
                        <Show
                            when={props.object.visible}
                            fallback={<TbEyeClosed />}
                        >
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
                    {(child) => {
                        // eslint-disable-next-line solid/reactivity
                        const obj = sceneStore.objects.get(child);

                        if (!obj) return <span> Error getting {child} </span>;
                        return (
                            <TreeNode object={obj} indent={props.indent + 1} />
                        );
                    }}
                </For>
            </KCollapsible.Content>
        </KCollapsible.Root>
    );
}

export function Tree() {
    const { sceneStore } = useContext(AppContext);

    const root = sceneStore.objects.get(uuid('root'));

    return (
        <div class="bg-orange-500 w-[400px] h-full overflow-y-scroll">
            <For each={root!.children}>
                {(child) => {
                    // eslint-disable-next-line solid/reactivity
                    const obj = sceneStore.objects.get(child);

                    if (!obj) return <span> Error getting {child} </span>;
                    return <TreeNode object={obj} indent={0} />;
                }}
            </For>
        </div>
    );
}
