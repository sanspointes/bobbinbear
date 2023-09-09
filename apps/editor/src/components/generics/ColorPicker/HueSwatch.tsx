import { createSignal } from 'solid-js';
import clsx from 'clsx';
import { useElementRelativeMouse } from '../../../composables/useElementRelativeMouse';
import { clamp } from '../../../utils/math';
import { HsvColor } from '../../../utils/color';

type HueSwatchProps = {
    color: HsvColor;
    onChange: (color: HsvColor) => void;
    class?: string;
};
export function HueSwatch(props: HueSwatchProps) {
    const [element, setElement] = createSignal<HTMLDivElement>();
    const [pos, interacting] = useElementRelativeMouse(element, (data) => {
        const x = clamp(data.x / data.elementWidth, 0, 1);
        console.log(data, x);
        // const y = clamp(data.y / data.elementHeight, 0, 1);

        props.onChange({
            ...props.color,
            h: x * 360,
        });
    });
    return (
        <div
            ref={setElement}
            class={clsx('relative', props.class)}
            data-help="1"
            style={{
                'background-image':
                    'linear-gradient(90deg,red 0,#ff0 17%,#0f0 33%,#0ff 50%,#00f 67%,#f0f 83%,red)',
            }}
        >
            <div
                class="absolute w-2 h-full rounded-md border-2 border-white border-solid -translate-x-1/2 pointer-events-none"
                classList={{
                    'w-2': !interacting(),
                    'w-6': interacting(),
                }}
                style={{
                    left: `${pos().x}px`,
                    'background-color': `hsl(${props.color.h}, 100%, 50%)`,
                }}
            />
        </div>
    );
}
