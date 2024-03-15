import { Collapsible } from '@kobalte/core';
import { DescribedObject } from 'bb_core';
import { Button } from '../../components/button';
import { For, Show } from 'solid-js';
import { TbEye, TbEyeClosed } from 'solid-icons/tb';
import { useBobbinBear } from '../../hooks/useBobbinBear';

type DocTreeNodeProps = {
    object: DescribedObject;
    indent: number;
};
export function DocTreeNode(props: DocTreeNodeProps) {
    const { document } = useBobbinBear();
    const { setVisible, selectSingle } = document;
    return (
        <Collapsible.Root style={{ 'margin-left': `${props.indent * 20}px` }}>
            <div
                class="flex items-center gap-2 select-none hover:outline"
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

    return (
        <div>
            <For each={document.objects()}>
                {(object) => <DocTreeNode object={object} indent={1} />}
            </For>
        </div>
    );
}
