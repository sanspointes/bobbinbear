import { MeshMaterial, PlaneGeometry, Point, Texture } from "pixi.js";
import { Canvas } from "../src";
import { Scene } from "./Scene";

export function App() {
  return (
    <Canvas
      gl={{
        antialias: true,
      }}
    >
      <Scene />
    </Canvas>
  );
}
