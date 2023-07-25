/** @jsxImportSource solid-js */
import { Canvas, JSX, onMount, useThree } from "../src/index";
import { Container } from "pixi.js";

import { $, browser, expect } from "@wdio/globals";
import { cleanup, render, screen } from "@solidjs/testing-library";
import { Canvas } from "../src/index";

type TestReadyHandler = (stage: Container) => void;
type TestWrapperProps = {
  onTestReady: TestReadyHandler;
  children: JSX.Element;
};
const TestWrapper = (props: TestWrapperProps) => {
  const stage = useThree((i) => i.app.stage);
  // eslint-disable-next-line solid/reactivity
  onMount(() => {
    props.onTestReady(stage());
  });
  return (
    <Canvas>
      {props.children}
    </Canvas>
  );
};

describe("Mesh", () => {
  it("should be added to the scene graph.", () => {
    const doTest = (stage: Container) => {
      expect(stage.children.length).toBe(0);
    };

    render(() => (
      <TestWrapper onTestReady={doTest}>
        <mesh>
          <planeGeometry />
          <meshMaterial color={"#ffffff"} />
        </mesh>
      </TestWrapper>
    ));
  });
});
