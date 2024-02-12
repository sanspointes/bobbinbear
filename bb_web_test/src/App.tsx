import { Show, onMount } from 'solid-js';
import './App.css';
import { useBBCore } from './hooks/useBBCore';
import { ApiButtons } from './ApiButtons';

function App() {
    const { api, setup } = useBBCore();
    onMount(() => {
        setup('#bb-canvas');
    });

    return (
        <>
            <canvas id="bb-canvas" />
            <h1>Vite + Solid</h1>
            <div class="card">
                <Show when={api()}>{(api) => <ApiButtons api={api()} />}</Show>
            </div>
        </>
    );
}

export default App;
