import { DebugApi, UndoRedoApi, ViewportApi } from 'bb_core';
import { Button } from './components/button';
import { createEffect, createSignal } from 'solid-js';

type ApiButtonsProps = {
    field1?: number;
};

export function ApiButtons(props: ApiButtonsProps) {
    const debugApi = new DebugApi();
    const undoRedoApi = new UndoRedoApi();
    const viewportApi = new ViewportApi();
    const handleSpawnNode = async () => {
        const x = -100 + Math.random() * 200;
        const y = -100 + Math.random() * 200;
        console.log(`JS: Trying to spawn circle at ${x} ${y}`);
        const result = await debugApi.spawn_box();
        console.log(`JS: (returned data from rust) ${result}`);
    };

    const handleUndo = async () => {
        await undoRedoApi.undo();
    };
    const handleRedo = async () => {
        await undoRedoApi.redo();
    };

    const [positionX, setPositionX] = createSignal(0);
    const [positionY, setPositionY] = createSignal(0);
    createEffect(() => {
        viewportApi.set_position(positionX(), positionY());
    });
    const zoomMult = async (multiplier: number) => {
        return viewportApi.set_zoom(
            (await viewportApi.get_zoom()) * multiplier,
        );
    };

    return (
        <div class="flex gap-2">
            <Button onClick={handleSpawnNode}>Spawn Circle</Button>
            <Button onClick={handleUndo}>Undo</Button>
            <Button onClick={handleRedo}>Redo</Button>

            <input
                value={positionX()}
                onChange={(e) => setPositionX(Number.parseInt(e.target.value))}
            />
            <input
                value={positionY()}
                onChange={(e) => setPositionY(Number.parseInt(e.target.value))}
            />
            <Button onClick={() => zoomMult(0.95)}>+</Button>
            <Button onClick={() => zoomMult(1.05)}>-</Button>
        </div>
    );
}
