import { Show } from "solid-js"
import { Progress, ProgressLabel } from "../ui/progress"

import LogoImage from '../../assets/logo.svg';

type LoadingOverlayProps = {
    status?: string,
    progress: number,
}
// TODO: Refactor to use proper transition animations
export default function LoadingOverlay(props: LoadingOverlayProps) {
    return <div class="z-50 gap-4 bg-background w-screen h-screen fixed top-0 left-0 flex items-center justify-center transition-opacity duration-500" classList={{
        'opacity-100 ': props.progress !== 1,
        'opacity-0 pointer-events-none': props.progress === 1,
    }}>
        <img
            src={LogoImage}
            style={{
                filter: 'drop-shadow(0px 0px 20px #ff8707)'
            }}
            class="animate-spin drop-shadow-2xl shadow-orange-500"
        />
        <Progress class="w-full max-w-96" value={props.progress * 100} maxValue={100} >
            <ProgressLabel>
                <h1 class="text-3xl mb-1">BobbinBear Embroidery</h1>
                <p class="text-xl mb-1">{props.status}</p>
            </ProgressLabel>
        </Progress>
    </div>
}
