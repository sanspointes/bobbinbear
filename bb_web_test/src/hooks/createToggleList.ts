import { createSignal } from 'solid-js/types/server/reactive.js';

export function createToggleList<T>(initialValue = []) {
    const [list, setList] = createSignal<T[]>(initialValue);

    return [
        list,
        {
            toggle(item: T) {
                if (list().includes(item)) {
                    setList((list) => list.filter((el) => el !== item));
                    return false;
                } else {
                    setList((list) => [...list, item]);
                    return true;
                }
            },
            enable(item: T) {
                if (!list().includes(item)) setList((list) => [...list, item]);
            },
            disable(item: T) {
                if (list().includes(item))
                    setList((list) => list.filter((el) => el !== item));
            },
        },
    ] as const;
}
