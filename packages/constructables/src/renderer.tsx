import { Context, createMemo, JSX } from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";
import { withContext } from "./utils";
import { Constructable } from "./types";


// type SxiProviderProps<TCanvas extends Canvas> = {
//   onCreated?: (state: SxiState) => void;
//   state: SxiState;
//   children: JSX.Element;
//   rootElement: TCanvas;
// }
// const SxiProvider = <TCanvas extends Canvas>(props: SxiProviderProps<TCanvas>) => {
//   return (
//     <SxiContext.Provider value={props.state}>
//       {props.children}
//     </SxiContext.Provider/>
//   );
// }

export type SolixiRoot<TContext, TRootObject extends InstanceType<Constructable>> = {
  rootObject: TRootObject,
  state: TContext;
  setState: SetStoreFunction<TContext>;
  render: (props: { children: JSX.Element | JSX.Element[] }) => void;
}

/**
 * Creates a root object for the Constructables to add to.
 * @template TRootObject extends InstanceType<Constructable> - 
 * @template TContext - 
 * @param rootObject - 
 * @param context - 
 * @param contextValue - 
 * @param initialState - 
 * @returns 
 */
export const createRoot = <
  TContext extends object,
  TRootObject extends InstanceType<Constructable>,
>(
  rootObject: TRootObject,
  context: Context<TContext>,
  contextValue: TContext
): SolixiRoot<TContext, TRootObject> => {
  const [sxiState, setSxiState] = createStore(contextValue);

  const root: SolixiRoot<TContext, TRootObject> = {
    rootObject,
    state: sxiState,
    setState: setSxiState,
    render(props) {
      const childrenWithContext = createMemo(
        withContext(() => props.children, context, contextValue),
      );
    }
  }

  return root;
}
