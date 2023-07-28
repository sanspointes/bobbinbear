import { createSolixiState, SxiContext } from "./store";
import { SxiState } from "./types";

type Canvas = HTMLCanvasElement | OffscreenCanvas;

type SxiProviderProps<TCanvas extends Canvas> = {
  onCreated?: (state: SxiState) => void;
  state: SxiState;
  children: JSX.Element;
  rootElement: TCanvas;
}
const SxiProvider = <TCanvas extends Canvas>(props: SxiProviderProps<TCanvas>) => {
  return (
    <SxiContext.Provider value={props.state}>
      {props.children}
    </SxiContext.Provider/>
  );
}


export type SolixiRootProps = {

}

export type SolixiRoot<TCanvas extends Canvas> = {
  state: SxiState;
  update: (props: SolixiRootProps) => SolixiRoot<TCanvas>;
}

const canvasToRootsLookup = new Map<Canvas, SolixiRoot<Canvas>>();

export const createRoot = <TCanvas extends Canvas>(canvas: TCanvas): SolixiRoot<TCanvas> => {
  // Check if createRoot has already been aclled on this element
  const prevRoot = canvasToRootsLookup.get(canvas);
  if (prevRoot) console.warn("Solixi.createRoot should only be called once!");

  const state = prevRoot?.state || createSolixiState();

  const root: SolixiRoot<TCanvas> = {
    state,
    update(props) {

    }
  }

  return root;
}
