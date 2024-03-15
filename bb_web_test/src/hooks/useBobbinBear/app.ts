import start, { AppApi, Effect, setup_bb_core } from 'bb_core';
import { createSignal, onCleanup } from 'solid-js';

export function useBBApp() {
    const [api, setApi] = createSignal<AppApi | undefined>(undefined);

    onCleanup(() => {
        const i = api();
        if (i) {
            i.exit();
            i.free();
        }
    });

    const handleEffect = (effect: Effect) => {
        console.log(effect);
    };
    // @ts-expect-error: untyped...
    window.receiveRustEvents = handleEffect;

    const handleInit = async (canvasSelector: string) => {
        try {
            await start();
        } catch (reason) {
            console.warn('init with error: ', reason);
        }
        try {
            setup_bb_core(canvasSelector);
        } catch (e) {
            console.log(e);
        } finally {
            setApi(new AppApi());
        }
    };

    return { setup: handleInit, api };
}
