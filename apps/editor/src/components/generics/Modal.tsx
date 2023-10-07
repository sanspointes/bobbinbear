import { Dialog } from '@kobalte/core';
import { JSX, Show, splitProps } from 'solid-js';
import { VsClose } from 'solid-icons/vs';
import clsx from 'clsx';

export type ModalProps = ModalRootProps & ModalWindowProps;
export function Modal(props: ModalProps) {
    const [modalRootProps, modalWindowProps] = splitProps(props, [
        'open',
        'onOpenChange',
        'onClose',
    ]);
    return (
        <ModalRoot {...modalRootProps}>
            <ModalWindow {...modalWindowProps} />
        </ModalRoot>
    );
}

export type ModalRootProps = {
    open: boolean;
    onOpenChange?: (open: boolean) => void;
    onClose?: () => void;
    children: JSX.Element;
};
export function ModalRoot(props: ModalRootProps) {
    const onOpenChange = (open: boolean) => {
        if (props.onOpenChange) props.onOpenChange(open);
        if (props.onClose && !open) props.onClose();
    };

    return (
        <Dialog.Root open={props.open} onOpenChange={onOpenChange}>
            {props.children}
        </Dialog.Root>
    );
}

export type ModalTriggerProps = JSX.HTMLAttributes<HTMLButtonElement>;
export function ModalTrigger(props: ModalTriggerProps) {
    return <Dialog.Trigger {...props} />;
}

export type ModalWindowProps = {
    title: JSX.Element;
    children: JSX.Element;
    disableClose?: boolean;
    class?: string;
    innerClass?: string;
};
export function ModalWindow(props: ModalWindowProps) {
    return (
        <Dialog.Portal>
            <Dialog.Overlay class="fixed inset-0 z-50 bg-orange-700 bg-opacity-60" />
            <div class="flex fixed inset-0 z-50 justify-center items-center">
                <Dialog.Content
                    class={clsx('p-4 bg-orange-100 rounded-md', props.class)}
                >
                    <div class="flex justify-between items-center mb-4 w-full">
                        <Dialog.Title class="text-lg font-bold">
                            {props.title}
                        </Dialog.Title>
                        <Show when={!props.disableClose}>
                            <Dialog.CloseButton class="b-cursor-pointer">
                                <VsClose />
                            </Dialog.CloseButton>
                        </Show>
                    </div>
                    <Dialog.Description as="div" class={props.innerClass}>
                        {props.children}
                    </Dialog.Description>
                </Dialog.Content>
            </div>
        </Dialog.Portal>
    );
}
