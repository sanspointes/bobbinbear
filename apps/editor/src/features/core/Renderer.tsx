import * as BobbinBearCore from '@bearbroidery/bobbinbear-core';
import { Show, createSignal, onMount } from 'solid-js';
import type { EditorApi } from '@bearbroidery/bobbinbear-core';

const CANVAS_ID = 'bobbinbear-core-canvas';

type LazyCanvasProps = {
    onReady: (api: EditorApi) => void;
};
function LazyRenderer(props: LazyCanvasProps) {
    onMount(() => {
        // eslint-disable-next-line solid/reactivity
        BobbinBearCore.default().then(() => {
            console.log(
                'initialising renderer',
                document.getElementById(CANVAS_ID),
            );
            BobbinBearCore.main_web(`#${CANVAS_ID}`, props.onReady);
        });
    });
    let canvasEl: HTMLCanvasElement | undefined;

    return (
        <canvas
            ref={canvasEl}
            id={CANVAS_ID}
            onPointerEnter={() => {
                console.log('over');
                if (canvasEl) canvasEl.focus();
            }}
            onPointerOut={() => {
                console.log('out');
                if (canvasEl) canvasEl.blur();
            }}
        />
    );
}

export function RendererFallback() {
    return (
        <div class="flex flex-col justify-center items-center w-full h-full bg-orange-100">
            <div>Loading...</div>
        </div>
    );
}

export function Renderer() {
    const [isReady, setIsReady] = createSignal(false);

    return (
        <div class="w-full h-full">
            <Show when={!isReady()}>
                <RendererFallback />
            </Show>
            <LazyRenderer onReady={() => setIsReady(true)} />
        </div>
    );
}
