import { TextField as KTextField } from '@kobalte/core';
import { createMemo, splitProps } from 'solid-js';

export type NumberInputProps = Omit<
    KTextField.TextFieldRootProps,
    'value' | 'onChange'
> & {
    value: number;
    onChange: (value: number) => void;

    min?: number;
    max?: number;
    step?: number;

    hideLabel?: boolean;
    precision?: number;
    label: string;
    description?: string;
};
const RESERVED_PROPS = [
    'value',
    'onChange',
    'min',
    'max',
    'step',
    'hideLabel',
    'precision',
    'label',
    'description',
] as const;
export function NumberInput(props: NumberInputProps) {
    const [remainingProps, rootProps] = splitProps(props, RESERVED_PROPS);
    const strValue = createMemo(() => {
        return remainingProps.value.toFixed(props.precision ?? 2);
    });
    const onChange = (value: string) => {
        props.onChange(Number.parseFloat(value));
    };
    return (
        <KTextField.Root
            {...rootProps}
            value={strValue()}
            onChange={onChange}
            class="flex gap-2 items-center"
            classList={{
                [props.class ?? '']: props.class !== undefined,
            }}
        >
            <KTextField.Label
                class="text-orange-50"
                classList={{ 'sr-only': remainingProps.hideLabel }}
            >
                {remainingProps.label}
            </KTextField.Label>
            <KTextField.Input
                type="number"
                class="p-2 w-full text-orange-900 bg-white rounded-md box-border"
                min={remainingProps.min}
                max={remainingProps.max}
                step={remainingProps.step}
            />
        </KTextField.Root>
    );
}
