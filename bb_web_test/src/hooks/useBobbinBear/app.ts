import start, { AppApi, BobbinCursor, Effect, setup_bb_core } from 'bb_core';
import wasmUrl from 'bb_core/pkg/bb_core_bg.wasm?url';
import fetchProgress, { FetchProgressInitOptions } from 'fetch-progress';
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

    const handleInit = async (
        canvasSelector: string,
        progressOptions?: FetchProgressInitOptions = {},
    ) => {
        try {
            const response = await fetch(wasmUrl).then(fetchProgress(progressOptions));

            await start(response);
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
