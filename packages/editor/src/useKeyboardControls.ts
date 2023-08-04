import { Accessor, createEffect } from "solid-js";
import { createKeyHold } from '@solid-primitives/keyboard';

export function useKeyboardControls(element: Element): void;
export function useKeyboardControls(element: Accessor<Element|null|undefined>): void;
export function useKeyboardControls(_element: Element | Accessor<Element|null|undefined>) {
  const spaceheld = createKeyHold('Space', { preventDefault: true });
  createEffect(() => {
    if (spaceheld()) {

    }
  })
}
