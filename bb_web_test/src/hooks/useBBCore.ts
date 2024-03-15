import start, { AppApi, setup_bb_core } from 'bb_core';
import { createSignal, onCleanup } from 'solid-js';

export function useBBCore() {
    const [api, setApi] = createSignal<AppApi | undefined>(undefined);

    onCleanup(() => {
        const i = api();
        if (i) {
            i.exit();
            i.free();
        }
    });

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
            const newApi = new AppApi();
            // @ts-expect-error Make API global
            window.api = newApi;
            setApi(newApi);
        }
    };

    return { setup: handleInit, api };
}
