import { Mesh, MeshMaterial, PlaneGeometry, Point, Texture } from "pixi.js";
import { createSignal, useFrame } from "../src";

const geometry = new PlaneGeometry();
const material = new MeshMaterial(Texture.WHITE);

type BoxProps = {
  position?: Point
}
export function Box(props: BoxProps) {
  let mesh: Mesh | undefined;

  useFrame(() => setRotationY(mesh!.rotation.y += 0.01));

  const [rotationY, setRotationY] = createSignal(0);

  return (
    <mesh
      ref={mesh}
      rotation={rotationY()}
      position={props.position}
    >
      <primitive object={geometry} attach="geometry" />
      <primitive object={material} attach="material" />
    </mesh>
  );
}
