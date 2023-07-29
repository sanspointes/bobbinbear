import { createContext, useContext } from "solid-js";
import { createRoot } from "./renderer";
import { ExtraPropHandler, wrapConstructable, WrapConstructableOptions } from "./elements";
import { Constructable } from "./types";
import { createStore } from "solid-js/store";

export type { SxiObject, SxiInstance } from './types';

/**
 * Creates a new type of renderer that returns functions for generating 
 * the root of the renderer, sharing the context around, and wrapping new Constructables.
 */
export const createRenderer = <TContext extends object>(initialState: TContext) => {
  const Context = createContext<TContext>(initialState);

  const useConstructableState = () => {
    const state = useContext(Context);
    if (!state) {
      throw new Error(
        "Constructable: Must use constructable state within constructable root.",
      );
    }
    return state;
  };

  return {
    useConstructableState,
    createRoot: <TRootObject>(rootObject: TRootObject) =>
      createRoot<TContext, TRootObject>(rootObject, Context, initialState),
    wrapConstructable: <
      TSource extends Constructable,
      TExtraProps extends Record<string, ExtraPropHandler<TContext, TSource>>,
    >(
      source: TSource,
      options: WrapConstructableOptions<TContext, TSource, TExtraProps>,
    ) => wrapConstructable<TContext, TSource, TExtraProps>(source, options, useConstructableState),
  };
};
