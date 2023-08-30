import { EventBoundary } from "@pixi/events";
import { Accessor, createEffect } from "solid-js";
import { AllMessages, BaseStore, GeneralHandler, generateStore } from "..";
import { Cursor } from "../toolStore";
import {
  createViewportStateMachine,
  ToolInputMessage,
  ToolInputs,
  ViewportEvents,
} from "./shared";
import { createExclusiveStateMachine, t } from "../../utils/fsm";
import { SolixiState } from "@bearbroidery/solixi";
import { Uuid } from "../../utils/uuid";
import { EmbBase, EmbHasVirtual } from "../../types/scene";
import {
  Command,
  DeselectObjectsCommand,
  MoveObjectCommand,
} from "../commands";
import { SceneModel } from "../sceneStore";
import { InputModel } from "../inputStore";
import { Point } from "@pixi/core";
import { MultiCommand } from "../commands/shared";
import { SetInspectingCommand } from "../commands/SetInspectingCommand";
import { EmbObject } from "../../types/scene";
import { tryMakeGraphicsNodeACurve } from "../helpers";

export const PenEvents = {
  Hover: Symbol("s-Hover"),
  Unhover: Symbol("s-Unhover"),
  PointerDown: Symbol("s-Pointerdown"),
  PointerUp: Symbol("s-Pointerup"),
  DragStart: Symbol("s-Dragstart"),
  DoubleClick: Symbol("s-Doubleclick"),
  DragMove: Symbol("s-Dragmove"),
  DragEnd: Symbol("s-Dragend"),
} as const;

export const PenStates = {
  Default: Symbol("s-Default"),
  Hoverring: Symbol("s-Hoverring"),
  Moving: Symbol("s-Moving"),
  PointerDownOnElement: Symbol("s-PointerDownOnElement"),
  PointerDownOnEmpty: Symbol("s-PointerDownOnElement"),
  Pening: Symbol("s-Pening"),
} as const;

export type PenToolMessage = {
  "activate": void;
  "deactivate": void;
  "input": ToolInputMessage;
};
export type PenToolModel = {
  state: Accessor<typeof PenStates[keyof typeof PenStates]>;
};

export type PenToolStore = BaseStore<PenToolModel, PenToolMessage>;

export const createPenToolStore = (
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
  let currHover: Uuid<EmbBase> | undefined;
  const offset = new Point();
  let newPosition: Point | undefined;

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
    t(
      PenStates.Default,
      PenEvents.Hover,
      PenStates.Hoverring,
      () => dispatch("tool:push-cursor", Cursor.Point),
    ),
    t(
      PenStates.Hoverring,
      PenEvents.Unhover,
      PenStates.Default,
      () => dispatch("tool:clear-cursor", Cursor.Point),
    ),
    t(
      PenStates.Hoverring,
      PenEvents.PointerDown,
      PenStates.PointerDownOnElement,
      (id: Uuid<EmbBase>) => {
        // const cmds: Command[] = [];
        const obj = sceneModel.objects.get(id) as
          & EmbBase
          & EmbHasVirtual;
        console.log(obj);
        // if (obj && obj.virtual) {
        //   const cmd = obj.virtualCreator();
        //   cmds.push(cmd);
        // }
        // cmds.push(
        //   new DeselectObjectsCommand(
        //     ...sceneModel.selectedIds,
        //   ),
        // );
        // cmds.push(new PenObjectsCommand(id));
        // const cmd = new MultiCommand(...cmds);
        // cmd.name = `Pen ${id}`;
        // dispatch("scene:do-command", cmd);
      },
    ),
    t(
      PenStates.PointerDownOnEmpty,
      PenEvents.PointerUp,
      PenStates.Default,
      () => {
      },
    ),
    t(
      PenStates.Default,
      PenEvents.PointerDown,
      PenStates.PointerDownOnEmpty,
    ),
    t(
      PenStates.PointerDownOnEmpty,
      PenEvents.DragStart,
      PenStates.Pening,
      () => {
      },
    ),
    t(
      PenStates.Pening,
      PenEvents.DragEnd,
      PenStates.Default,
      () => {
      },
    ),
    t(
      PenStates.PointerDownOnElement,
      PenEvents.PointerUp,
      PenStates.Hoverring,
    ),
    t(
      PenStates.PointerDownOnElement,
      PenEvents.DoubleClick,
      PenStates.PointerDownOnElement,
      () => {
      },
    ),
    t(
      PenStates.PointerDownOnElement,
      PenEvents.DragStart,
      PenStates.Moving,
      () => {
      },
    ),
    t(PenStates.Moving, PenEvents.DragMove, PenStates.Moving, () => {
    }),
    t(PenStates.Moving, PenEvents.DragEnd, PenStates.Hoverring, () => {
    }),
  ];

  const {
    state,
    block: sBlock,
    unblock: sUnblock,
    can: sCan,
    dispatch: sDispatch,
  } = createExclusiveStateMachine(PenStates.Default, transitions, {
    exclusiveStates: [PenStates.Pening, PenStates.Moving],
    onExclusive: () => {
      vpBlock();
    },
    onNonExclusive: () => {
      vpUnblock();
    },
  });

  sUnblock();
  vpUnblock();

  const model: PenToolModel = {
    state: state,
  };

  const result = generateStore<PenToolModel, PenToolMessage>(model, {
    "input": (_1, _2, msg) => {
      switch (msg.type) {
        case "pointer1-move":
          {
            if (boundary) {
              const data = msg.data as ToolInputs["pointer1-move"];
              const result = boundary.hitTest(data.position.x, data.position.y);
              if (result && result.id) {
                if (currHover && result.id !== currHover) {
                  if (sCan(PenEvents.Unhover)) {
                    sDispatch(PenEvents.Unhover);
                    dispatch("scene:unhover", currHover);
                  }
                }
                if (sCan(PenEvents.Hover)) {
                  sDispatch(PenEvents.Hover);
                  dispatch("scene:hover", result.id);
                  currHover = result.id;
                }
              } else if (sCan(PenEvents.Unhover)) {
                // console.log("Unhovering");
                sDispatch(PenEvents.Unhover);
                if (currHover) dispatch("scene:unhover", currHover);
                currHover = undefined;
              }
            }
          }
          break;
        case "pointer1-down":
          {
            if (vpCan(ViewportEvents.PointerDown)) {
              vpDispatch(ViewportEvents.PointerDown);
            }
            if (sCan(PenEvents.PointerDown)) {
              sDispatch(PenEvents.PointerDown, currHover);
            }
          }
          break;
        case "pointer1-doubleclick":
          {
            if (sCan(PenEvents.DoubleClick)) {
              sDispatch(PenEvents.DoubleClick);
            }
          }
          break;
        case "pointer1-up":
          {
            if (vpCan(ViewportEvents.PointerUp)) {
              vpDispatch(ViewportEvents.PointerUp);
            }
            if (sCan(PenEvents.PointerUp)) sDispatch(PenEvents.PointerUp);
          }
          break;
        case "pointer1-dragstart":
          {
            if (sCan(PenEvents.DragStart)) sDispatch(PenEvents.DragStart);
          }
          break;
        case "pointer1-dragmove":
          {
            if (sCan(PenEvents.DragMove)) sDispatch(PenEvents.DragMove);
          }
          break;
        case "pointer1-dragend":
          {
            if (sCan(PenEvents.DragEnd)) sDispatch(PenEvents.DragEnd);
          }
          break;
        case "keydown":
          {
            const data = msg.data as ToolInputs["keydown"];
            if (data.key === " " && vpCan(ViewportEvents.SpaceDown)) {
              console.log("dispatch space down");
              vpDispatch(ViewportEvents.SpaceDown);
            }
          }
          break;
        case "keyup": {
          const data = msg.data as ToolInputs["keyup"];
          if (data.key === " " && vpCan(ViewportEvents.SpaceUp)) {
            console.log("dispatch space up");
            vpDispatch(ViewportEvents.SpaceUp);
          }
        }
      }
    },
    "activate": (_1, _2) => {
      console.log("Pen tool activated");
      vpUnblock();
      sUnblock();
    },
    "deactivate": (_1, _2) => {
      console.log("Pen tool deactivated");
      vpBlock();
      sBlock();
    },
  });

  return result;
};
