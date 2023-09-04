import { Point } from "@pixi/core";
import {
    createExclusiveStateMachine,
    CreateExclusiveStateMachineOptions,
    t,
} from "../../utils/fsm";
import { Cursor, ToolHandler } from "../toolStore";

export type ToolInputs = {
    "pointer1-down": {
        position: Point;
        screenPosition: Point;
    };
    "pointer1-move": {
        position: Point;
        downPosition?: Point;
        screenPosition: Point;
        screenDownPosition?: Point;
    };
    "pointer1-up": {
        downPosition: Point;
        position: Point;
        screenPosition: Point;
        screenDownPosition: Point;
    };
    "pointer1-click": {
        position: Point;
        screenPosition: Point;
    };
    "pointer1-doubleclick": {
        position: Point;
        screenPosition: Point;
    };
    "pointer1-dragstart": {
        downPosition: Point;
        position: Point;
    };
    "pointer1-dragmove": {
        downPosition: Point;
        position: Point;
        screenDownPosition: Point;
        screenPosition: Point;
    };
    "pointer1-dragend": {
        downPosition: Point;
        position: Point;
        screenDownPosition: Point;
        screenPosition: Point;
    };
    "pointer3-down": {
        position: Point;
        screenPosition: Point;
    };
    "pointer3-move": {
        position: Point;
        downPosition?: Point;
        screenPosition: Point;
        screenDownPosition?: Point;
    };
    "pointer3-up": {
        downPosition: Point;
        position: Point;
        screenPosition: Point;
        screenDownPosition: Point;
    };
    "keypress": {
        key: string;
    };
    "keydown": {
        key: string;
        keys: Set<string>;
    };
    "keyup": {
        key: string;
        keys: Set<string>;
    };
};

export type ToolInputMessage<
    K extends keyof ToolInputs = keyof ToolInputs,
    M extends ToolInputs[K] = ToolInputs[K],
> = {
    type: K;
    data: M;
};

export const ViewportStates = {
    Blocked: Symbol("Blocked"),
    Default: Symbol("Default"),
    CanPan: Symbol("CanPan"),
    Panning: Symbol("Panning"),
    PanningWithoutSpace: Symbol("PanningWithoutSpace"),
};
type ViewportStatesType = typeof ViewportStates[keyof typeof ViewportStates];
export const ViewportEvents = {
    Block: Symbol("Block"),
    Unblock: Symbol("Unblock"),
    SpaceDown: Symbol("SpaceDown"),
    SpaceUp: Symbol("SpaceUp"),
    PanButtonDown: Symbol("PanButtonDown"),
    PanButtonUp: Symbol("PanButtonUp"),
    PointerDown: Symbol("PointerDown"),
    PointerUp: Symbol("PointerUp"),
};
type ViewportEventsType = typeof ViewportEvents[keyof typeof ViewportEvents];

export const createViewportStateMachine = (
    dispatch: ToolHandler,
    options: Partial<
        CreateExclusiveStateMachineOptions<ViewportStatesType, ViewportEventsType>
    >,
) => {
    const transitions = [
        t(
            ViewportStates.Default,
            ViewportEvents.SpaceDown,
            ViewportStates.CanPan,
            () => {
                dispatch("tool:push-cursor", Cursor.Grab);
            },
        ),
        t(
            ViewportStates.CanPan,
            ViewportEvents.PointerDown,
            ViewportStates.Panning,
            () => dispatch("tool:push-cursor", Cursor.Grabbing),
        ),
        t(
            ViewportStates.CanPan,
            ViewportEvents.SpaceUp,
            ViewportStates.Default,
            () => {
                dispatch("tool:clear-cursor", Cursor.Grab);
            },
        ),
        t(
            ViewportStates.Default,
            ViewportEvents.PanButtonDown,
            ViewportStates.Panning,
            () => {
                dispatch('tool:push-cursor', Cursor.Grabbing);
            }
        ),
        t(
            ViewportStates.Panning,
            ViewportEvents.PanButtonUp,
            ViewportStates.Default,
            () => {
                dispatch("tool:clear-cursor", [Cursor.Grabbing, Cursor.Grab]);
            }
        ),
        t(
            ViewportStates.Panning,
            ViewportEvents.SpaceUp,
            ViewportStates.PanningWithoutSpace,
        ),
        t(
            ViewportStates.Panning,
            ViewportEvents.PointerUp,
            ViewportStates.CanPan,
            () => dispatch("tool:clear-cursor", Cursor.Grabbing),
        ),
        t(
            ViewportStates.PanningWithoutSpace,
            ViewportEvents.PointerUp,
            ViewportStates.Default,
            () => {
                dispatch("tool:clear-cursor", [Cursor.Grabbing, Cursor.Grab]);
            },
        ),
    ];
    return createExclusiveStateMachine(ViewportStates.Default, transitions, {
        ...(options ?? {}),
        exclusiveStates: [
            ViewportStates.CanPan,
            ViewportStates.Panning,
            ViewportStates.PanningWithoutSpace,
        ],
        onBlock() {
            dispatch("tool:clear-cursor", [Cursor.Grab, Cursor.Grabbing]);
        },
    });
};
