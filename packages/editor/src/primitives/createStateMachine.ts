import { Accessor, createEffect, createSignal, untrack } from 'solid-js';
import { MaybeAccessor, access } from '@solid-primitives/utils';
import { StateMachine, ITransition, Callback, t } from 'typescript-fsm';

type StateMachineResult<TState, TEvent> = {
  state: Accessor<TState>;
  can: (event: TEvent) => boolean;
  dispatch: (event: TEvent) => Promise<void>;
}

export function tFromMulti<TState, TEvent>(from: TState[], event: TEvent, to: TState, cb?: Callback) {
  return from.map(from => 
    t(from, event, to, cb));
}

export function createStateMachine<TState, TEvent>(initialState: TState, transitions: MaybeAccessor<ITransition<TState, TEvent>[]>): StateMachineResult<TState, TEvent> {
  const [state, setState] = createSignal(initialState);
  let machine: StateMachine<TState, TEvent>;

  createEffect(() => {
    machine = new StateMachine<TState, TEvent>(untrack(state), access(transitions));
  })

  return {
    state,
    can: (event) => {
      return machine.can(event)
    },
    dispatch: async (event) => {
      await machine.dispatch(event);
      setState(() => machine.getState())
    }
  }
}
