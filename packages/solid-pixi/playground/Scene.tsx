import { MeshMaterial, PlaneGeometry, Texture } from "pixi.js";
import { createSignal, For, useFrame, useThree } from "../src";

const geometry = new PlaneGeometry(20, 20);
const material = new MeshMaterial(Texture.WHITE);

export function Scene() {
  const [rotation, setRotation] = createSignal(0);

  const ticker = useThree((i) => i.ticker);

  useFrame(() => {
    setRotation(rotation() + ticker().deltaTime);
    console.log('Setting rotation to ', rotation());
  });

  const boxes = new Array(500).fill(0).map((_, i) => i);
  return (
    <>
      <For each={boxes}>
        {(i) => <mesh position={[i * 50, 0]} rotation={rotation()} geometry={geometry} material={material}/>}
      </For>
    </>
  );
}
