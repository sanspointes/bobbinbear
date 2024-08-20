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
import { createResizeObserver } from '@solid-primitives/resize-observer';
import LoadingOverlay from './components/loading-overlay';
import { Toolbar } from './features/toolbar';
import clsx from 'clsx';

function App() {
    const app = useBBApp();
    const [loading, setLoading] = createSignal(0);
    const [loadingStatus, setLoadingStatus] = createSignal<
        string | undefined
    >();
    const [ctx, setCtx] = createSignal<BobbinBearModel | undefined>(undefined);
    onMount(async () => {
        setLoadingStatus('Loading editor...');
        setLoading(0.1);
        await app.setup('#bb-canvas', {
            onProgress(ev) {
                setLoading(0.1 + (ev.transferred / ev.total) * 0.7);
            },
            onComplete() {
                setLoading(0.8);
                setLoadingStatus('Booting Up...');
            },
        });
        setCtx(createBobbinBearContext());
        setTimeout(() => {
            setLoading(1);
        }, 1000);
    });

    const [canvasContainer, setCanvasContainer] =
        createSignal<HTMLDivElement | null>(null);
    createResizeObserver(canvasContainer, (rect) => {
        const c = ctx();
        if (!c) return;
        c.viewport.setResolution(
            rect.width * window.devicePixelRatio,
            rect.height * window.devicePixelRatio,
        );
    });

    return (
        <>
            <LoadingOverlay progress={loading()} status={loadingStatus()} />
            <div class="flex flex-col w-full h-screen">
                <Show when={loading() === 1}>
                    <BobbinBearContext.Provider value={ctx()}>
                        <Toolbar />
                    </BobbinBearContext.Provider>
                </Show>
                <div class="flex items-stretch grow">
                    <Show when={loading() === 1}>
                        <BobbinBearContext.Provider value={ctx()}>
                            <div class="w-[300px] overflow-y-scroll">
                                <DocTree />
                            </div>
                        </BobbinBearContext.Provider>
                    </Show>
                    <div class="bg-red-500 grow">
                        <div
                            ref={setCanvasContainer}
                            class="relative w-full h-full"
                        >
                            <canvas
                                id="bb-canvas"
                                class={clsx(
                                    'absolute top-0 left-0 ring-transparent',
                                    `cursor-${ctx()?.tools.cursor()}`,
                                )}
                            />
                        </div>
                    </div>
                    <Show when={loading() === 1}>
                        <BobbinBearContext.Provider value={ctx()}>
                            <div class="w-[300px] overflow-y-scroll">
                                <Inspector />
                            </div>
                        </BobbinBearContext.Provider>
                    </Show>
                </div>
            </div>
        </>
    );
}

export default App;
