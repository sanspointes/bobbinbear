import { P } from "@bearbroidery/solixi";
import { createEffect, createMemo, createSignal, untrack, useContext } from "solid-js";
import { AppContext } from "../store";
import { Graphics } from "@pixi/graphics";
import { Point, Rectangle } from "@pixi/core";
import { SelectStates } from "../store/tools/select";
import { Tool } from "../store/toolStore";

export function SelectBox () {
  const { inputStore, toolStore } = useContext(AppContext);

  console.warn('Creating new select box.');

  const visible = createMemo(() => toolStore.tool === Tool.Select && toolStore.selectTool.state() === SelectStates.Selecting);

  const [bounds, setBounds] = createSignal(new Rectangle(), {
    equals: false,
  })
  createEffect(() => {
    if (!visible()) return;
    const b = untrack(bounds);
    if (!inputStore.downPosition) {
      b.width = 0;
      b.height = 0;
      setBounds(b);
      return;
    };
    const diffx = inputStore.downPosition.x - inputStore.position.x;
    const diffy = inputStore.downPosition.y - inputStore.position.y;
    const min = new Point(Math.min(inputStore.downPosition.x, inputStore.position.x), Math.min(inputStore.downPosition.y, inputStore.position.y));

    b.x = min.x;
    b.y = min.y;
    b.width = Math.abs(diffx);
    b.height = Math.abs(diffy);

    setBounds(b)
  })
  
  let graphicsEl: Graphics|undefined;
  createEffect(() => {
    if (!graphicsEl) throw new Error('Could not get graphics geometry.')
    console.log(graphicsEl);
    graphicsEl.clear()

    graphicsEl.beginFill(0x0A8CE9, 0.3);
    graphicsEl.lineStyle(1, 0x0A8CE9, 0.7)
    graphicsEl.drawRect(0, 0, bounds().width, bounds().height);
  })

  return (
    <P.Graphics name="SelectBox" visible={visible()} zIndex={1000} ref={graphicsEl} position={[bounds().x, bounds().y]}>
    </P.Graphics>
  )
}
