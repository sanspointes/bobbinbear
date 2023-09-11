import { Accessor } from 'solid-js';
import {
    createViewportStateMachine,
    ToolInputMessage,
    ToolInputs,
    ViewportEvents,
} from './shared';
import { AllMessages, BaseStore, GeneralHandler, generateStore } from '..';
import { createExclusiveStateMachine, t } from '../../utils/fsm';
import { newUuid, uuid, Uuid } from '../../utils/uuid';
import { createBoxGraphicsCommands } from '../../utils/graphics';
import { SetSceneObjectFieldCommand, CreateObjectCommand } from '../commands';
import { Point } from '@pixi/core';
import { MultiCommand } from '../commands/shared';
import {
    EMB_STATE_DEFAULTS,
    EmbObject,
    EmbState,
    EmbVector,
} from '../../emb-objects';
import { hslFromRgb } from '../../utils/color';
import { Cursor } from '../toolStore';

export const BoxEvents = {
    PointerDown: Symbol('b-Pointerdown'),
    PointerUp: Symbol('b-Pointerup'),
    DragStart: Symbol('Dragstart'),
    DragMove: Symbol('b-Dragmove'),
    DragEnd: Symbol('b-Dragend'),
} as const;
export const BoxStates = {
    Default: Symbol('b-Default'),
    Down: Symbol('b-Down'),
    Building: Symbol('b-Building'),
} as const;

export type BoxToolMessage = {
    activate: void;
    deactivate: void;
    input: ToolInputMessage;
};
export type BoxToolModel = {
    state: Accessor<(typeof BoxStates)[keyof typeof BoxStates]>;
};

export type BoxToolStore = BaseStore<BoxToolModel, BoxToolMessage>;

export const createBoxToolStore = (dispatch: GeneralHandler<AllMessages>) => {
    //
    // Viewport FSM
    const {
        block: vpBlock,
        unblock: vpUnblock,
        dispatch: vpDispatch,
        can: vpCan,
    } = createViewportStateMachine(dispatch, {
        onExclusive() {
            bBlock();
        },
        onNonExclusive() {
            bUnblock();
        },
    });

    let createCommand: CreateObjectCommand<EmbVector & EmbState> | undefined;
    let currentlyBuildingId: Uuid<EmbVector & EmbState> | undefined;

    const transitions = [
        t(BoxStates.Default, BoxEvents.PointerDown, BoxStates.Down),
        t(BoxStates.Down, BoxEvents.PointerUp, BoxStates.Default),
        t(
            BoxStates.Down,
            BoxEvents.DragStart,
            BoxStates.Building,
            (e: ToolInputs['pointer1-dragstart'], parent?: EmbObject) => {
                dispatch("tool:push-cursor", Cursor.Cross);
                currentlyBuildingId = newUuid();
                const currentShape = createBoxGraphicsCommands(0, 0);

                const newVector: EmbVector & EmbState = {
                    ...EMB_STATE_DEFAULTS,
                    type: 'vector',
                    name: 'Box',
                    position: e.position,
                    id: currentlyBuildingId,
                    parent: parent ? parent.id : uuid('root'),
                    children: [],
                    shape: currentShape,
                    disableMove: false,
                    fill: {
                        color: hslFromRgb({ r: 200, g: 200, b: 200 }),
                        alpha: 1,
                    },
                    line: {
                        width: 1,
                        color: hslFromRgb({ r: 0, g: 0, b: 0 }),
                        alpha: 1,
                    },
                };

                createCommand = new CreateObjectCommand(newVector);

                const setShapeCommand =
                    new SetSceneObjectFieldCommand<EmbVector>(
                        currentlyBuildingId,
                        'shape',
                        currentShape,
                    );
                setShapeCommand.final = false;
                const setPositionCommand =
                    new SetSceneObjectFieldCommand<EmbVector>(
                        currentlyBuildingId,
                        'position',
                        e.position,
                    );
                setPositionCommand.final = false;

                const cmd = new MultiCommand(
                    createCommand,
                    setShapeCommand,
                    setPositionCommand,
                );
                cmd.name = 'Creating Box';
                cmd.final = false;
                dispatch('scene:do-command', cmd);
            },
        ),
        t(
            BoxStates.Building,
            BoxEvents.DragMove,
            BoxStates.Building,
            (e: ToolInputs['pointer1-dragmove']) => {
                if (!createCommand || !currentlyBuildingId) {
                    throw new Error(
                        'boxTool: DragMove event but no box currently being built.',
                    );
                }
                if (!e.downPosition) {
                    throw new Error(
                        'boxTool: DragMove event but no downPosition provided by input store.',
                    );
                }
                const { position: pos, downPosition: dPos } = e;

                const minx = Math.min(pos.x, dPos.x);
                const miny = Math.min(pos.y, dPos.y);
                const position = new Point(minx, miny);

                const diffx = pos.x - dPos.x;
                const diffy = pos.y - dPos.y;
                const currentShape = createBoxGraphicsCommands(
                    Math.abs(diffx),
                    Math.abs(diffy),
                );

                const setShapeCommand = new SetSceneObjectFieldCommand<
                    EmbVector,
                    keyof EmbVector
                >(currentlyBuildingId, 'shape', currentShape);
                setShapeCommand.final = false;
                const setPositionCommand = new SetSceneObjectFieldCommand<
                    EmbVector,
                    keyof EmbVector
                >(currentlyBuildingId, 'position', position);
                setPositionCommand.final = false;

                const cmd = new MultiCommand(
                    createCommand,
                    // @ts-expect-error ; Issues with generic typing of GraphicSceneObject
                    setShapeCommand,
                    setPositionCommand,
                );
                cmd.name = 'Updating Box';
                cmd.final = false;
                dispatch('scene:do-command', cmd);
            },
        ),
        t(
            BoxStates.Building,
            BoxEvents.DragEnd,
            BoxStates.Default,
            (e: ToolInputs['pointer1-dragend']) => {
                dispatch("tool:clear-cursor", Cursor.Cross);
                if (!createCommand || !currentlyBuildingId) {
                    throw new Error(
                        'boxTool: DragMove event but no box currently being built.',
                    );
                }
                if (!e.downPosition) {
                    throw new Error(
                        'boxTool: DragMove event but no downPosition provided by input store.',
                    );
                }
                const { position: pos, downPosition: dPos } = e;

                const minx = Math.min(pos.x, dPos.x);
                const miny = Math.min(pos.y, dPos.y);
                const position = new Point(minx, miny);

                const diffx = pos.x - dPos.x;
                const diffy = pos.y - dPos.y;
                const currentShape = createBoxGraphicsCommands(
                    Math.abs(diffx),
                    Math.abs(diffy),
                );

                const setShapeCommand = new SetSceneObjectFieldCommand<
                    EmbVector,
                    keyof EmbVector
                >(currentlyBuildingId, 'shape', currentShape);
                const setPositionCommand = new SetSceneObjectFieldCommand<
                    EmbVector,
                    keyof EmbVector
                >(currentlyBuildingId, 'position', position);

                const cmd = new MultiCommand(
                    createCommand,
                    setShapeCommand,
                    setPositionCommand,
                );
                cmd.name = 'Creating Box Finished';
                dispatch('scene:do-command', cmd);

                createCommand = undefined;
                currentlyBuildingId = undefined;
            },
        ),
    ];

    const {
        state: bState,
        block: bBlock,
        unblock: bUnblock,
        can: bCan,
        dispatch: bDispatch,
    } = createExclusiveStateMachine(BoxStates.Default, transitions, {
        exclusiveStates: [BoxStates.Down, BoxStates.Building],
        onExclusive: () => {
            vpBlock();
        },
        onNonExclusive: () => {
            vpUnblock();
        },
    });

    const model: BoxToolModel = {
        state: bState,
    };

    const result = generateStore<BoxToolModel, BoxToolMessage>(model, {
        input: (_1, _2, msg) => {
            switch (msg.type) {
                case 'pointer1-move':
                    {
                        // if (boundary) {
                        //   const data = msg.data as ToolInputs["pointer1-move"];
                        //   const result = boundary.hitTest(data.position.x, data.position.y);
                        //   if (result) {
                        //     const data = metadata.get(result);
                        //     if (data && bCan(SelectEvents.Hover)) {
                        //       // console.log("Hovering");
                        //       bDispatch(BoxEvents.Hover);
                        //       dispatch("scene:hover", data.id);
                        //       currHover = data.id;
                        //     } else if (!data && bCan(BoxEvents.Unhover)) {
                        //       // console.log("Unhovering");
                        //       bDispatch(BoxEvents.Unhover);
                        //       if (currHover) dispatch("scene:unhover", currHover);
                        //       currHover = undefined;
                        //     }
                        //   }
                        // }
                    }
                    break;
                case 'pointer1-down':
                    {
                        if (vpCan(ViewportEvents.PointerDown)) {
                            vpDispatch(ViewportEvents.PointerDown);
                        }
                        if (bCan(BoxEvents.PointerDown)) {
                            bDispatch(BoxEvents.PointerDown, undefined);
                        }
                    }
                    break;
                case 'pointer1-up':
                    {
                        if (vpCan(ViewportEvents.PointerUp)) {
                            vpDispatch(ViewportEvents.PointerUp);
                        }
                        if (bCan(BoxEvents.PointerUp)) {
                            bDispatch(BoxEvents.PointerUp, msg.data);
                        }
                    }
                    break;
                case 'pointer1-dragstart':
                    {
                        if (bCan(BoxEvents.DragStart)) {
                            bDispatch(BoxEvents.DragStart, msg.data);
                        }
                    }
                    break;
                case 'pointer1-dragmove':
                    {
                        if (bCan(BoxEvents.DragMove)) {
                            bDispatch(BoxEvents.DragMove, msg.data);
                        }
                    }
                    break;
                case 'pointer1-dragend':
                    {
                        if (bCan(BoxEvents.DragEnd))
                            bDispatch(BoxEvents.DragEnd, msg.data);
                    }
                    break;
                case 'keydown':
                    {
                        const data = msg.data as ToolInputs['keydown'];
                        if (
                            data.key === ' ' &&
                            vpCan(ViewportEvents.SpaceDown)
                        ) {
                            vpDispatch(ViewportEvents.SpaceDown);
                        }
                    }
                    break;
                case 'keyup': {
                    const data = msg.data as ToolInputs['keyup'];
                    if (data.key === ' ' && vpCan(ViewportEvents.SpaceUp)) {
                        vpDispatch(ViewportEvents.SpaceUp);
                    }
                }
            }
        },
        activate: (_1, _2) => {
            vpUnblock();
            bUnblock();
        },
        deactivate: (_1, _2) => {
            vpBlock();
            bBlock();
        },
    });

    return result;
};
