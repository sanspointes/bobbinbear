import { Collapsible } from '@kobalte/core';
import { DescribedObject } from 'bb_core';
import { Button } from '../../components/button';
import { For, Show } from 'solid-js';
import { TbEye, TbEyeClosed } from 'solid-icons/tb';
import { useDocTreeContext } from './createDocTreeState';

type DocTreeNodeProps = {
    object: DescribedObject;
    indent: number;
};
export function DocTreeNode(props: DocTreeNodeProps) {
    const [_, { setVisible }] = useDocTreeContext();
    return (
        <Collapsible.Root style={{ 'margin-left': `${props.indent * 20}px` }}>
            <div class="flex justify-between select-none hover:outline">
                <Button
                    size="small"
                    class="bg-transparent bg-opacity-50 hover:bg-orange-50"
                    onClick={() =>
                        setVisible(props.object.uid, !props.object.visible)
                    }
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
    const [items, _] = useDocTreeContext();

    return (
        <div>
            <For each={items()}>
                {(object) => <DocTreeNode object={object} indent={1} />}
            </For>
        </div>
    );
}
