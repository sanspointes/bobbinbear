import { EventBoundary } from '@pixi/events';
import { Accessor, createEffect, sharedConfig } from 'solid-js';
import { SolixiState } from '@bearbroidery/solixi';
import { Uuid, newUuid, uuid } from '@/utils/uuid';
import {
    EMB_STATE_DEFAULTS,
    EmbBase,
    EmbState,
    EmbVector,
    NodeUtils,
} from '@/emb-objects';
import {
    ToolInputMessage,
    ToolInputs,
    ViewportEvents,
    createViewportStateMachine,
} from './shared';
import { AllMessages, BaseStore, GeneralHandler, generateStore } from '..';
import { InputModel } from '../inputStore';
import { SceneModel } from '../sceneStore';
import { t } from 'typescript-fsm';
import { createExclusiveStateMachine } from '@/utils/fsm';
import { hslFromRgb } from '@/utils/color';
import { VectorShape } from '@/emb-objects/vec-seg';
import { CreateObjectCommand, SetSceneObjectFieldCommand } from '../commands';
import { MultiCommand } from '../commands/shared';
import { EmbText } from '@/emb-objects/text';

export const TextEvents = {
    Hover: Symbol('s-Hover'),
    Unhover: Symbol('s-Unhover'),
    PointerDown: Symbol('s-Pointerdown'),
    PointerUp: Symbol('s-Pointerup'),
    DragStart: Symbol('s-Dragstart'),
    DoubleClick: Symbol('s-Doubleclick'),
    DragMove: Symbol('s-Dragmove'),
    DragEnd: Symbol('s-Dragend'),
} as const;

export const TextStates = {
    Default: Symbol('s-Default'),
    Hoverring: Symbol('s-Hoverring'),
    Moving: Symbol('s-Moving'),
    PendingCreateType: Symbol('s-CreatingNew'),
    CreatingLineTo: Symbol('s-CreatingLineTo'),
    Default: Symbol('s-CreatingBezierTo'),
    Pening: Symbol('s-Pening'),
} as const;

export type TextToolMessage = {
    activate: void;
    deactivate: void;
    input: ToolInputMessage;
};
export type TextToolModel = {
    state: Accessor<(typeof TextStates)[keyof typeof TextStates]>;
};

export type TextToolStore = BaseStore<TextToolModel, TextToolMessage>;

export const createTextToolStore = (
    dispatch: GeneralHandler<AllMessages>,
    solixi: Accessor<SolixiState | undefined>,
    inputModel: InputModel,
    sceneModel: SceneModel,
) => {
    let boundary: EventBoundary | undefined;
    createEffect(() => {
        const sxi = solixi();
        if (sxi) {
            boundary = sxi.boundary;
        } else {
            boundary = undefined;
        }
    });
    // Internal State
    let currHover: Uuid<EmbBase & EmbState> | undefined;
    let currentVectorId: Uuid<EmbText & EmbState> | undefined;
    // const offset = new Point();
    // let newPosition: Point | undefined;

    let createCommand: CreateObjectCommand<EmbText & EmbState> | undefined;
    // Viewport FSM
    const {
        block: vpBlock,
        unblock: vpUnblock,
        dispatch: vpDispatch,
        can: vpCan,
    } = createViewportStateMachine(dispatch, {
        onExclusive() {
            sBlock();
        },
        onNonExclusive() {
            sUnblock();
        },
    });

    // FSM definition
    const transitions = [
        t(TextStates.Default, TextEvents.Hover, TextStates.Hoverring),
        t(TextStates.Hoverring, TextEvents.Unhover, TextStates.Default),
        t(
            TextStates.Default,
            TextEvents.PointerDown,
            TextStates.PendingCreateType,
            (e: ToolInputs['pointer1-down']) => {
                currentVectorId = newUuid<EmbText & EmbState>();
                const newVector: EmbText & EmbState = {
                    ...EMB_STATE_DEFAULTS,
                    id: currentVectorId,
                    type: 'text',
                    name: 'Text',
                    position: e.position,
                    parent: uuid('root'),
                    children: [],
                    value: 'My Text',
                    width: 150,
                    height: 30,
                };

                createCommand = new CreateObjectCommand(newVector);

                const setWidthCmd = new SetSceneObjectFieldCommand<
                    EmbText,
                    keyof EmbText
                >(currentVectorId, 'width', 150);
                setWidthCmd.final = false;

                const setHeightCmd = new SetSceneObjectFieldCommand<
                    EmbText,
                    keyof EmbText
                >(currentVectorId, 'height', 30);
                setHeightCmd.final = false;

                const cmd = new MultiCommand(
                    createCommand,
                    setWidthCmd,
                    setHeightCmd,
                );
                cmd.name = 'Creating Text';
                cmd.final = false;

                dispatch('scene:do-command', cmd);
            },
        ),
        t(
            TextStates.PendingCreateType,
            TextEvents.PointerUp,
            TextStates.Default,
            (e: ToolInputs['pointer1-down']) => {
                currentVectorId = newUuid<EmbText & EmbState>();
                const newVector: EmbText & EmbState = {
                    ...EMB_STATE_DEFAULTS,
                    id: currentVectorId,
                    type: 'text',
                    name: 'Text',
                    position: e.position,
                    parent: uuid('root'),
                    children: [],
                    value: 'My Text',
                    width: 150,
                    height: 30,
                };

                createCommand = new CreateObjectCommand(newVector);

                const setWidthCmd = new SetSceneObjectFieldCommand<
                    EmbText,
                    keyof EmbText
                >(currentVectorId, 'width', 150);

                const setHeightCmd = new SetSceneObjectFieldCommand<
                    EmbText,
                    keyof EmbText
                >(currentVectorId, 'height', 30);

                const cmd = new MultiCommand(
                    createCommand,
                    setWidthCmd,
                    setHeightCmd,
                );
                cmd.name = 'Creating text';

                dispatch('scene:do-command', cmd);
            },
        ),
        t(TextStates.Pening, TextEvents.DragEnd, TextStates.Default, () => {}),
    ];

    const {
        state,
        block: sBlock,
        unblock: sUnblock,
        can: sCan,
        dispatch: sDispatch,
    } = createExclusiveStateMachine(TextStates.Default, transitions, {
        exclusiveStates: [TextStates.Pening, TextStates.Moving],
        onExclusive: () => {
            vpBlock();
        },
        onNonExclusive: () => {
            vpUnblock();
        },
    });

    sUnblock();
    vpUnblock();

    const model: TextToolModel = {
        state: state,
    };

    const result = generateStore<TextToolModel, TextToolMessage>(model, {
        input: (_1, _2, msg) => {
            switch (msg.type) {
                case 'pointer1-move':
                    {
                        if (boundary) {
                            const data =
                                msg.data as ToolInputs['pointer1-move'];
                            const result = boundary.hitTest(
                                data.position.x,
                                data.position.y,
                            );
                            if (result && result.id) {
                                if (currHover && result.id !== currHover) {
                                    if (sCan(TextEvents.Unhover)) {
                                        sDispatch(TextEvents.Unhover);
                                        dispatch('scene:unhover', currHover);
                                    }
                                }
                                if (sCan(TextEvents.Hover)) {
                                    sDispatch(TextEvents.Hover);
                                    dispatch('scene:hover', result.id);
                                    currHover = result.id;
                                }
                            } else if (sCan(TextEvents.Unhover)) {
                                // console.log("Unhovering");
                                sDispatch(TextEvents.Unhover);
                                if (currHover)
                                    dispatch('scene:unhover', currHover);
                                currHover = undefined;
                            }
                        }
                    }
                    break;
                case 'pointer1-down':
                    {
                        if (vpCan(ViewportEvents.PointerDown)) {
                            vpDispatch(ViewportEvents.PointerDown);
                        }
                        if (sCan(TextEvents.PointerDown)) {
                            sDispatch(TextEvents.PointerDown, msg.data);
                        }
                    }
                    break;
                case 'pointer1-doubleclick':
                    {
                        if (sCan(TextEvents.DoubleClick)) {
                            sDispatch(TextEvents.DoubleClick, msg.data);
                        }
                    }
                    break;
                case 'pointer1-up':
                    {
                        if (vpCan(ViewportEvents.PointerUp)) {
                            vpDispatch(ViewportEvents.PointerUp, msg.data);
                        }
                        if (sCan(TextEvents.PointerUp))
                            sDispatch(TextEvents.PointerUp, msg.data);
                    }
                    break;
                case 'pointer1-dragstart':
                    {
                        if (sCan(TextEvents.DragStart))
                            sDispatch(TextEvents.DragStart, msg.data);
                    }
                    break;
                case 'pointer1-dragmove':
                    {
                        if (sCan(TextEvents.DragMove))
                            sDispatch(TextEvents.DragMove, msg.data);
                    }
                    break;
                case 'pointer1-dragend':
                    {
                        if (sCan(TextEvents.DragEnd))
                            sDispatch(TextEvents.DragEnd, msg.data);
                    }
                    break;
                case 'keydown':
                    {
                        const data = msg.data as ToolInputs['keydown'];
                        if (
                            data.key === ' ' &&
                            vpCan(ViewportEvents.SpaceDown)
                        ) {
                            console.log('dispatch space down');
                            vpDispatch(ViewportEvents.SpaceDown);
                        }
                    }
                    break;
                case 'keyup': {
                    const data = msg.data as ToolInputs['keyup'];
                    if (data.key === ' ' && vpCan(ViewportEvents.SpaceUp)) {
                        console.log('dispatch space up');
                        vpDispatch(ViewportEvents.SpaceUp);
                    }
                }
            }
        },
        activate: (_1, _2) => {
            console.log('Text tool activated');
            vpUnblock();
            sUnblock();
        },
        deactivate: (_1, _2) => {
            console.log('Text tool deactivated');
            vpBlock();
            sBlock();
        },
    });

    return result;
};
