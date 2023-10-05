import { createSignal, Show, useContext } from 'solid-js';
import { TbChevronDown, TbEye, TbEyeClosed } from 'solid-icons/tb';
import { Collapsible as KCollapsible } from '@kobalte/core';

import { AllMessages, AppContext, GeneralHandler } from '../store';
import { Uuid, uuid } from '../utils/uuid';
import { Tree } from './generics/Tree';
import { EmbBase, EmbState } from '../emb-objects/shared';
import { SceneModel } from '../store/sceneStore';
import { Button } from './generics/Button';
import {
    DeselectObjectsCommand,
    SelectObjectsCommand,
    SetSceneObjectFieldCommand,
} from '../store/commands';
import { TextInput } from './generics/TextInput';
import { useClickOutside } from '../composables/useClickOutside';
import { stopPropagation } from '@solid-primitives/event-listener';
import { useDragDropContext } from '@thisbeyond/solid-dnd';
import { ParentObjectCommand } from '../store/commands/ParentObjectCommand';
import { MultiCommand } from '../store/commands/shared';
import { EmbObject } from '@/emb-objects';

/**
 * Mutation Helpers
 */
const toggleVisibility = (
    object: EmbBase & EmbState,
    dispatch: GeneralHandler<AllMessages>,
) => {
    const newValue = !object.visible;
    const cmd = new SetSceneObjectFieldCommand(object.id, 'visible', newValue);
    dispatch('scene:do-command', cmd);
};

const selectObject = (
    objectId: Uuid<EmbBase & EmbState>,
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

const setObjectName = (
    objectId: Uuid<EmbBase & EmbState>,
    name: string,
    dispatch: GeneralHandler<AllMessages>,
    final?: boolean,
) => {
    const cmd = new SetSceneObjectFieldCommand(objectId, 'name', name);
    cmd.final = final ?? true;
    dispatch('scene:do-command', cmd);
};

/**
 * Tree related helpers
 */
const childResolver = (sceneStore: SceneModel, node: EmbObject & EmbState) => {
    const children = node.children
        .map((id) => sceneStore.objects.get(id))
        // .filter(o => !o?.shallowLocked)
        .filter((o): o is EmbObject & EmbState => o !== undefined);
    return children;
};

const DROPPABLE_ID_REGEX = /(?:(?:(before|after)-)?)(cl-\d+)/;

export function SceneTree() {
    const { dispatch, sceneStore } = useContext(AppContext);
    const [state, { onDragEnd }] = useDragDropContext()!;

    // Handle drag and drop to reparent / reorder
    onDragEnd((event) => {
        const draggableData = event.draggable.data as unknown as EmbObject;
        const droppableData = event.droppable?.data as unknown as
            | EmbObject
            | undefined;
        if (draggableData && event.droppable && droppableData) {
            const droppableParent = sceneStore.objects.get(
                droppableData.parent,
            );
            if (!droppableParent) {
                console.warn(
                    `SceneTree: Attempting to drag ${draggableData.id} to ${droppableData} but can't get parent data.`,
                );
                return;
            }
            const id = event.droppable.id as string;
            if (id.startsWith('cl-')) {
                // Re-parenting
                const cmd = new ParentObjectCommand(
                    draggableData.id,
                    id as Uuid<EmbBase>,
                    'last',
                );
                dispatch('scene:do-command', cmd);
            } else {
                const result = DROPPABLE_ID_REGEX.exec(id);
                if (!result || result.length !== 3) {
                    throw new Error(
                        `SceneTree: Could not parse id of droppable "${id}". Expected ${DROPPABLE_ID_REGEX}.`,
                    );
                }
                const [_, strategy, relatedId] = result as unknown as [
                    _: string,
                    strategy: 'before' | 'after',
                    relatedId: Uuid<EmbObject>,
                ];

                let newPosition = droppableParent.children.findIndex((uuid) => {
                    return relatedId === uuid;
                });
                console.log(droppableParent.children, newPosition);
                if (newPosition === -1) {
                    throw new Error(
                        `SceneTree: Could not get new target position of droppable "${id}"(${relatedId}) out of "${droppableParent.children}".`,
                    );
                }
                console.log(id, strategy, relatedId, newPosition);
                if (strategy === 'before') {
                    newPosition = Math.max(0, newPosition - 1);
                }

                const cmd = new ParentObjectCommand(
                    draggableData.id,
                    droppableData.parent,
                    'absolute',
                    newPosition,
                );
                dispatch('scene:do-command', cmd);
            }
        }
    });

    const root = sceneStore.objects.get(uuid('root'));

    const [currentlyRenaming, setCurrentlyRenaming] = createSignal<string>();
    const [currentClickOutsideTarget, setCurrentClickOutsideTarget] =
        createSignal<HTMLElement>();

    useClickOutside(currentClickOutsideTarget, () => {
        setCurrentlyRenaming(undefined);
        setCurrentClickOutsideTarget(undefined);
    });

    return (
        <div class="overflow-y-scroll text-orange-800 bg-orange-50 fill-orange-800 stroke-orange-50">
            <Tree
                root={root as EmbObject & EmbState}
                childResolver={(node) => childResolver(sceneStore, node)}
                onDragEnd={(e) => console.log(e)}
                isDroppablePredicate={(n) =>
                    n.type === 'canvas' || n.type === 'group'
                }
                droppableTemplate={(active) => (
                    <div
                        class="z-20 w-full bg-orange-800 h-[1px] b-1"
                        classList={{
                            invisible: !active,
                            visible: active,
                        }}
                    />
                )}
                nodeTemplate={(node, children) => (
                    <KCollapsible.Root>
                        <div
                            class="flex items-center w-full h-8 group"
                            classList={{
                                'bg-orange-200': node.selected,
                            }}
                            onClick={() =>
                                selectObject(node.id, sceneStore, dispatch)
                            }
                        >
                            <KCollapsible.Trigger
                                classList={{
                                    'invisible pointer-events-none':
                                        node.children.length === 0,
                                }}
                            >
                                <Button size="tiny" link={true} inverted={true}>
                                    <TbChevronDown class="ml-auto transition-transform transform kb-expanded:rotate-180" />
                                </Button>
                            </KCollapsible.Trigger>
                            <Show
                                when={currentlyRenaming() !== node.id}
                                fallback={
                                    <TextInput
                                        ref={(el) =>
                                            setCurrentClickOutsideTarget(el)
                                        }
                                        autofocus
                                        class="w-full"
                                        label={`Rename "${node.name}"`}
                                        value={node.name}
                                        onChange={(v) =>
                                            setObjectName(
                                                node.id,
                                                v,
                                                dispatch,
                                                false,
                                            )
                                        }
                                        onBlur={(e) => {
                                            setObjectName(
                                                node.id,
                                                e.target.value,
                                                dispatch,
                                                true,
                                            );
                                            setCurrentlyRenaming(undefined);
                                        }}
                                    />
                                }
                            >
                                <span
                                    class="overflow-hidden ml-2 h-6 whitespace-nowrap select-none text-ellipsis"
                                    onDblClick={() =>
                                        setCurrentlyRenaming(node.id)
                                    }
                                >
                                    {node.name} {node.id}
                                </span>

                                <Button
                                    size="tiny"
                                    link={true}
                                    inverted={true}
                                    class="hidden mr-2 ml-auto group-hover:block"
                                    onClick={stopPropagation(() =>
                                        toggleVisibility(node, dispatch),
                                    )}
                                >
                                    <Show
                                        when={node.visible}
                                        fallback={<TbEyeClosed />}
                                    >
                                        <TbEye />
                                    </Show>
                                </Button>
                            </Show>
                        </div>
                        <Show when={node.children.length > 0}>
                            <KCollapsible.Content class="ml-4 border-solid border-y-orange-300 border-y">
                                {children()}
                            </KCollapsible.Content>
                        </Show>
                    </KCollapsible.Root>
                )}
            />
        </div>
    );
}
