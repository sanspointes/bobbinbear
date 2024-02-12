import { Show, onMount } from 'solid-js';
import { useBBCore } from './hooks/useBBCore';
import { ApiButtons } from './ApiButtons';
import { DocTree } from './features/doc-tree';

function App() {
    const { api, setup } = useBBCore();
    onMount(() => {
        setup('#bb-canvas');
    });

    return (
        <div class="w-full h-full min-h-screen flex flex-col">
            <div class="w-full h-20 bg-white">
                <div class="card">
                    <Show when={api()}>
                        {(api) => <ApiButtons api={api()} />}
                    </Show>
                </div>
            </div>
            <div class="flex grow">
                <Show when={api()}>
                    {(api) => (
                        <div class="min-w-52">
                            <DocTree api={api()} />
                        </div>
                    )}
                </Show>
                <canvas id="bb-canvas" />
            </div>
        </div>
    );
}

export default App;
