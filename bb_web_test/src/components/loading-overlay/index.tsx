import { Show } from "solid-js"
import { Progress, ProgressLabel } from "../ui/progress"

type LoadingOverlayProps = {
    status?: string,
    progress: number,
}
// TODO: Refactor to use proper transition animations
export default function LoadingOverlay(props: LoadingOverlayProps) {
    return <div class="z-50 bg-white w-screen h-screen fixed top-0 left-0 flex items-center justify-center transition-opacity duration-500" classList={{
        'opacity-100 ': props.progress !== 1,
        'opacity-0 pointer-events-none': props.progress === 1,
    }}>
        <Progress class="w-full max-w-96" value={props.progress * 100} maxValue={100} >
            <Show when={props.status}>
                <ProgressLabel>{props.status}</ProgressLabel>
            </Show>
        </Progress>
    </div>
}
