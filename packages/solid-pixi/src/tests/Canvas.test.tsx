/** @jsxImportSource solid-js */
import { browser, $, expect } from '@wdio/globals'
import { cleanup, render, screen } from '@solidjs/testing-library'
import { Canvas } from "../src/index";

// type TestReadyHandler = (stage: Container) => void;
// type TestWrapperProps = {
//   onTestReady: TestReadyHandler;
//   children: JSX.Element;
// };
// const TestWrapper = (props: TestWrapperProps) => {
//   const stage = useThree((i) => i.app.stage);
//   // eslint-disable-next-line solid/reactivity
//   onMount(() => {
//     props.onTestReady(stage());
//   });
//   return (
//     <Canvas>
//       {props.children}
//     </Canvas>
//   );
// };

describe("Canvas", () => {
  it("Should mount successfully", () => {
    const { getByRole } = render(() => (
      <Canvas />
    ));
    getByRole('canvas');
    expect(document.body.querySelector("canvas")).not.toBeNull();
  });
});
