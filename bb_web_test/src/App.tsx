import { Show, onMount } from 'solid-js'
import './App.css'
import { IpcButtons } from './IpcButtons';
import { useBBCore } from './hooks/useBBCore';

function App() {
    const { ipc, setup } = useBBCore();
    onMount(() => {
        setup('#bb-canvas')
    })

    return (
        <>
            <canvas id="bb-canvas">
            </canvas>
            <h1>Vite + Solid</h1>
            <div class="card">
                <Show when={ipc()}>
                    {ipc => <IpcButtons ipc={ipc()} />}
                </Show>
            </div>
        </>
    )
}

export default App
