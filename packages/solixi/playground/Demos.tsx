/* @jsxImportSource solid-js */
import { createSignal } from "solid-js";
import { PContainer, PMesh, PMeshMaterial, PPlaneGeometry, useFrame } from "../src"

export const Interactivity = () => {
  const [color, setColor] = createSignal('#ff0000');
  return (
    <PMesh position-x={100} position-y={() => 100} onPointerOver={() => {
      setColor('#ff0000');
    }} onPointerOut={() => {
      setColor('#00ff00');
    }}>
      <PPlaneGeometry args={[100, 100]} />
      <PMeshMaterial tint={color} />
    </PMesh>
  );
}

export const Parenting = () => {
  const [rotation, setRotation] = createSignal(0);
  const [position, setPosition] = createSignal<[number, number]>([100, 100]);
  useFrame((_, time, delta) => {
    setPosition([100 + Math.sin(time / 3000) * 20, 100 + Math.cos(time / 300) * 20]);
    setRotation(rotation() + delta / 100);
  })
  return (
    <PContainer position-x={300} position-y={200}>
      <PMesh position={position()} rotation={rotation()}>
        <PPlaneGeometry args={[50, 50]} width={100} height={100} />
        <PMeshMaterial tint={'#ff0000'} />
        <PMesh position={position()} rotation={rotation()}>
          <PPlaneGeometry args={[50, 50]} width={100} height={100} />
          <PMeshMaterial tint={'#ff0000'} />
          <PMesh position={position()} rotation={rotation()}>
            <PPlaneGeometry args={[50, 50]} width={100} height={100} />
            <PMeshMaterial tint={'#ff0000'} />
          </PMesh>
        </PMesh>
      </PMesh>
    </PContainer>
  )
}
