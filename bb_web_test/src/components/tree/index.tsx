import { JSX } from 'solid-js/jsx-runtime';
import { TreeContext, createTreeContext, injectTreeContext } from './context';
import { For } from 'solid-js';

export type TreeItem<TId, TData> = {
    id: TId;
    data: TData;
    children: TId[];
};

export type TreeProps<TId extends string | number, TData> = {
    data: Record<TId, TreeItem<TId, TData>>;
    itemRenderer: (item: TreeItem<TId, TData>) => JSX.Element;
    onItemSelected: (item: TreeItem<TId, TData>) => void;
};

export function Tree<TId extends string | number, TData>(
    props: TreeProps<TId, TData>,
) {
    const treeContext = createTreeContext({
        data: props.data,
        itemRenderer: props.itemRenderer,
    });
    const [data] = treeContext;
    return (
        <TreeContext.Provider value={treeContext}>
            <div>
                <For each={data.roots()}>
                    {(root) => <TreeNode item={root} />}
                </For>
            </div>
        </TreeContext.Provider>
    );
}

type TreeNodeProps<TId, TData> = {
    node: TreeItem<TId, TData>;
};
export function TreeNode<TId extends string | number, TData>(
    props: TreeNodeProps<TId, TData>,
) {
    const [data, api] = injectTreeContext();
    return (
        <div>
            <div
                class="flex gap-2 justify-between items-center"
                onClick={() => api.toggleExpanded(props.node.id)}
            >
                <div>{data.itemRenderer(props.node)}</div>
                <div>x</div>
            </div>
            {props.node.children && (
                <div>
                    <For each={props.node.children}>
                        {(id) => <TreeNode node={data.data[id]} />}
                    </For>
                </div>
            )}
        </div>
    );
}
