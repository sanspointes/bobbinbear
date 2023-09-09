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
import { createEllipseGraphicsCommands } from '../../utils/graphics';
import { SetSceneObjectFieldCommand, CreateObjectCommand } from '../commands';
import { Point } from '@pixi/core';
import { MultiCommand } from '../commands/shared';
import { EmbObject, EmbState, EmbVector } from '../../emb-objects';
import { hslFromRgb } from '../../utils/color';

export const EllipseEvents = {
    PointerDown: Symbol('b-Pointerdown'),
    PointerUp: Symbol('b-Pointerup'),
    DragStart: Symbol('Dragstart'),
    DragMove: Symbol('b-Dragmove'),
    DragEnd: Symbol('b-Dragend'),
} as const;
export const EllipseStates = {
    Default: Symbol('b-Default'),
    Down: Symbol('b-Down'),
    Building: Symbol('b-Building'),
} as const;

export type EllipseToolMessage = {
    activate: void;
    deactivate: void;
    input: ToolInputMessage;
};
export type EllipseToolModel = {
    state: Accessor<(typeof EllipseStates)[keyof typeof EllipseStates]>;
};

export type EllipseToolStore = BaseStore<EllipseToolModel, EllipseToolMessage>;

export const createEllipseToolStore = (
    dispatch: GeneralHandler<AllMessages>,
) => {
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
        t(EllipseStates.Default, EllipseEvents.PointerDown, EllipseStates.Down),
        t(EllipseStates.Down, EllipseEvents.PointerUp, EllipseStates.Default),
        t(
            EllipseStates.Down,
            EllipseEvents.DragStart,
            EllipseStates.Building,
            (e: ToolInputs['pointer1-dragstart'], parent?: EmbObject) => {
                currentlyBuildingId = newUuid();
                const currentShape = createEllipseGraphicsCommands(0, 0);

                createCommand = new CreateObjectCommand({
                    type: 'vector',
                    name: 'Ellipse',
                    visible: true,
                    position: e.position,
                    id: currentlyBuildingId,
                    parent: parent ? parent.id : uuid('root'),
                    children: [],
                    selected: false,
                    locked: false,
                    inspectingRoot: undefined,
                    shallowLocked: false,
                    hovered: false,
                    shape: currentShape,
                    close: true,
                    inspecting: false,
                    fill: {
                        color: hslFromRgb({ r: 200, g: 200, b: 200 }),
                        alpha: 1,
                    },
                    line: {
                        width: 1,
                        color: hslFromRgb({ r: 0, g: 0, b: 0 }),
                        alpha: 1,
                    },
                });

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
                cmd.name = 'Creating Ellipse';
                cmd.final = false;
                dispatch('scene:do-command', cmd);
            },
        ),
        t(
            EllipseStates.Building,
            EllipseEvents.DragMove,
            EllipseStates.Building,
            (e: ToolInputs['pointer1-dragmove']) => {
                if (!createCommand || !currentlyBuildingId) {
                    throw new Error(
                        'EllipseTool: DragMove event but no Ellipse currently being built.',
                    );
                }
                if (!e.downPosition) {
                    throw new Error(
                        'EllipseTool: DragMove event but no downPosition provided by input store.',
                    );
                }
                const { position: pos, downPosition: dPos } = e;

                const minx = Math.min(pos.x, dPos.x);
                const miny = Math.min(pos.y, dPos.y);
                const position = new Point(minx, miny);

                const diffx = pos.x - dPos.x;
                const diffy = pos.y - dPos.y;
                const currentShape = createEllipseGraphicsCommands(
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
                cmd.name = 'Updating Ellipse';
                cmd.final = false;
                dispatch('scene:do-command', cmd);
            },
        ),
        t(
            EllipseStates.Building,
            EllipseEvents.DragEnd,
            EllipseStates.Default,
            (e: ToolInputs['pointer1-dragend']) => {
                if (!createCommand || !currentlyBuildingId) {
                    throw new Error(
                        'EllipseTool: DragMove event but no Ellipse currently being built.',
                    );
                }
                if (!e.downPosition) {
                    throw new Error(
                        'EllipseTool: DragMove event but no downPosition provided by input store.',
                    );
                }
                const { position: pos, downPosition: dPos } = e;

                const minx = Math.min(pos.x, dPos.x);
                const miny = Math.min(pos.y, dPos.y);
                const position = new Point(minx, miny);

                const diffx = pos.x - dPos.x;
                const diffy = pos.y - dPos.y;
                const currentShape = createEllipseGraphicsCommands(
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
                cmd.name = 'Creating Ellipse Finished';
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
    } = createExclusiveStateMachine(EllipseStates.Default, transitions, {
        exclusiveStates: [EllipseStates.Down, EllipseStates.Building],
        onExclusive: () => {
            vpBlock();
        },
        onNonExclusive: () => {
            vpUnblock();
        },
    });

    const model: EllipseToolModel = {
        state: bState,
    };

    const result = generateStore<EllipseToolModel, EllipseToolMessage>(model, {
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
                        //       bDispatch(EllipseEvents.Hover);
                        //       dispatch("scene:hover", data.id);
                        //       currHover = data.id;
                        //     } else if (!data && bCan(EllipseEvents.Unhover)) {
                        //       // console.log("Unhovering");
                        //       bDispatch(EllipseEvents.Unhover);
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
                        if (bCan(EllipseEvents.PointerDown)) {
                            bDispatch(EllipseEvents.PointerDown, undefined);
                        }
                    }
                    break;
                case 'pointer1-up':
                    {
                        if (vpCan(ViewportEvents.PointerUp)) {
                            vpDispatch(ViewportEvents.PointerUp);
                        }
                        if (bCan(EllipseEvents.PointerUp)) {
                            bDispatch(EllipseEvents.PointerUp, msg.data);
                        }
                    }
                    break;
                case 'pointer1-dragstart':
                    {
                        if (bCan(EllipseEvents.DragStart)) {
                            bDispatch(EllipseEvents.DragStart, msg.data);
                        }
                    }
                    break;
                case 'pointer1-dragmove':
                    {
                        if (bCan(EllipseEvents.DragMove)) {
                            bDispatch(EllipseEvents.DragMove, msg.data);
                        }
                    }
                    break;
                case 'pointer1-dragend':
                    {
                        if (bCan(EllipseEvents.DragEnd))
                            bDispatch(EllipseEvents.DragEnd, msg.data);
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
