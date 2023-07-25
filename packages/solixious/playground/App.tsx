/** @jsxImportSource solid-js */
import { Canvas, RootState, T } from "../src";
import { Texture } from "@pixi/core";

export function App() {
  const onCreated = (state: RootState) => {
    console.log({ state });

    console.log(state.gl.stage === state.scene);
    globalThis.__PIXI_APP__ = state.gl;
  }
  return (
    <div style="width: 100vw; height: 100vh">
      <Canvas
        onCreated={onCreated}
        gl={{
          antialias: true,
          background: "#333",
        }}
      >
        <T.Container>
          <T.Mesh>
            <T.PlaneGeometry />
            <T.MeshMaterial args={[Texture.WHITE]} />
          </T.Mesh>
        </T.Container>
      </Canvas>
    </div>
  );
}
