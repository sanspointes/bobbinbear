import { createContext, createMemo, useContext } from 'solid-js/types/server/reactive.js';
import { TreeItem } from '.';
import { JSX } from 'solid-js/jsx-runtime';
import { createToggleList } from '../../hooks/createToggleList';
import { objectValues } from '../../utils/array';

type TreeContextProps<TId extends string | number, TData> = {
    data: Record<TId, TreeItem<TId, TData>>;
    itemRenderer: (item: TreeItem<TId, TData>) => JSX.Element;
    onItemSelected?: (item: TreeItem<TId, TData>) => void;
    onItemExpandChanged?: (
        item: TreeItem<TId, TData>,
        isExpanded: boolean,
    ) => void;
};

export const TreeContext = createContext<ReturnType<
    typeof createTreeContext
> | null>(null);

export function createTreeContext<TId extends string | number, TData>(
    props: TreeContextProps<TId, TData>,
) {
    const [expandedNodes, { toggle: toggleExpanded }] = createToggleList<TId>(
        [],
    );
    const parentLookup = createMemo(() => {
        const parentLookup = {} as Record<TId, TId>;
        for (const id in props.data) {
            const node = props.data[id];
            for (const c of node.children) {
                parentLookup[c] = node.id;
            }
        }
        return parentLookup;
    });
    const roots = createMemo(() => {
        const pl = parentLookup();
        return objectValues(props.data).filter((v) => pl[v.id] === undefined);
    });

    return [
        {
            // eslint-disable-next-line solid/reactivity
            data: props.data,
            // eslint-disable-next-line solid/reactivity
            itemRenderer: props.itemRenderer,
            roots,
            expandedNodes,
            parentLookup,
        },
        {
            /**
             * @param id - ID of node to toggle expanded on.
             */
            toggleExpanded(id: TId) {
                const isExpanded = toggleExpanded(id);
                if (props.onItemExpandChanged)
                    props.onItemExpandChanged(props.data[id], isExpanded);
            },
        },
    ] as const;
}

export function injectTreeContext() {
    return useContext(TreeContext)!;
}
