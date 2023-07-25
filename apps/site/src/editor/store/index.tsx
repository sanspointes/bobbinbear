import { createContext, JSXElement } from "solid-js";
import { createStore } from "solid-js/store";
import { Vector2 } from "three";

/// EmbAppState
export enum EmbTool {
  Default,
  Grab,
}
export type EmbAppState = {
  tool: EmbTool;
};

/// EmbCameraState
export type EmbCameraState = {
  position: Vector2;
  zoom: number;
};

export type EmbState = {
  tool: EmbTool;
  camera: EmbCameraState;
};

export const EditorContext = createContext<EmbState>(
  null as unknown as EmbState,
);

type EditorProviderProps = {
  children: JSXElement;
};
export const EditorProvider = ({ children }: EditorProviderProps) => {
  const [state, setState] = createStore<EmbState>({
    camera: {
      position: new Vector2(),
      zoom: 1,
    },
    tool: {
      current: EmbTool.Default,
    },
  });

  const setTool = (tool: EmbTool) => {
    setState("tool", "current", tool);
  };

  return (
    <EditorContext.Provider value={state}>
      {children}
    </EditorContext.Provider>
  );
};
