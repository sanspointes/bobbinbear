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
import { cn } from '../../lib/utils';
import { isDefined } from '~/utils/typeguards';

import { Button } from '../../components/button';
import {
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
} from '~/components/ui/collapsible';

type DocTreeNodeProps = {
    object: DetailedObject;
    indent: number;
};
export function DocTreeNode(props: DocTreeNodeProps) {
    const { document } = useBobbinBear();
    const { setVisible, selectSingle } = document;
    const [expanded, setExpanded] = createSignal(true);

    const childObjects = createMemo(() => {
        if (!props.object.children) return;
        const childObjects = props.object.children
            .map((uid) => document.objects.get(uid))
            .filter(isDefined);
        if (childObjects.length === 0) return undefined;
        else return childObjects;
    });

    createEffect(() => {
        console.log(
            `object(${props.object.uid}) child objects`,
            childObjects(),
        );
    });
    return (
        <Collapsible
            style={{ 'margin-left': `${props.indent * 12}px` }}
            open={expanded()}
            onOpenChange={(open) => setExpanded(open)}
        >
            <div
                class={cn(
                    'flex items-center gap-2 select-none hover:outline hover:outline-1 outline-yellow-600 py-1',
                    props.object.selected && 'bg-orange-100',
                )}
                onClick={async (e) => {
                    selectSingle(props.object.uid);
                    e.stopPropagation();
                }}
            >
                <Show when={childObjects()}>
                    <CollapsibleTrigger onClick={(e) => e.stopPropagation()}>
                        <Button
                            size="tiny"
                            class="bg-transparent bg-opacity-50 hover:bg-orange-50"
                        >
                            <Show
                                when={expanded()}
                                fallback={<TbChevronRight />}
                            >
                                <TbChevronDown />
                            </Show>
                        </Button>
                    </CollapsibleTrigger>
                </Show>

                <Button
                    size="tiny"
                    class="bg-transparent bg-opacity-50 hover:bg-orange-50"
                    onClick={(e) => {
                        setVisible(props.object.uid, !props.object.visible);
                        e.stopPropagation();
                    }}
                >
                    <Show
                        when={props.object.visible}
                        fallback={<TbEyeClosed />}
                    >
                        <TbEye />
                    </Show>
                </Button>
                <Show when={props.object.inspected}>
                    <TbFocus />
                </Show>

                {props.object.name ?? 'Unknown'}
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

export function DocTreeList() {
    const { document } = useBobbinBear();

    const objects = createMemo(() =>
        Array.from(document.objects.values()).filter(
            (obj) => obj.parent === undefined,
        ),
    );

    createEffect(() => {
        console.log('root objects', objects());
    });
    return (
        <div>
            <For each={objects()}>
                {(object) => <DocTreeNode object={object} indent={1} />}
            </For>
        </div>
    );
}
