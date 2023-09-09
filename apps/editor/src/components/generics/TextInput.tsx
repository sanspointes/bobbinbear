import { TextField as KTextField } from '@kobalte/core';
import { onMount, splitProps } from 'solid-js';

export type TextInputProps = KTextField.TextFieldRootProps & {
    label: string;
    hideLabel?: string;
    autofocus?: boolean;
};
export function TextInput(props: TextInputProps) {
    const [remainingProps, rootProps] = splitProps(props, [
        'autofocus',
        'label',
        'hideLabel',
        'onBlur',
    ]);
    let inputEl: HTMLInputElement | undefined;
    onMount(() => {
        if (props.autofocus && inputEl) inputEl.focus();
    });
    return (
        <KTextField.Root
            {...rootProps}
            class="flex gap-2 items-center"
            classList={{
                [props.class ?? '']: props.class !== undefined,
            }}
        >
            <KTextField.Label
                class="text-orange-50"
                classList={{ 'sr-only': !remainingProps.hideLabel }}
            >
                {remainingProps.label}
            </KTextField.Label>
            <KTextField.Input
                ref={inputEl}
                class="p-2 w-full text-orange-900 bg-white rounded-md box-border"
                onBlur={remainingProps.onBlur}
            />
        </KTextField.Root>
    );
}
