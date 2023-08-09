import { createEventListener } from '@solid-primitives/event-listener';
import { t } from './utils/fsm';
import { createStateMachine, tFromMulti } from "./primitives/createStateMachine";
import { createEffect, useContext } from 'solid-js';
import { AppContext } from './store';
import { Cursor } from './store/toolStore';

enum States {
  Blocked = 'Blocked',
  Default = 'Default',
  CanPan = 'CanPan',
  Panning = 'Panning',
  PanningWithoutSpace = 'PanningWithoutSpace'
}
enum Events {
  Block = 'Block',
  Unblock = 'Unblock',
  SpaceDown = 'SpaceDown',
  SpaceUp = 'SpaceUp',
  PointerDown = 'PointerDown',
  PointerUp = 'PointerUp',
}

export function createInputControls(element: HTMLElement) {
  const { dispatch } = useContext(AppContext);

  const transitions = [
    ...tFromMulti([States.Default, States.CanPan, States.Panning, States.PanningWithoutSpace], Events.Block, States.Blocked, () => dispatch('tool:clear-cursor')),
    t(States.Blocked, Events.Unblock, States.Default),
    t(States.Default, Events.SpaceDown, States.CanPan, () => dispatch('tool:push-cursor', Cursor.Grab)),
    t(States.CanPan, Events.PointerDown, States.Panning, () => dispatch('tool:push-cursor', Cursor.Grabbing)),
    t(States.CanPan, Events.SpaceUp, States.Default, () => dispatch('tool:pop-cursor', Cursor.Grab)),
    t(States.Panning, Events.SpaceUp, States.PanningWithoutSpace),
    t(States.Panning, Events.PointerUp, States.CanPan, () => dispatch('tool:pop-cursor', Cursor.Grabbing)),
    t(States.PanningWithoutSpace, Events.PointerUp, States.Default, () => dispatch('tool:clear-cursor')),
  ];

  console.log('Input controls', element);
  const { state, dispatch: panDispatch, can: panCan } = createStateMachine(States.Default, transitions);

  createEffect(() => console.log(state()));

  createEventListener(element, ['pointerdown', 'pointerup'], (event) => {
    console.log('Pointer ev ', event);
    if (event.type === 'pointerdown' && panCan(Events.PointerDown)) panDispatch(Events.PointerDown);
    if (event.type === 'pointerup' && panCan(Events.PointerUp)) panDispatch(Events.PointerUp);
  })
  createEventListener(element, 'keydown', (event) => {
    console.log('key ev ', event);
    if (panCan(Events.SpaceDown))  panDispatch(Events.SpaceDown);
  })
  createEventListener(element, 'keyup', (event) => {
    console.log('key ev ', event);
    if (panCan(Events.SpaceUp))  panDispatch(Events.SpaceUp);
  })
}
