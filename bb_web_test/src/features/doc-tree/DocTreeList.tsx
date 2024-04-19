import { For, createEffect, createMemo } from 'solid-js';
import { useBobbinBear } from '../../hooks/useBobbinBear';
import { DocTreeNode } from './DocTreeNode';


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
                {(object) => <DocTreeNode object={object} indent={0} />}
            </For>
        </div>
    );
}
