import { EventBoundary } from "@pixi/events";
import { Accessor, createEffect } from "solid-js";
import { AllMessages, BaseStore, GeneralHandler, generateStore } from "..";
import { Cursor, } from "../toolStore";
import {
  createViewportStateMachine,
  ToolInputMessage,
  ToolInputs,
  ViewportEvents,
} from "./shared";
import { createExclusiveStateMachine, t } from "../../utils/fsm";
import { SolixiState } from "@bearbroidery/solixi";
import { metadata } from "../../utils/metadata";
import { Uuid } from "../../utils/uuid";
import { SceneObject } from "../../types/scene";
import {
  DeselectObjectsCommand,
  MoveObjectCommand,
  SelectObjectsCommand,
} from "../commands";
import { SceneModel } from "../sceneStore";
import { InputModel } from "../inputStore";
import { MultiCommand } from "../commands/index";
import { Point } from "@pixi/core";

export const SelectEvents = {
  Hover: Symbol("s-Hover"),
  Unhover: Symbol("s-Unhover"),
  PointerDown: Symbol("s-Pointerdown"),
  PointerUp: Symbol("s-Pointerup"),
  DragStart: Symbol("s-Dragstart"),
  DoubleClick: Symbol("s-Doubleclick"),
  DragMove: Symbol("s-Dragmove"),
  DragEnd: Symbol("s-Dragend"),
} as const;

export const SelectStates = {
  Default: Symbol("s-Default"),
  Hoverring: Symbol("s-Hoverring"),
  Moving: Symbol("s-Moving"),
  PointerDownOnElement: Symbol("s-PointerDownOnElement"),
  PointerDownOnEmpty: Symbol("s-PointerDownOnElement"),
  Selecting: Symbol("s-Selecting"),
} as const;

export type SelectToolMessage = {
  "activate": void;
  "deactivate": void;
  "input": ToolInputMessage;
};
export type SelectToolModel = {
  state: Accessor<typeof SelectStates[keyof typeof SelectStates]>;
};

export type SelectToolStore = BaseStore<SelectToolModel, SelectToolMessage>;

export const createSelectToolStore = (
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
  let currHover: Uuid<SceneObject> | undefined;
  let offset = new Point();
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
      SelectStates.Default,
      SelectEvents.Hover,
      SelectStates.Hoverring,
      () => dispatch("tool:push-cursor", Cursor.Point),
    ),
    t(
      SelectStates.Hoverring,
      SelectEvents.Unhover,
      SelectStates.Default,
      () => dispatch("tool:clear-cursor", Cursor.Point),
    ),
    t(
      SelectStates.Hoverring,
      SelectEvents.PointerDown,
      SelectStates.PointerDownOnElement,
      (id: Uuid<SceneObject>) => {
        const deselectAllCmd = new DeselectObjectsCommand(
          ...sceneModel.selectedIds,
        );
        const selectObjCmd = new SelectObjectsCommand(id);
        const cmd = new MultiCommand(deselectAllCmd, selectObjCmd);
        cmd.name = `Select ${id}`;
        dispatch("scene:do-command", cmd);
      },
    ),
    t(
      SelectStates.PointerDownOnEmpty,
      SelectEvents.PointerUp,
      SelectStates.Default,
      () => {
        if (sceneModel.inspecting !== undefined) dispatch("scene:uninspect");
        const deselectAllCmd = new DeselectObjectsCommand(
          ...sceneModel.selectedIds,
        );
        dispatch("scene:do-command", deselectAllCmd);
      },
    ),
    t(
      SelectStates.Default,
      SelectEvents.PointerDown,
      SelectStates.PointerDownOnEmpty,
    ),
    t(
      SelectStates.PointerDownOnEmpty,
      SelectEvents.DragStart,
      SelectStates.Selecting,
      () => {
      },
    ),
    t(
      SelectStates.Selecting,
      SelectEvents.DragEnd,
      SelectStates.Default,
      () => {
      },
    ),
    t(
      SelectStates.PointerDownOnElement,
      SelectEvents.PointerUp,
      SelectStates.Hoverring,
    ),
    t(
      SelectStates.PointerDownOnElement,
      SelectEvents.DoubleClick,
      SelectStates.PointerDownOnElement,
      () => {
        if (!currHover) {
          throw new Error(
            "Attempted to inspect: Currently hovered element but no element hovered.",
          );
        }
        dispatch("scene:inspect", currHover);
      },
    ),
    t(
      SelectStates.PointerDownOnElement,
      SelectEvents.DragStart,
      SelectStates.Moving,
      () => {
        const obj = sceneModel.selectedObjects[0];
        inputModel.position;

        const diffx = obj.position.x - inputModel.position.x;
        const diffy = obj.position.y - inputModel.position.y;
        offset.set(diffx, diffy);

        newPosition = inputModel.position.clone();
        newPosition.x += offset.x;
        newPosition.y += offset.y;

        const moveCommand = new MoveObjectCommand(obj.id, newPosition);
        moveCommand.final = false;
        dispatch("scene:do-command", moveCommand);
      },
    ),
    t(SelectStates.Moving, SelectEvents.DragMove, SelectStates.Moving, () => {
      const obj = sceneModel.selectedObjects[0];
      newPosition = inputModel.position.clone();
      newPosition.x += offset.x;
      newPosition.y += offset.y;

      const moveCommand = new MoveObjectCommand(obj.id, newPosition);
      moveCommand.final = false;
      dispatch("scene:do-command", moveCommand);
    }),
    t(SelectStates.Moving, SelectEvents.DragEnd, SelectStates.Hoverring, () => {
      const obj = sceneModel.selectedObjects[0];
      newPosition = inputModel.position.clone();
      newPosition.x += offset.x;
      newPosition.y += offset.y;

      const moveCommand = new MoveObjectCommand(obj.id, newPosition);
      moveCommand.final = true;
      dispatch("scene:do-command", moveCommand);
    }),
  ];

  const {
    state,
    block: sBlock,
    unblock: sUnblock,
    can: sCan,
    dispatch: sDispatch,
  } = createExclusiveStateMachine(SelectStates.Default, transitions, {
    exclusiveStates: [SelectStates.Selecting, SelectStates.Moving],
    onExclusive: () => {
      vpBlock();
    },
    onNonExclusive: () => {
      vpUnblock();
    },
  });

  sUnblock();
  vpUnblock();

  const model: SelectToolModel = {
    state: state,
  };

  const result = generateStore<SelectToolModel, SelectToolMessage>(model, {
    "input": (_1, _2, msg) => {
      switch (msg.type) {
        case "pointer1-move":
          {
            if (boundary) {
              const data = msg.data as ToolInputs["pointer1-move"];
              const result = boundary.hitTest(data.position.x, data.position.y);
              if (result) {
                const data = metadata.get(result);
                if (data) {
                  if (currHover && data.id !== currHover) {
                    if (sCan(SelectEvents.Unhover)) {
                      sDispatch(SelectEvents.Unhover);
                      dispatch("scene:unhover", currHover);
                    }
                  }
                  if (sCan(SelectEvents.Hover)) {
                    sDispatch(SelectEvents.Hover);
                    dispatch("scene:hover", data.id);
                    currHover = data.id;
                  }
                } else if (sCan(SelectEvents.Unhover)) {
                  // console.log("Unhovering");
                  sDispatch(SelectEvents.Unhover);
                  if (currHover) dispatch("scene:unhover", currHover);
                  currHover = undefined;
                }
              }
            }
          }
          break;
        case "pointer1-down":
          {
            if (vpCan(ViewportEvents.PointerDown)) {
              vpDispatch(ViewportEvents.PointerDown);
            }
            if (sCan(SelectEvents.PointerDown)) {
              sDispatch(SelectEvents.PointerDown, currHover);
            }
          }
          break;
        case "pointer1-doubleclick":
          {
            if (sCan(SelectEvents.DoubleClick)) {
              sDispatch(SelectEvents.DoubleClick);
            }
          }
          break;
        case "pointer1-up":
          {
            if (vpCan(ViewportEvents.PointerUp)) {
              vpDispatch(ViewportEvents.PointerUp);
            }
            if (sCan(SelectEvents.PointerUp)) sDispatch(SelectEvents.PointerUp);
          }
          break;
        case "pointer1-dragstart":
          {
            if (sCan(SelectEvents.DragStart)) sDispatch(SelectEvents.DragStart);
          }
          break;
        case "pointer1-dragmove":
          {
            if (sCan(SelectEvents.DragMove)) sDispatch(SelectEvents.DragMove);
          }
          break;
        case "pointer1-dragend":
          {
            if (sCan(SelectEvents.DragEnd)) sDispatch(SelectEvents.DragEnd);
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
      console.log("Select tool activated");
      vpUnblock();
      sUnblock();
    },
    "deactivate": (_1, _2) => {
      console.log("Select tool deactivated");
      vpBlock();
      sBlock();
    },
  });

  return result;
};
