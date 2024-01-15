import { CANVAS_ID } from '@/constants';
import { useAppStore } from '@/stores';
import { onMount } from 'solid-js';

export function Renderer() {
    let canvasEl: HTMLCanvasElement | undefined;
    const [_, api] = useAppStore();
    onMount(() => {
        api.core.initEditor();
    });

    return (
        <div class="w-full h-full">
            <canvas
                ref={canvasEl}
                id={CANVAS_ID}
                onPointerEnter={() => {
                    if (canvasEl) canvasEl.focus();
                }}
                onPointerOut={() => {
                    if (canvasEl) canvasEl.blur();
                }}
            />
        </div>
    );
}
