import { Collapsible } from '@kobalte/core';
import { DescribedObject } from 'bb_core';
import { Button } from '../../components/button';
import { For, Show, createMemo } from 'solid-js';
import { TbEye, TbEyeClosed } from 'solid-icons/tb';
import { useBobbinBear } from '../../hooks/useBobbinBear';
import { cn } from '../../lib/utils';

type DocTreeNodeProps = {
    object: DescribedObject;
    indent: number;
};
export function DocTreeNode(props: DocTreeNodeProps) {
    const { document } = useBobbinBear();
    const { setVisible, selectSingle } = document;
    return (
        <Collapsible.Root style={{ 'margin-left': `${props.indent * 0}px` }}>
            <div
                class={cn(
                    'flex items-center gap-2 select-none hover:outline hover:outline-1 outline-yellow-600',
                    props.object.selected && 'bg-orange-100',
                )}
                onClick={async (e) => {
                    selectSingle(props.object.uid);
                    e.stopPropagation();
                }}
            >
                <Button
                    size="small"
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

                {props.object.name ?? 'Unknown'}
            </div>
        </Collapsible.Root>
    );
}

export function DocTreeList() {
    const { document } = useBobbinBear();

    const objects = createMemo(() => Array.from(document.objects.values()));

    return (
        <div>
            <For each={objects()}>
                {(object) => <DocTreeNode object={object} indent={1} />}
            </For>
        </div>
    );
}
