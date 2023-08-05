import { Point } from '@pixi/core';
import { t } from "typescript-fsm";
import { tFromMulti } from "../../primitives/createStateMachine";
import { Cursor, ToolHandler } from "../toolStore";

export type ToolInputs = {
  'pointer1-down': {
    position: Point,
  },
  'pointer1-move': {
    downPosition: Point,
    position: Point,
  },
  'pointer1-up': {
    downPosition: Point,
    position: Point,
  },
  'pointer1-click': {
    position: Point,
  }
  'pointer1-dragstart': {
    downPosition: Point,
    position: Point,
  },
  'pointer1-dragmove': {
    downPosition: Point,
    position: Point,
  },
  'pointer1-dragend': {
    downPosition: Point,
    position: Point,
  },
  'keypress': {
    key: string,
  },
  'keydown': {
    key: string,
    keys: Set<string>,
  },
  'keyup': {
    key: string,
    keys: Set<string>,
  }
}

export type ToolInputMessage<K extends keyof ToolInputs = keyof ToolInputs, M extends ToolInputs[K] = ToolInputs[K]> = {
    type: K,
    data: M,
  };

export enum BaseStates {
  Blocked = 'Blocked',
  Default = 'Default',
  CanPan = 'CanPan',
  Panning = 'Panning',
  PanningWithoutSpace = 'PanningWithoutSpace'
}
export enum BaseEvents {
  Block = 'Block',
  Unblock = 'Unblock',
  SpaceDown = 'SpaceDown',
  SpaceUp = 'SpaceUp',
  PointerDown = 'PointerDown',
  PointerUp = 'PointerUp',
}

export const generateViewportStateMachine = (dispatch: ToolHandler) => {
  const transitions = [
      ...tFromMulti([BaseStates.Default, BaseStates.CanPan, BaseStates.Panning, BaseStates.PanningWithoutSpace], BaseEvents.Block, BaseStates.Blocked, () => dispatch('tool:clear-cursor')),
      t(BaseStates.Blocked, BaseEvents.Unblock, BaseStates.Default),
      t(BaseStates.Default, BaseEvents.SpaceDown, BaseStates.CanPan, () => dispatch('tool:push-cursor', Cursor.Grab)),
      t(BaseStates.CanPan, BaseEvents.PointerDown, BaseStates.Panning, () => dispatch('tool:push-cursor', Cursor.Grabbing)),
      t(BaseStates.CanPan, BaseEvents.SpaceUp, BaseStates.Default, () => dispatch('tool:pop-cursor', Cursor.Grab)),
      t(BaseStates.Panning, BaseEvents.SpaceUp, BaseStates.PanningWithoutSpace),
      t(BaseStates.Panning, BaseEvents.PointerUp, BaseStates.CanPan, () => dispatch('tool:pop-cursor', Cursor.Grabbing)),
      t(BaseStates.PanningWithoutSpace, BaseEvents.PointerUp, BaseStates.Default, () => dispatch('tool:clear-cursor')),
    ];
  return { events: BaseEvents, states: BaseStates, transitions };
}

