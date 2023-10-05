import { TextField as KTextField } from '@kobalte/core';
import clsx from 'clsx';
import { onMount, splitProps } from 'solid-js';

export type TextInputProps = KTextField.TextFieldRootProps & {
    label: string;
    hideLabel?: boolean;
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
        <TextInputRoot {...rootProps}>
            <TextInputLabel class={clsx(remainingProps.hideLabel && 'sr-only')}>
                {remainingProps.label}
            </TextInputLabel>
            <TextInputInput ref={inputEl} onBlur={remainingProps.onBlur} />
        </TextInputRoot>
    );
}

export type TextInputRootProps = KTextField.TextFieldRootProps & {
    noStyles?: boolean;
};
export function TextInputRoot(props: TextInputRootProps) {
    return (
        <KTextField.Root
            {...props}
            class={clsx(
                props.noStyles || 'flex gap-2 items-center',
                props.class,
            )}
        />
    );
}
export type TextInputLabelProps = KTextField.TextFieldLabelProps & {
    noStyles?: boolean;
};
export function TextInputLabel(props: TextInputLabelProps) {
    return <KTextField.Label {...props} class={clsx(props.class)} />;
}
export type TextInputInputProps = KTextField.TextFieldInputProps & {
    noStyles?: boolean;
};
export function TextInputInput(props: TextInputInputProps) {
    return (
        <KTextField.Input
            {...props}
            class={clsx(
                props.noStyles ||
                    'p-2 w-full text-orange-900 bg-white rounded-md box-border appearance-none focus:outline-orange-400/50 focus:outline focus:outline-4',
                props.class,
            )}
        />
    );
}
