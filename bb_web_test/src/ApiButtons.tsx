import { Api } from 'bb_core';

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
        <div>
            <button onClick={handleSpawnNode}>Spawn Circle</button>
            <button onClick={handleDescribeWorld}>Describe World</button>
            <button onClick={handleUndo}>Undo</button>
            <button onClick={handleRedo}>Redo</button>
        </div>
    );
}
