import { TextField as KTextField } from '@kobalte/core';
import {
    createEffect,
    createMemo,
    createSignal,
    on,
    splitProps,
} from 'solid-js';

export type NumberInputProps = Omit<
    KTextField.TextFieldRootProps,
    'value' | 'onChange'
> & {
    value?: number;
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

    // eslint-disable-next-line solid/reactivity
    const [lastValidValue, setLastValidValue] = createSignal(props.value);
    const [internalValue, setInternalValue] = createSignal(
        // eslint-disable-next-line solid/reactivity
        props.value !== undefined ? `${props.value}` : '',
    );

    createEffect(
        on(internalValue, (internalValue) => {
            const v = Number.parseFloat(internalValue);
            if (!Number.isNaN(v)) {
                props.onChange(v);
                setLastValidValue(v);
            }
        }),
    );
    createEffect(
        on(
            () => props.value,
            (value) => {
                const str = `${value}`;
                if (str !== internalValue()) {
                    setInternalValue(str);
                }
            },
        ),
    );

    const handleBlur = () => {
        setInternalValue(`${lastValidValue()}`);
    };

    return (
        <KTextField.Root
            {...rootProps}
            value={internalValue()}
            onChange={setInternalValue}
            onBlur={handleBlur}
            class="flex gap-2 items-center"
            classList={{
                [props.class ?? '']: props.class !== undefined,
            }}
        >
            <KTextField.Label
                classList={{ 'sr-only': remainingProps.hideLabel }}
            >
                {remainingProps.label}
            </KTextField.Label>
            <KTextField.Input
                type="number"
                class="p-2 w-full text-orange-900 bg-white rounded-md box-border"
                pattern='[0-9].'
                min={remainingProps.min}
                max={remainingProps.max}
                step={remainingProps.step}
            />
        </KTextField.Root>
    );
}
