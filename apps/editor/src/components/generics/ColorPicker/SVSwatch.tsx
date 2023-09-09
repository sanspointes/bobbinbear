import { createMemo, createSignal } from 'solid-js';
import clsx from 'clsx';
import { useElementRelativeMouse } from '../../../composables/useElementRelativeMouse';
import { clamp } from '../../../utils/math';
import { HsvColor, hslFromHsv, hslToCssString } from '../../../utils/color';

type SVSwatchProps = {
    color: HsvColor;
    onChange: (color: HsvColor) => void;
    class?: string;
};
export function SVSwatch(props: SVSwatchProps) {
    const hslStyle = createMemo(() =>
        hslToCssString(hslFromHsv({ ...props.color })),
    );

    const [element, setElement] = createSignal<HTMLDivElement>();
    const [pos, interacting] = useElementRelativeMouse(element, (data) => {
        const x = clamp(data.x / data.elementWidth, 0, 1);
        const y = clamp(data.y / data.elementHeight, 0, 1);

        props.onChange({
            ...props.color,
            s: x * 100,
            v: 100 - y * 100,
        });
    });
    return (
        <div
            ref={setElement}
            class={clsx('relative', props.class)}
            style={{
                'background-color': `hsl(${props.color.h}, 100%, 50%)`,
                'background-image':
                    'linear-gradient(0deg,#000,transparent),linear-gradient(90deg,#fff,hsla(0,0%,100%,0))',
            }}
        >
            <div
                class="absolute rounded-full border-2 border-white border-solid -translate-x-1/2 -translate-y-1/2 pointer-events-none"
                classList={{
                    'w-4 h-4': !interacting(),
                    'w-8 h-8': interacting(),
                }}
                data-color={hslStyle()}
                style={{
                    'background-color': hslStyle(),
                    left: `${(props.color.s / 100) * pos().elementWidth}px`,
                    top: `${
                        pos().elementHeight -
                        (props.color.v / 100) * pos().elementHeight
                    }px`,
                }}
            />
        </div>
    );
}
