import { DebugApi, UndoRedoApi } from 'bb_core';
import { Button } from './components/button';

type ApiButtonsProps = {
    field1?: number;
};

export function ApiButtons(props: ApiButtonsProps) {
    const debugApi = new DebugApi();
    const undoRedoApi = new UndoRedoApi();
    const handleSpawnNode = async () => {
        const x = -100 + Math.random() * 200;
        const y = -100 + Math.random() * 200;
        console.log(`JS: Trying to spawn circle at ${x} ${y}`);
        const result = await debugApi.spawn_circle();
        console.log(`JS: (returned data from rust) ${result}`);
    };

    const handleUndo = async () => {
        await undoRedoApi.undo();
    };
    const handleRedo = async () => {
        await undoRedoApi.redo();
    };
    return (
        <div class="flex gap-2">
            <Button onClick={handleSpawnNode}>Spawn Circle</Button>
            <Button onClick={handleUndo}>Undo</Button>
            <Button onClick={handleRedo}>Redo</Button>
        </div>
    );
}
