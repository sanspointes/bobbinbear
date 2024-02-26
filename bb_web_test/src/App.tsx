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
        <div class="flex flex-col w-full h-full min-h-screen">
            <div class="w-full h-20 bg-white">
                <div class="card">
                    <Show when={api()}>
                        {(api) => <ApiButtons api={api()} />}
                    </Show>
                </div>
            </div>
            <div class="flex items-stretch grow">
                <Show when={api()}>
                    {(api) => (
                        <div class="min-w-52">
                            <DocTree api={api()} />
                        </div>
                    )}
                </Show>
                <canvas
                    id="bb-canvas"
                    style={{ width: '100%', height: '100%' }}
                />
            </div>
        </div>
    );
}

export default App;
