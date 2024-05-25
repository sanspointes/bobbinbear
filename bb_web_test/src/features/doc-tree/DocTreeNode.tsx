import { DetailedObject } from 'bb_core';
import { For, Show, createEffect, createMemo, createSignal } from 'solid-js';
import {
    TbChevronDown,
    TbChevronRight,
    TbEye,
    TbEyeClosed,
    TbFocus,
} from 'solid-icons/tb';
import { useBobbinBear } from '../../hooks/useBobbinBear';
import { isDefined } from '~/utils/typeguards';

import { Button } from '~/components/ui/button';
import { Collapsible, CollapsibleContent } from '~/components/ui/collapsible';
import { cn } from '~/lib/utils';

type DocTreeNodeProps = {
    object: DetailedObject;
    indent: number;
};
export function DocTreeNode(props: DocTreeNodeProps) {
    const { document } = useBobbinBear();
    const { setVisible, selectSingle, inspect, hover, unhover } = document;
    const [expanded, setExpanded] = createSignal(true);

    const childObjects = createMemo(() => {
        if (!props.object.children) return;
        const childObjects = props.object.children
            .map((uid) => document.objects.get(uid))
            .filter(isDefined);
        if (childObjects.length === 0) return undefined;
        else return childObjects;
    });
    const showExpandButton = createMemo(() => !!childObjects());

    let pointerInside = false;
    const handlePointerEnter = () => {
        if (!pointerInside) hover(props.object.uid);
        pointerInside = true;
    }
    const handlePointerLeave = () => {
        if (pointerInside) unhover(props.object.uid);
        pointerInside = false;
    }
    return (
        <Collapsible
            open={expanded()}
            onOpenChange={(open) => setExpanded(open)}
        >
            <div
                class="outline-yellow-600"
                classList={{
                    'hover:outline hover:outline-1': props.object.hovered,
                    'bg-accent-background': props.object.selected,
                }}
                onPointerEnter={handlePointerEnter}
                onPointerLeave={handlePointerLeave}
            >
                <div
                    class="flex relative gap-2 items-center py-1 select-none"
                    style={{ 'margin-left': `${props.indent * 12}px` }}
                    onDblClick={() => inspect(props.object.uid)}
                    onClick={async (e) => {
                        selectSingle(props.object.uid);
                        e.stopPropagation();
                    }}
                >
                    <DocTreeNodeButtons
                        showExpandButton={showExpandButton()}
                        expanded={expanded()}
                        onExpandedChange={setExpanded}
                        visible={props.object.visible}
                        onVisibleChange={(visible) =>
                            setVisible(props.object.uid, visible)
                        }
                    />
                    {props.object.name ?? 'Unknown'}

                    <Show when={props.object.inspected}>
                        <div class="absolute right-1 top-1/2 -translate-y-1/2">
                            <TbFocus />
                        </div>
                    </Show>
                </div>
            </div>
            <Show when={childObjects()}>
                {(children) => (
                    <CollapsibleContent>
                        <For each={children()}>
                            {(object) => (
                                <DocTreeNode
                                    object={object}
                                    indent={props.indent + 1}
                                />
                            )}
                        </For>
                    </CollapsibleContent>
                )}
            </Show>
        </Collapsible>
    );
}

type DocTreeNodeButtonProps = {
    showExpandButton: boolean;
    expanded: boolean;
    onExpandedChange: (expanded: boolean) => void;
    visible: boolean;
    onVisibleChange: (visible: boolean) => void;
};
function DocTreeNodeButtons(props: DocTreeNodeButtonProps) {
    return (
        <div class="flex gap-1 items-center">
            <Button
                size="tiny"
                class={cn(
                    'bg-transparent bg-opacity-50 hover:bg-orange-50',
                    !props.showExpandButton && 'opacity-0',
                )}
                onClick={(e) => {
                    e.stopPropagation();
                    props.onExpandedChange(!props.expanded);
                }}
            >
                <Show when={props.expanded} fallback={<TbChevronRight />}>
                    <TbChevronDown />
                </Show>
            </Button>
            <Button
                size="tiny"
                class="bg-transparent bg-opacity-50 hover:bg-orange-50"
                onClick={(e) => {
                    props.onVisibleChange(!props.visible);
                    e.stopPropagation();
                }}
            >
                <Show when={props.visible} fallback={<TbEyeClosed />}>
                    <TbEye />
                </Show>
            </Button>
        </div>
    );
}
