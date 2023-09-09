import { clamp } from '@solid-primitives/utils';
import clsx from 'clsx';
import {
    ComponentProps,
    createEffect,
    createMemo,
    createSignal,
    type JSX,
    mergeProps,
    splitProps,
} from 'solid-js';

type ResizableProps = ComponentProps<'div'> & {
    children: JSX.Element;
    defaultWidth: number;
    handlePosition: 'left' | 'right';
    minWidth: number;
    maxWidth: number;
};
export function Resizable(props: ResizableProps) {
    const [internalProps, divProps] = splitProps(props, [
        'children',
        'class',
        'handlePosition',
        'style',
    ]);
    const [width, setWidth] = createSignal(props.defaultWidth);

    // const [baseStyle, setBaseStyle] = createSignal<JSX.CSSProperties>()
    let baseStyle: JSX.CSSProperties | undefined;
    const [style, setStyle] = createSignal<JSX.CSSProperties | undefined>(
        undefined,
        { equals: false },
    );
    createEffect(() => {
        if (typeof props.style === 'string') return;
        else if (props.style) baseStyle = props.style;
        else baseStyle = {};
        baseStyle['flex-basis'] = `${width()}px`;
        setStyle(baseStyle);
    });
    createEffect(() => {
        if (baseStyle) {
            baseStyle['flex-basis'] = `${width()}px`;
            setStyle(baseStyle);
        }
    });

    let handleEl: HTMLDivElement | undefined;

    let startWidth = 0;
    let startX = 0;
    const handlePointerDown = (e: PointerEvent) => {
        if (!handleEl) return;
        startWidth = width();
        startX = e.clientX;
        document.body.addEventListener('pointermove', handlePointerMove);
        document.body.addEventListener('pointerup', handlePointerUp);
    };

    const handlePointerMove = (e: PointerEvent) => {
        e.preventDefault();
        e.stopPropagation();
        const diff =
            props.handlePosition === 'right'
                ? e.clientX - startX
                : startX - e.clientX;
        const rawWidth = startWidth + diff;
        const newWidth = clamp(rawWidth, props.minWidth, props.maxWidth);
        setWidth(newWidth);
    };

    const handlePointerUp = (e: PointerEvent) => {
        e.preventDefault();
        e.stopPropagation();
        document.body.removeEventListener('pointermove', handlePointerMove);
        document.body.removeEventListener('pointerup', handlePointerUp);
    };

    return (
        <div
            {...divProps}
            class={clsx(
                'relative flex-grow-0 flex-shrink-0',
                internalProps.class,
            )}
            style={style()}
        >
            {internalProps.children}
            <div
                ref={handleEl}
                class="absolute top-0 bottom-0 z-50 w-2 h-full cursor-ew-resize bg-transparent hover:bg-orange-600 hover:bg-opacity-20 ResizableHandle"
                classList={{
                    '-left-1': props.handlePosition === 'left',
                    '-right-1': props.handlePosition === 'right',
                }}
                onPointerDown={handlePointerDown}
            />
        </div>
    );
}
