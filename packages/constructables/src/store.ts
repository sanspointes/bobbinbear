import { createContext, useContext } from "solid-js"
import { createStore } from "solid-js/store";
import { SxiState } from "./types";

/** Context to store the SxiState */
export const SxiContext = createContext<SxiState>();

/**
 * @returns The SxiState of the current Solixi app (if within a SolixiCanvas)
 */
export const usePixi = () => {
  const store = useContext(SxiContext);
  if (!store) {
    throw new Error("Solixi: Hooks can only be used within the Canvas component!");
  }
  return store;
}

export const createSolixiState = (initialState: SxiState) => {
}
