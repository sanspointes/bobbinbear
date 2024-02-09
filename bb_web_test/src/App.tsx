import { Show, createSignal } from 'solid-js'
import init, { IpcApi, setup_bb_core } from 'bb_core';
import './App.css'
import { IpcButtons } from './IpcButtons';

function App() {
    const [ipc, setIpc] = createSignal<IpcApi|undefined>();

    const handleInit = async () => {
        const result = await init();
        try {
            setup_bb_core("#bb-canvas")
        } catch (e) {
            console.log(e);
        } finally {
            const newIpc = new IpcApi();
            setIpc(newIpc);
        }
    }

    return (
        <>
            <canvas id="bb-canvas">
            </canvas>
            <h1>Vite + Solid</h1>
            <div class="card">
                <Show when={!ipc()}>
                    <button onClick={handleInit}>Init</button>
                </Show>
                <Show when={ipc()}>
                    {ipc => <IpcButtons ipc={ipc()} />}
                </Show>
            </div>
        </>
    )
}

export default App
