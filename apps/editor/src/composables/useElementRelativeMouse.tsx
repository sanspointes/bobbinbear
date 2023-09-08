import { access } from "@solid-primitives/utils";
import { Accessor, createEffect, createSignal, onCleanup, untrack } from "solid-js";

type ElementRelativeModel = { x: number, y: number, elementWidth: number, elementHeight: number};

export function useElementRelativeMouse(
    elementRef: HTMLElement | Accessor<HTMLElement | undefined>,
    onChange?: (data: ElementRelativeModel) => void,
): [pos: Accessor<ElementRelativeModel>, interacting: Accessor<boolean>] {
    const [interacting, setInteracting] = createSignal(false);
    const [pos, setPos] = createSignal<ElementRelativeModel>({ x: 0, y: 0, elementWidth: 1, elementHeight: 1 });

    const handleChange = (ev: PointerEvent) => {
        const bounds = access(elementRef)!.getBoundingClientRect();
        const x = ev.clientX - bounds.x;
        const y = ev.clientY - bounds.y;

        const model = { x, y, elementWidth: bounds.width, elementHeight: bounds.height };
        if (onChange) onChange(model);
        setPos(model);
    }

    const handleDown = (ev: PointerEvent) => {
        handleChange(ev);
        setInteracting(true);
        window.addEventListener('pointermove', handleMove);
        window.addEventListener('pointerup', handleUp);
    }

    const handleMove = (ev: PointerEvent) => {
        handleChange(ev);
    }

    const handleUp = (ev: PointerEvent) => {
        handleChange(ev);
        setInteracting(false);
        window.removeEventListener('pointermove', handleMove);
        window.removeEventListener('pointerup', handleUp);
    }

    createEffect(() => {
        const el = access(elementRef);

        untrack(() => {
            if(el) {
                const bounds = access(elementRef)!.getBoundingClientRect();
                setPos({...pos(), elementWidth: bounds.width, elementHeight: bounds.height})

                el.addEventListener('pointerdown', handleDown);
                onCleanup(() => el.removeEventListener('pointerdown', handleDown));
            }
        })
    })

    return [pos, interacting];
}
