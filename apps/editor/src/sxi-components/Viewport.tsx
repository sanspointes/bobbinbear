import { Solixi, useFrame } from '@bearbroidery/solixi';
import {
    Clamp,
    Drag,
    IClampOptions,
    IViewportOptions,
    Pinch,
    Viewport as PixiViewport,
    Wheel,
} from 'pixi-viewport';
import {
    createEffect,
    type JSX,
    on,
    onMount,
    untrack,
    useContext,
} from 'solid-js';
import { createMemo } from 'solid-js';
import { Cursor } from '../store/toolStore';
import { AppContext } from '../store';
import { Container } from '@pixi/display';
import { createEventListener } from '@solid-primitives/event-listener';
import { FederatedPointerEvent } from '@pixi/events';
import { Point } from '@pixi/core';
import { mapLinear } from '../utils/math';

const PViewport = Solixi.wrapConstructable(PixiViewport, {
    // @ts-expect-error; Parent must be constainer.
    attach: (_, parent: Container, object) => {
        parent.addChild(object);
        return () => parent.removeChild(object);
    },
    defaultArgs: (ctx) =>
        [
            {
                events: ctx.app.renderer.events,
                ticker: ctx.ticker,
            },
        ] as [options: IViewportOptions],
    extraProps: {
        pinch: (_1, _2, object, enable: boolean) => {
            const pinchPlugin = new Pinch(object);
            if (enable) {
                object.plugins.add('pinch', pinchPlugin);
            } else {
                object.plugins.remove('pinch');
            }
            return () => {
                if (object.plugins.list.includes(pinchPlugin)) {
                    object.plugins.remove('pinch');
                }
            };
        },
        drag: (_1, _2, object, enable: boolean) => {
            const dragPlugin = new Drag(object);
            if (enable) {
                object.plugins.add('drag', dragPlugin);
            } else {
                object.plugins.remove('drag');
            }
            return () => {
                if (object.plugins.list.includes(dragPlugin)) {
                    object.plugins.remove('drag');
                }
            };
        },
        clamp: (_1, _2, object, options: false | IClampOptions) => {
            let clampPlugin: Clamp | undefined;
            if (options) {
                const clampPlugin = new Clamp(object, options);
                object.plugins.add('clamp', clampPlugin);
            } else {
                object.plugins.remove('clamp');
            }
            return () => {
                if (clampPlugin && object.plugins.list.includes(clampPlugin)) {
                    object.plugins.remove('clamp');
                }
            };
        },
        wheel: (_1, _2, object, enable: boolean) => {
            const wheelPlugin = new Wheel(object);
            if (enable) {
                object.plugins.add('wheel', wheelPlugin);
            } else {
                object.plugins.remove('wheel');
            }
            return () => {
                if (object.plugins.list.includes(wheelPlugin)) {
                    object.plugins.remove('wheel');
                }
            };
        },
    },
});

type ViewportProps = {
    children: JSX.Element;
};
export const Viewport = (props: ViewportProps) => {
    const { toolStore, viewportStore, dispatch } = useContext(AppContext);
    let viewportEl: PixiViewport | undefined;
    const panPaused = createMemo(() => {
        const { Grab, Grabbing } = Cursor;
        const cursor = toolStore.currentCursor;
        return cursor !== Grab && cursor !== Grabbing;
    });
    createEffect(
        on(panPaused, (panPaused) => {
            if (viewportEl) {
                if (panPaused) {
                    viewportEl.plugins.plugins['drag']?.pause();
                } else {
                    viewportEl.plugins.plugins['drag']?.resume();
                }
            }
        }),
    );

    // Update viewport store position each frame.
    useFrame(() => {
        untrack(() => {
            if (viewportEl !== undefined) {
                if (!viewportStore.position.equals(viewportEl.position)) {
                    dispatch('viewport:move-to', viewportEl.position);
                }

                const { left, right, top, bottom } = viewportEl;
                if (
                    left !== viewportStore.left ||
                    top !== viewportStore.top ||
                    right !== viewportStore.right ||
                    bottom !== viewportStore.bottom
                ) {
                    dispatch('viewport:set-bounds', {
                        left,
                        right,
                        top,
                        bottom,
                    });
                }
            }
        });
    });

    onMount(() => {
        if (viewportEl) {
            let screenDownPosition: Point | undefined;
            let downPosition: Point | undefined;
            createEventListener(viewportEl, 'pointerdown', (event) => {
                const ev = event as unknown as FederatedPointerEvent;

                const { left, right, top, bottom, screenWidth, screenHeight } =
                    viewportEl!;
                const x = mapLinear(ev.global.x, 0, screenWidth, left, right);
                const y = mapLinear(ev.global.y, 0, screenHeight, top, bottom);
                const position = new Point(x, y);

                screenDownPosition = ev.global.clone();
                downPosition = position.clone();
                dispatch('input:pointerdown', {
                    screenPosition: ev.global.clone(),
                    position,
                });
            });

            createEventListener(viewportEl, 'pointermove', (event) => {
                const ev = event as unknown as FederatedPointerEvent;

                const { left, right, top, bottom, screenWidth, screenHeight } =
                    viewportEl!;
                const x = mapLinear(ev.global.x, 0, screenWidth, left, right);
                const y = mapLinear(ev.global.y, 0, screenHeight, top, bottom);

                const position = new Point(x, y);
                dispatch('input:pointermove', {
                    screenDownPosition,
                    downPosition,
                    screenPosition: ev.global.clone(),
                    position,
                });
            });

            createEventListener(viewportEl, 'pointerup', (event) => {
                const ev = event as unknown as FederatedPointerEvent;

                const { left, right, top, bottom, screenWidth, screenHeight } =
                    viewportEl!;
                const x = mapLinear(ev.global.x, 0, screenWidth, left, right);
                const y = mapLinear(ev.global.y, 0, screenHeight, top, bottom);

                const position = new Point(x, y);
                dispatch('input:pointerup', {
                    screenDownPosition,
                    downPosition,
                    screenPosition: ev.global.clone(),
                    position,
                });
            });
        }
    });

    return (
        <PViewport
            ref={viewportEl}
            drag
            wheel
            position={viewportStore.position}
            children={props.children}
            sortableChildren={true}
        />
    );
};
