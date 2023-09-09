import { Accessor, onCleanup } from 'solid-js';

type ClickOutsideHandler = (e: MouseEvent) => void;

declare module 'solid-js' {
    // eslint-disable-next-line @typescript-eslint/no-namespace
    namespace JSX {
        interface Directives {
            // use:model
            clickOutside: ClickOutsideHandler;
        }
    }
}

export function clickOutside(
    el: HTMLElement,
    accessor: Accessor<ClickOutsideHandler>,
) {
    console.log('Click outside directive bound');
    const onClick = (e: MouseEvent) =>
        !el.contains(e.target as Node | null) && accessor()?.(e);
    document.body.addEventListener('click', onClick);

    onCleanup(() => document.body.removeEventListener('click', onClick));
}
