import { Accessor, createRenderEffect, createSignal, untrack } from "solid-js";
import { access, MaybeAccessor } from "@solid-primitives/utils";
import { setAppError } from "../Editor";
//
// https://github.com/eram/typescript-fsm/tree/master
// Adapted from typescript-fsm

/**
 * FSM Class definitions/implementations
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type Callback =
  | ((...args: any[]) => Promise<void>)
  | ((...args: any[]) => void)
  | undefined;

export interface ITransition<STATE, EVENT> {
  fromState: STATE;
  event: EVENT;
  toState: STATE;
  cb: Callback;
}

export type Transitions<STATE, EVENT> = Map<EVENT, {
  toState: STATE;
  cb: Callback;
}>;
export type AllTranstions<STATE, EVENT> = Map<STATE, Transitions<STATE, EVENT>>;

export function t<STATE, EVENT>(
  fromState: STATE,
  event: EVENT,
  toState: STATE,
  cb?: Callback,
): ITransition<STATE, EVENT> {
  return { fromState, event, toState, cb };
}

export class StateMachine<STATE, EVENT> {
  protected _current: STATE;
  protected transitions: AllTranstions<STATE, EVENT> = new Map();

  static errorHandler?: (error: Error) => void;

  // initialize the state-machine
  constructor(
    _init: STATE,
    transitions: ITransition<STATE, EVENT>[] = [],
  ) {
    this._current = _init;

    this.addTransitions(transitions);
  }

  addTransitions(transitions: ITransition<STATE, EVENT>[]): void {
    for (const trans of transitions) {
      if (!this.transitions.has(trans.fromState)) {
        this.transitions.set(trans.fromState, new Map());
      }

      this.transitions.get(trans.fromState)!.set(trans.event, {
        toState: trans.toState,
        cb: trans.cb,
      });
    }
  }

  getState(): STATE {
    return this._current;
  }

  can(event: EVENT): boolean {
    const availiableTransitions = this.transitions.get(this._current);
    if (!availiableTransitions) return false;
    return availiableTransitions.has(event);
  }

  peak(event: EVENT): STATE | undefined {
    const availiableTransitions = this.transitions.get(this._current);
    if (!availiableTransitions) return undefined;
    return availiableTransitions.get(event)?.toState;
  }

  isFinal(): boolean {
    const availiableTransitions = this.transitions.get(this._current);
    if (!availiableTransitions) return true;
    return availiableTransitions.size === 0;
  }

  dispatchUnwrapped(event: EVENT, ...args: unknown[]): Promise<void> {
    return new Promise<void>((resolve, reject) => {
      const availiableTransitions = this.transitions.get(this._current);
      if (!availiableTransitions) {
        console.error(
          `No transition: from ${
            // @ts-expect-error ; To string may not be implemeneted on unknown type
            this._current.toString
              // @ts-expect-error ; To string may not be implemeneted on unknown type
              ? this._current.toString()
              : this._current} event ${event}`,
        );
        reject();
      }

      const toTransition = availiableTransitions!.get(event);
      if (!toTransition) {
        console.error(
          `No transition: from ${
            // @ts-expect-error ; To string may not be implemeneted on unknown type
            this._current.toString
              // @ts-expect-error ; To string may not be implemeneted on unknown type
              ? this._current.toString()
              : this._current} event ${event}`,
        );
        reject();
      }

      this._current = toTransition!.toState;
      if (toTransition!.cb) {
        const p = toTransition!.cb(...args);
        if (p instanceof Promise) {
          p.then(resolve).catch((e: Error) => reject(e));
        } else {
          resolve();
        }
      }
    });
  }
  // post event async
  dispatch(event: EVENT, ...args: unknown[]) {
    const promise = this.dispatchUnwrapped(event, ...args);
    promise.catch(reason => {
      console.log('FSM Error during dispatch.', reason)
      setAppError(reason);
    })
    return promise;
  }

  force(state: STATE) {
    this._current = state;
  }
}

/**
 * HELPERS
 */

export function tFromMulti<TState, TEvent>(
  from: TState[],
  event: TEvent,
  to: TState,
  cb?: Callback,
) {
  return from.map((from) => t(from, event, to, cb));
}

/**
 * SOLIDJS PRIMITIVES
 */

type StateMachineResult<TState, TEvent> = {
  state: Accessor<TState>;
  can: (event: TEvent) => boolean;
  peak: (event: TEvent) => TState | undefined;
  dispatch: (event: TEvent, ...extraArgs: unknown[]) => Promise<void>;
  force: (state: TState) => void;
};
export function createStateMachine<TState, TEvent>(
  initialState: TState,
  transitions: MaybeAccessor<ITransition<TState, TEvent>[]>,
): StateMachineResult<TState, TEvent> {
  const [state, setState] = createSignal(initialState);
  let machine: StateMachine<TState, TEvent>;

  createRenderEffect(() => {
    machine = new StateMachine<TState, TEvent>(
      untrack(state),
      access(transitions),
    );
  });

  return {
    state,
    force: (state) => {
      setState(() => state);
      machine.force(state);
    },
    can: (event) => {
      const v = machine.can(event);
      // console.warn(`fsm @${machine.getState().toString()} can ${event.toString()}? ${v}`);
      return v;
    },
    peak: (event) => {
      return machine.peak(event);
    },
    dispatch: async (event, ...extraArgs) => {
      await machine.dispatch(event, ...extraArgs);
      setState(() => machine.getState());
    },
  };
}

export type ExclusiveStateMachineResult<TState, TEvent> =
  & StateMachineResult<TState, TEvent>
  & {
    needsExclusive: Accessor<boolean>;
    block: () => void;
    unblock: () => void;
  };

export type CreateExclusiveStateMachineOptions<TState, TEvent> = {
  exclusiveStates: TState[];
  onBlock?: () => void;
  onUnblock?: () => void;
  onExclusive?: () => void;
  onNonExclusive?: () => void;
};

const BLOCKED_STATE = Symbol("Blocked");

export function createExclusiveStateMachine<TState, TEvent>(
  initialState: TState,
  transitions: MaybeAccessor<ITransition<TState, TEvent>[]>,
  opts: MaybeAccessor<CreateExclusiveStateMachineOptions<TState, TEvent>>,
): ExclusiveStateMachineResult<TState, TEvent> {
  const { state, can, dispatch, peak, force } = createStateMachine(
    initialState,
    transitions,
  );

  const [needsExclusive, setNeedsExclusive] = createSignal(
    access(opts).exclusiveStates.includes(initialState),
  );

  return {
    block: () => {
      force(BLOCKED_STATE as unknown as TState);
      const o = access(opts);
      if (o.onBlock) o.onBlock();
    },
    unblock: () => {
      force(initialState);
      const o = access(opts);
      if (o.onUnblock) o.onUnblock();
    },
    needsExclusive,
    state,
    can,
    peak,
    force,
    dispatch: (event, ...extraArgs) => {
      const next = peak(event);
      const o = access(opts);
      if (!needsExclusive() && next && o.exclusiveStates.includes(next)) {
        setNeedsExclusive(true);
        if (o.onExclusive) o.onExclusive();
      } else if (
        needsExclusive() && next && !o.exclusiveStates.includes(next)
      ) {
        setNeedsExclusive(false);
        if (o.onNonExclusive) o.onNonExclusive();
      }
      return dispatch(event, ...extraArgs);
    },
  };
}
