import { Checkbox as KCheckbox } from '@kobalte/core';
import { TbCheck } from 'solid-icons/tb';
import { splitProps } from 'solid-js';

type CheckboxProps = KCheckbox.CheckboxRootProps & {
    label: string;
};
export function Checkbox(props: CheckboxProps) {
    const [remainingProps, rootProps] = splitProps(props, ['label']);
    return (
        <KCheckbox.Root
            {...rootProps}
            class="flex gap-2 items-center"
            classList={{
                [props.class ?? '']: props.class !== undefined,
            }}
        >
            <KCheckbox.Input />
            <KCheckbox.Control class="bg-white rounded-md w-8 h-8 flex items-center justify-center">
                <KCheckbox.Indicator>
                    <TbCheck class="w-6 h-6 stroke-orange-800" />
                </KCheckbox.Indicator>
            </KCheckbox.Control>
            <KCheckbox.Label>{remainingProps.label}</KCheckbox.Label>
        </KCheckbox.Root>
    );
}
