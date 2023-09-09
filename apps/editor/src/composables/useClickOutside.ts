import { access, MaybeAccessor } from '@solid-primitives/utils';
import { createEffect, onCleanup } from 'solid-js';

export const useClickOutside = (
    el: MaybeAccessor<HTMLElement | undefined>,
    handler: (e: MouseEvent) => void,
) => {
    const bodyClickHandler = (e: MouseEvent) => {
        const element = access(el)!;
        if (!element.contains(e.target as Node | null)) {
            handler(e);
        }
    };

    createEffect(() => {
        const element = access(el);
        if (element) {
            document.body.addEventListener('click', bodyClickHandler);
            onCleanup(() => {
                document.body.removeEventListener('click', bodyClickHandler);
            });
        }
    });
};
