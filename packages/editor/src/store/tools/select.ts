import { EventBoundary } from "@pixi/events";
import { Accessor, createEffect } from "solid-js";
import { AllMessages, BaseStore, GeneralHandler, generateStore } from "..";
import { Cursor, ToolModel, ToolStoreMessage } from "../toolStore";
import {
  createViewportStateMachine,
  ToolInputMessage,
  ToolInputs,
  ViewportEvents,
} from "./shared";
import { createExclusiveStateMachine, t } from "../../utils/fsm.ts";
import { SolixiState } from "@bearbroidery/solixi";
import { metadata } from "../../utils/metadata.ts";
import { Uuid } from "../../utils/uuid.ts";
import { SceneObject } from "../../types/scene.ts";
import { DeselectObjectsCommand, SelectObjectsCommand } from "../commands/object.ts";
import { SceneModel } from "../sceneStore.tsx";
import { InputModel } from "../inputStore.ts";
import { MultiCommand } from "../commands/index.ts";

export const SelectEvents = {
  Hover: Symbol("s-Hover"),
  Unhover: Symbol("s-Unhover"),
  PointerDown: Symbol("s-Pointerdown"),
  PointerUp: Symbol("s-Pointerup"),
  DragStart: Symbol("Dragstart"),
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
  isSelecting: boolean;
  state: Accessor<typeof SelectStates[keyof typeof SelectStates]>,
};

export type SelectToolStore = BaseStore<SelectToolModel, SelectToolMessage>;

export const createSelectToolStore = (
  dispatch: GeneralHandler<AllMessages>,
  solixi: Accessor<SolixiState | undefined>,
  _inputModel: InputModel,
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
        const deselectAllCmd = new DeselectObjectsCommand(...sceneModel.selectedIds);
        const selectObjCmd = new SelectObjectsCommand(id);
        const cmd = new MultiCommand(deselectAllCmd, selectObjCmd);
        dispatch("scene:do-command", cmd);
      },
    ),
    t(SelectStates.Default, SelectEvents.PointerDown, SelectStates.PointerDownOnEmpty),
    t(SelectStates.PointerDownOnEmpty, SelectEvents.DragStart, SelectStates.Selecting, () => {
      result.store.isSelecting = true;
    }),
    t(SelectStates.Selecting, SelectEvents.DragEnd, SelectStates.Default, () => {
      result.store.isSelecting = false;
    }),
    t(SelectStates.PointerDownOnElement, SelectEvents.PointerUp, SelectStates.Hoverring),
    t(SelectStates.PointerDownOnElement, SelectEvents.DragStart, SelectStates.Moving),
    t(SelectStates.PointerDownOnEmpty, SelectEvents.PointerUp, SelectStates.Default, () => {
      const deselectAllCmd = new DeselectObjectsCommand(...sceneModel.selectedIds);
      dispatch('scene:do-command', deselectAllCmd);
    }),
  ];

  const { state, block: sBlock, unblock: sUnblock, can: sCan, dispatch: sDispatch } =
    createExclusiveStateMachine(SelectStates.Default, transitions, {
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
    isSelecting: false,
    state: state,
  }

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
                if (data && sCan(SelectEvents.Hover)) {
                  // console.log("Hovering");
                  sDispatch(SelectEvents.Hover);
                  dispatch("scene:hover", data.id);
                  currHover = data.id;
                } else if (!data && sCan(SelectEvents.Unhover)) {
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
