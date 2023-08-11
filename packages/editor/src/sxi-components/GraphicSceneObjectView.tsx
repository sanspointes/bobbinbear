import { P } from "@bearbroidery/solixi";
import { SceneObjectChildren } from "./general";
import {
  GraphicNodeTypes,
  GraphicSceneObject,
  GraphicsNode,
} from "../types/scene";
import { Graphics, IFillStyleOptions, ILineStyleOptions } from "@pixi/graphics";
import { createEffect, createRenderEffect, onMount, untrack } from "solid-js";
import { OutlineFilter } from "@pixi/filter-outline";
import { metadata } from "../utils/metadata";
import { arrayRemoveEl } from "../utils/array";

const updateGraphics = (g: Graphics, shape: GraphicsNode[], fill: IFillStyleOptions, stroke: ILineStyleOptions) => {
  g.clear();
  
  // Stack stores up to 2 previous control points.
  let stackIndex = 0;
  const stack: [GraphicsNode | undefined, GraphicsNode | undefined] = [
    undefined,
    undefined,
  ];

  g.beginFill(fill.color, fill.alpha);
  g.lineStyle(stroke)

  for (const node of shape) {
    // Jump nodes jump to a new position
    if (node.type === GraphicNodeTypes.Jump) {
      g.moveTo(node.x, node.y);
      // Control points are stored to be used for curves
    } else if (node.type === GraphicNodeTypes.Control) {
      if (stackIndex >= stack.length) {
        throw new Error(
          "updateGraphics: Received more than 2 control nodes in a row.",
        );
      }
      stack[stackIndex] = node;
      stackIndex += 1;
      // When a Point is found either straight line or curve to it.
    } else if (node.type === GraphicNodeTypes.Point) {
      if (stackIndex === 0) {
        g.lineTo(node.x, node.y);
      } else if (stackIndex === 1) {
        const c0 = stack[0]!;
        g.quadraticCurveTo(c0.x, c0.y, node.x, node.y);
      } else if (stackIndex === 3) {
        const c0 = stack[0]!;
        const c1 = stack[1]!;
        g.bezierCurveTo(c0.x, c0.y, c1.x, c1.y, node.x, node.y);
      }

      if (node.close) g.closePath();
      stackIndex = 0;
    }
  }
  g.endFill();
};

export const GraphicSceneObjectView = (props: GraphicSceneObject) => {
  let graphics: Graphics | undefined;
  onMount(() => {
    if (!graphics) return;
    graphics.filters = [];
    metadata.set(graphics, {
      type: props.type,
      id: props.id,
    })
  })

  createEffect(() => {
    if (graphics) updateGraphics(graphics, props.shape, props.fill, props.stroke);
  });

  let outlinePushed = false;
  const outlineFilter = new OutlineFilter(1, 0x0A8CE9, 0.1, 1);
  createRenderEffect(() => {
    const needsPush = !outlinePushed && (props.hovered || props.selected);
    const needsRemove = outlinePushed && (!props.hovered && !props.selected);

    if (props.selected) {
      outlineFilter.color = 0x41A3E9;
      outlineFilter.thickness = 2;
    }
    else if (props.hovered) {
      outlineFilter.color = 0x0A8CE9;
      outlineFilter.thickness = 1;
    }

    if (!graphics) return;
    if (needsPush) {
      graphics.filters!.push(outlineFilter);
      outlinePushed = true;
    } else if (needsRemove) {
      arrayRemoveEl(graphics.filters!, outlineFilter);
      outlinePushed = false;
    } 
  })

  // onMount(() => {
  //   if (!graphics) throw new Error('Aafbasbdkjan')
  //   graphics.beginFill(0xff0000);
  //   graphics.lineTo(100, 0)
  //   graphics.lineTo(100, 100)
  //   graphics.lineTo(0, 100)
  //   graphics.closePath();
  // })

  return (
    <P.Graphics
      name={`${props.id} ${props.name}`}
      visible={props.visible}
      ref={graphics}
      position={props.position}
      interactive={true}
    >
      <SceneObjectChildren children={props.children} />
    </P.Graphics>
  );
};
