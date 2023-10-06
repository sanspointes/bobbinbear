import { JSX, createContext, createMemo, useContext } from 'solid-js';
import clsx from 'clsx';
import { Select as KSelect } from '@kobalte/core';
import { TbCheck } from 'solid-icons/tb';
import { Dynamic } from 'solid-js/web';

/**
 * CONTEXT SETUP
 */
type SelectContextModel<TOption> = {
    itemRenderer: (value: TOption) => JSX.Element | string;
};

const SelectContext = createContext<SelectContextModel<unknown> | null>(null);

const useSelectContext = <TOption,>() => {
    const ctx = useContext(SelectContext);
    if (!ctx)
        throw new Error(
            'useSelectContext: This component must exist within a <SelectRoot /> component.',
        );
    return ctx as SelectContextModel<TOption>;
};

/**
 * COMPONENTS
 */
export type SelectRootProps<TOption> = KSelect.SelectRootProps<TOption> & {
    /**
     * Renderer for each element in options
     */
    itemRenderer: (value: TOption | undefined) => JSX.Element | string;
};
export function SelectRoot<TOption>(props: SelectRootProps<TOption>) {
    const contextValue = createMemo<SelectContextModel<TOption>>(() => ({
        itemRenderer: props.itemRenderer,
    }));

    return (
        <SelectContext.Provider
            value={contextValue() as SelectContextModel<unknown>}
        >
            <KSelect.Root
                {...props}
                class={clsx(props.class)}
                itemComponent={(itemProps) => (
                    <SelectItem {...itemProps}>
                        {props.itemRenderer(itemProps.item.rawValue)}
                    </SelectItem>
                )}
            />
        </SelectContext.Provider>
    );
}

export type SelectLabelProps = KSelect.SelectLabelProps;
export function SelectLabel(props: SelectLabelProps) {
    return <KSelect.Label {...props} />;
}

export type SelectItemProps<TOption> =
    KSelect.SelectRootItemComponentProps<TOption> & {
        children: JSX.Element;
    };
export function SelectItem<TOption>(props: SelectItemProps<TOption>) {
    return (
        <KSelect.Item
            item={props.item}
            class="flex justify-between items-center py-2 px-4 w-full border-b last:border-b-0 border-orange-500 b-cursor-pointer hover:bg-orange-50"
        >
            <KSelect.ItemLabel class="text-left w-full">
                {props.children}
            </KSelect.ItemLabel>
            <KSelect.ItemIndicator>
                <TbCheck />
            </KSelect.ItemIndicator>
        </KSelect.Item>
    );
}

export type SelectTriggerProps = KSelect.SelectTriggerProps;
export function SelectTrigger<TOption>(props: SelectTriggerProps) {
    const ctx = useSelectContext<TOption>();
    return (
        <KSelect.Trigger
            {...props}
            class={clsx(
                'flex overflow-hidden justify-between items-center py-2 px-4 w-full rounded-md',
                'bg-white appearance-none focus:outline-orange-400/50 focus:outline focus:outline-4',
                props.class,
            )}
        >
            <KSelect.Value<TOption> class="w-full text-left">
                {(state) => ctx.itemRenderer(state.selectedOption())}
            </KSelect.Value>
            <KSelect.Icon class="select_icon" />
        </KSelect.Trigger>
    );
}

export type SelectListProps = KSelect.SelectContentProps & {
    usePortal?: boolean;
};
export function SelectList(props: SelectListProps) {
    return (
        <KSelect.Content
            as={props.usePortal ? KSelect.Portal : 'div'}
            {...props}
            class={clsx(
                'w-full rounded-b-md shadow-xl shadow-orange-500/50 bg-white rounded-md overflow-hidden',
                props.class,
            )}
        >
            <KSelect.Listbox />
        </KSelect.Content>
    );
}
