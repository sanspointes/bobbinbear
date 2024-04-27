import { Show, createSignal, onMount } from 'solid-js';

import { ApiButtons } from './ApiButtons';
import { DocTree } from './features/doc-tree';
import { Inspector } from './features/inspector';
import {
    BobbinBearContext,
    BobbinBearModel,
    createBobbinBearContext,
} from './hooks/useBobbinBear';
import { useBBApp } from './hooks/useBobbinBear/app';
import { createElementSize, createResizeObserver } from '@solid-primitives/resize-observer';

function App() {
    const app = useBBApp();
    const [loading, setLoading] = createSignal(0);
    const [ctx, setCtx] = createSignal<BobbinBearModel | undefined>(undefined);
    onMount(async () => {
        setLoading(0.1);
        await app.setup('#bb-canvas');
        setLoading(0.8);
        setCtx(createBobbinBearContext());
        setTimeout(() => {
            setLoading(1);
        }, 50);
    });

    const [canvasContainer, setCanvasContainer] = createSignal<HTMLDivElement|null>(null);
    createResizeObserver(canvasContainer, (rect) => {
        const c = ctx();
        if (!c) return;
        c.viewport.setResolution(rect.width * window.devicePixelRatio, rect.height * window.devicePixelRatio);
    });

    return (
        <div class="flex flex-col w-full h-full min-h-screen">
            <div class="w-full h-20 bg-white">
                <div class="card">
                    <Show when={loading() === 1}>{(_) => <ApiButtons />}</Show>
                </div>
            </div>
            <div class="flex items-stretch grow">
                <Show when={loading() === 1}>
                    <BobbinBearContext.Provider value={ctx()}>
                        <div class="w-[300px]">
                            <DocTree />
                        </div>
                    </BobbinBearContext.Provider>
                </Show>
                <div class="bg-red-500 grow">
                    <div ref={setCanvasContainer} class="relative w-full h-full">
                        <canvas id="bb-canvas" class="absolute top-0 left-0" />
                    </div>
                </div>
                <Show when={loading() === 1}>
                    <BobbinBearContext.Provider value={ctx()}>
                        <div class="w-[300px]">
                            <Inspector />
                        </div>
                    </BobbinBearContext.Provider>
                </Show>
            </div>
        </div>
    );
}

export default App;
