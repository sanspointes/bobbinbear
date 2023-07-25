import { useFrame } from "solid-three";
import { Mesh } from "three";

export default function Scene() {
  let mesh: Mesh | undefined;
  const [hovered, setHovered] = createSignal(false);

  useFrame(() => (mesh!.rotation.y += 0.01));

  return (
    <mesh>
      <boxGeometry args={[1, 1, 1]} />
      <meshStandardMaterial color={"#ff0000"} />
    </mesh>
  );
}
