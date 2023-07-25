import { Canvas } from "solid-three";
import Scene from "./Scene";

export default function Editor() {
  return (
    <Canvas
      camera={{
        position: [3, 3, 3],
      }}
      gl={{
        antialias: true,
      }}
      shadows
    >
      <Scene />
      <ambientLight />
      <spotLight position={[0, 5, 10]} intensity={1} />
    </Canvas>
  );
}
