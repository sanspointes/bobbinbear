import { Select as KSelect } from '@kobalte/core';
import { TbCheck } from 'solid-icons/tb';
import type { JSX } from 'solid-js';
import { splitProps } from 'solid-js';
import { Dynamic } from 'solid-js/web';

export interface SelectSingleSelectionOptions<T> {
    /** The controlled value of the select. */
    value?: T;
    /**
     * The value of the select when initially rendered.
     * Useful when you do not need to control the value.
     */
    defaultValue?: T;
    /** Event handler called when the value changes. */
    onChange?: (value: T) => void;
    /** Whether the select allow multiple selection. */
    multiple?: false;
}
export interface SelectMultipleSelectionOptions<T> {
    /** The controlled value of the select. */
    value?: T[];
    /**
     * The value of the select when initially rendered.
     * Useful when you do not need to control the value.
     */
    defaultValue?: T[];
    /** Event handler called when the value changes. */
    onChange?: (value: T[]) => void;
    /** Whether the select allow multiple selection. */
    multiple: true;
}

export type SelectProps<TOption> = Omit<
    KSelect.SelectRootProps<TOption>,
    | 'itemComponent'
    | 'children'
    | 'value'
    | 'defaultValue'
    | 'onChange'
    | 'multiple'
> &
    (
        | SelectSingleSelectionOptions<TOption>
        | SelectMultipleSelectionOptions<TOption>
    ) & {
        children: (value: TOption) => JSX.Element | string;
        placementStrategy?: 'portal' | 'none';
    };

export function Select<TOption>(props: SelectProps<TOption>) {
    const [internalProps, remainingProps] = splitProps(props, [
        'children',
        'class',
    ]);

    return (
        // @ts-expect-error ; Because KSelect supports both single and multiple selections
        // Typescript can't infer if this is single or multiple props
        <KSelect.Root
            {...remainingProps}
            class="text-orange-900"
            classList={{
                [internalProps.class ?? '']: props.class !== undefined,
            }}
            itemComponent={(props) => (
                <KSelect.Item
                    item={props.item}
                    class="flex justify-between items-center py-2 px-4 w-full border-b border-orange-500 border-solid cursor-pointer hover:bg-orange-100 last-of-type:border-b-0"
                >
                    <KSelect.ItemLabel class="text-left">
                        {internalProps.children(props.item.rawValue)}
                    </KSelect.ItemLabel>
                    <KSelect.ItemIndicator class="select__item-indicator">
                        <TbCheck />
                    </KSelect.ItemIndicator>
                </KSelect.Item>
            )}
        >
            <KSelect.Trigger
                class="flex overflow-hidden justify-between items-center py-2 px-4 w-full bg-orange-50 rounded-md"
                aria-label="Fruit"
            >
                <KSelect.Value<TOption> class="w-full text-left">
                    {(state) => internalProps.children(state.selectedOption())}
                </KSelect.Value>
                <KSelect.Icon class="select_icon" />
            </KSelect.Trigger>
            <Dynamic
                component={
                    props.placementStrategy === 'portal'
                        ? KSelect.Portal
                        : 'div'
                }
            >
                <KSelect.Content class="pt-2 -mt-4 w-full rounded-b-md shadow-2xl shadow-orange-500">
                    <KSelect.Listbox class="bg-orange-200" />
                </KSelect.Content>
            </Dynamic>
        </KSelect.Root>
    );
}
