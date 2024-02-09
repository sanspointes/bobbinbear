import { IpcApi } from "bb_core"

type IpcButtonsProps = {
    ipc: IpcApi,
}

export function IpcButtons(props: IpcButtonsProps) {
    const handleSpawnCircle = async () => {
        const x = -100 + Math.random() * 200;
        const y = -100 + Math.random() * 200;
        console.log(`JS: Trying to spawn circle at ${x} ${y}`)
        const result = await props.ipc.spawn_circle(x, y);
        console.log(`JS: (returned data from rust) ${result}`);
    }

    const handleDescribeWorld = async () => {
        console.log(await props.ipc.describe_world())
    }
    return <div>
        <button onClick={handleSpawnCircle}>
            Spawn Circle
        </button>
        <button onClick={handleDescribeWorld}>
            Describe World
        </button>
    </div>
}
