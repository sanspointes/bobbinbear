import { Point } from '@pixi/core';
import { AllMessages, GeneralHandler, generateStore } from '.';
import { produce } from 'solid-js/store';
import { createEffect, on } from 'solid-js';
import { pointDistance } from '../utils/point';
import { ToolInputs } from './tools/shared';
import { createEventListener } from '@solid-primitives/event-listener';

export type InputMessages = {
    'input:set-source': {
        pointer?: HTMLElement;
        keys?: HTMLElement;
    };
    'input:pointerdown': {
        screenPosition: Point;
        position: Point;
    };
    'input:pointermove': {
        screenPosition: Point;
        position: Point;
        screenDownPosition: Point;
        downPosition: Point;
    };
    'input:pointerup': {
        screenPosition: Point;
        position: Point;
        screenDownPosition: Point;
        downPosition: Point;
    };
    'input:keydown': {
        key: KeyboardEvent['key'];
    };
    'input:keypress': {
        key: KeyboardEvent['key'];
    };
    'input:keyup': {
        key: KeyboardEvent['key'];
    };
};

export type InputToolSettings = {
    dragThreshold: number;
    doubleClickTime: number;
};

export type InputModel = {
    settings: InputToolSettings;

    pointerSource: HTMLElement | undefined;
    keySource: HTMLElement | undefined;
    isDragging: boolean;
    keys: Set<string>;

    position: Point;
    screenPosition: Point;
    downPosition?: Point;
    screenDownPosition?: Point;
};

const makeToolInputResponse = <
    K extends keyof ToolInputs,
    M extends ToolInputs[K],
>(
    type: K,
    data: M,
): AllMessages['tool:input'] => {
    return {
        type,
        data,
    };
};

export const createInputStore = (dispatch: GeneralHandler<AllMessages>) => {
    // Internal state
    let doubleClickTimeout: number | undefined;

    const startDoubleClickTimeout = () => {
        window.clearTimeout(doubleClickTimeout);
        doubleClickTimeout = window.setTimeout(() => {
            doubleClickTimeout = undefined;
        }, store.store.settings.doubleClickTime);
    };
    const shouldDoubleClick = () => {
        const result = doubleClickTimeout !== undefined;
        window.clearTimeout(doubleClickTimeout);
        doubleClickTimeout = undefined;
        return result;
    };

    const store = generateStore<InputModel, InputMessages>(
        {
            settings: {
                dragThreshold: 2,
                doubleClickTime: 300,
            },
            pointerSource: undefined,
            keySource: undefined,
            isDragging: false,
            keys: new Set(),
            screenPosition: new Point(),
            position: new Point(),
            downPosition: undefined,
            screenDownPosition: undefined,
        },
        {
            'input:set-source': (_, set, message) => {
                set(
                    produce((store) => {
                        if (message.pointer) {
                            store.pointerSource = message.pointer;
                        }
                        if (message.keys) {
                            store.keySource = message.keys;
                        }
                    }),
                );
            },

            'input:pointerdown': (_, set, message, respond) => {
                set(
                    produce((store) => {
                        store.downPosition = message.position;
                        store.screenDownPosition = message.screenPosition;
                    }),
                );
                respond!(
                    'tool:input',
                    makeToolInputResponse('pointer1-down', {
                        screenPosition: message.screenPosition,
                        position: message.position,
                    }),
                );
            },

            'input:pointermove': (store, set, message, respond) => {
                set(
                    produce((store) => {
                        store.position = message.position;
                        store.screenPosition = message.screenPosition;
                    }),
                );

                const dragDistance =
                    store.downPosition &&
                    pointDistance(store.downPosition, message.position);
                if (store.isDragging) {
                    respond!(
                        'tool:input',
                        makeToolInputResponse('pointer1-dragmove', {
                            screenPosition: message.screenPosition,
                            position: message.position,
                            screenDownPosition: message.screenDownPosition,
                            downPosition: store.downPosition as Point,
                        }),
                    );
                } else if (
                    dragDistance &&
                    dragDistance > store.settings.dragThreshold
                ) {
                    set('isDragging', true);
                    respond!(
                        'tool:input',
                        makeToolInputResponse('pointer1-dragstart', {
                            screenPosition: message.screenPosition,
                            position: message.position,
                            screenDownPosition: message.screenDownPosition,
                            downPosition: store.downPosition as Point,
                        }),
                    );
                } else {
                    respond!(
                        'tool:input',
                        makeToolInputResponse('pointer1-move', {
                            screenPosition: message.screenPosition,
                            position: message.position,
                            screenDownPosition: message.screenDownPosition,
                            downPosition: store.downPosition as Point,
                        }),
                    );
                }
            },

            'input:pointerup': (store, set, message, respond) => {
                if (store.isDragging) {
                    respond!(
                        'tool:input',
                        makeToolInputResponse('pointer1-dragend', {
                            screenPosition: message.screenPosition,
                            position: message.position,
                            screenDownPosition: message.screenDownPosition,
                            downPosition: store.downPosition as Point,
                        }),
                    );
                    set('isDragging', false);
                } else {
                    if (shouldDoubleClick()) {
                        respond!(
                            'tool:input',
                            makeToolInputResponse('pointer1-doubleclick', {
                                screenPosition: message.screenPosition,
                                position: message.position,
                            }),
                        );
                    } else {
                        startDoubleClickTimeout();
                        respond!(
                            'tool:input',
                            makeToolInputResponse('pointer1-click', {
                                screenPosition: message.screenPosition,
                                position: message.position,
                            }),
                        );
                    }
                }
                respond!(
                    'tool:input',
                    makeToolInputResponse('pointer1-up', {
                        position: message.position,
                        downPosition: store.downPosition as Point,
                    }),
                );
                set(
                    produce((store) => {
                        store.downPosition = undefined;
                        store.screenDownPosition = undefined;
                    }),
                );
            },

            'input:keypress': (_1, _2, message, respond) => {
                respond!(
                    'tool:input',
                    makeToolInputResponse('keypress', {
                        key: message.key,
                    }),
                );
            },

            'input:keydown': (store, set, message, respond) => {
                if (!store.keys.has(message.key)) {
                    set(produce((store) => store.keys.add(message.key)));
                    respond!(
                        'tool:input',
                        makeToolInputResponse('keydown', {
                            key: message.key,
                            keys: store.keys,
                        }),
                    );
                }
            },

            'input:keyup': (store, set, message, respond) => {
                set(produce((store) => store.keys.delete(message.key)));
                respond!(
                    'tool:input',
                    makeToolInputResponse('keyup', {
                        key: message.key,
                        keys: store.keys,
                    }),
                );
            },
        },
    );

    // Pointer events are handled by the Viewport.tsx class.

    // Bind key events on the key source
    createEffect(
        on(
            () => store.store.keySource,
            (keySource) => {
                if (keySource) {
                    createEventListener(keySource, 'keydown', (e) => {
                        store.handle(
                            'input:keydown',
                            {
                                key: e.key,
                            },
                            dispatch,
                        );
                    });
                    createEventListener(keySource, 'keyup', (e) =>
                        store.handle(
                            'input:keyup',
                            {
                                key: e.key,
                            },
                            dispatch,
                        ),
                    );
                    createEventListener(keySource, 'keypress', (e) =>
                        store.handle(
                            'input:keypress',
                            {
                                key: e.key,
                            },
                            dispatch,
                        ),
                    );
                }
            },
        ),
    );

    return store;
};
