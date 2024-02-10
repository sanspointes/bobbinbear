import initBBCore, { IpcApi, setup_bb_core } from "bb_core";
import { createSignal, onCleanup } from "solid-js";

export function useBBCore() {
    const [ipc, setIpc] = createSignal<IpcApi|undefined>(undefined);

    onCleanup(() => {
        const i = ipc();
        if (i) {
            i.exit();
            i.free();
        }
    })

    const handleInit = async (canvasSelector: string) => {
        try {
            await initBBCore();
        } catch(reason) {
            console.warn('init with error: ', reason);
        }
        try {
            setup_bb_core(canvasSelector)
        } catch (e) {
            console.log(e);
        } finally {
            const newIpc = new IpcApi();
            setIpc(newIpc);
        }
    }

    return { setup: handleInit, ipc };
}
