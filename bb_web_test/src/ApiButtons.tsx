import { Api } from 'bb_core';
import { Button } from './components/button';

type ApiButtonsProps = {
    api: Api;
};

export function ApiButtons(props: ApiButtonsProps) {
    const handleSpawnNode = async () => {
        const x = -100 + Math.random() * 200;
        const y = -100 + Math.random() * 200;
        console.log(`JS: Trying to spawn circle at ${x} ${y}`);
        const result = await props.api.scene.spawn_node(x, y);
        console.log(`JS: (returned data from rust) ${result}`);
    };

    const handleDescribeWorld = async () => {
        console.log(await props.api.describe_world());
    };
    const handleUndo = async () => {
        await props.api.undoredo.undo();
    };
    const handleRedo = async () => {
        await props.api.undoredo.redo();
    };
    return (
        <div class="flex gap-2">
            <Button onClick={handleSpawnNode}>Spawn Circle</Button>
            <Button onClick={handleDescribeWorld}>Describe World</Button>
            <Button onClick={handleUndo}>Undo</Button>
            <Button onClick={handleRedo}>Redo</Button>
        </div>
    );
}
