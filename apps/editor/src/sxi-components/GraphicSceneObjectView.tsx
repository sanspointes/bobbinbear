import { P } from "@bearbroidery/solixi";
import { SceneObjectChildren } from "./general";
import {
  GraphicNodeTypes,
  GraphicSceneObject,
  GraphicsNode,
  GroupSceneObject,
  NodeSceneObject,
  SceneObject,
} from "../types/scene";
import { Graphics, IFillStyleOptions, ILineStyleOptions } from "@pixi/graphics";
import {
  createEffect,
  createMemo,
  on,
  onMount,
  useContext,
} from "solid-js";
import { metadata } from "../utils/metadata";
import { AppContext } from "../store";
import {
  CreateObjectCommand,
  DeleteObjectCommand,
  MultiCommand,
} from "../store/commands";
import { Point } from "@pixi/core";
import { newUuid, uuid } from "../utils/uuid";
import { useHoverSelectOutline } from "../composables/useHoverSelectOutline";

const updateGraphics = (
  g: Graphics,
  shape: GraphicsNode[],
  fill: IFillStyleOptions,
  stroke: ILineStyleOptions,
) => {
  g.clear();

  // Stack stores up to 2 previous control points.
  let stackIndex = 0;
  const stack: [GraphicsNode | undefined, GraphicsNode | undefined] = [
    undefined,
    undefined,
  ];

  g.beginFill(fill.color, fill.alpha);
  g.lineStyle(stroke);

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

type GraphicSceneObjectViewProps = GraphicSceneObject & {
  order: number;
};
export const GraphicSceneObjectView = (props: GraphicSceneObjectViewProps) => {
  let graphics: Graphics | undefined;
  onMount(() => {
    if (!graphics) return;
    graphics.filters = [];
    metadata.set(graphics, {
      type: props.type,
      id: props.id,
    });

    useHoverSelectOutline(graphics, props);
  });

  const { sceneStore } = useContext(AppContext);

  createEffect(
    on([
      () => props.shape,
      () => props.fill,
      () => props.stroke,
    ], ([shape, fill, stroke]) => {
      if (graphics) updateGraphics(graphics, shape, fill, stroke);
    }),
  );

  const isAppInspecting = createMemo(() => sceneStore.inspecting !== undefined);
  const isThisInspecting = createMemo(() => isAppInspecting() && sceneStore.inspecting === props.id);

  return (
    <P.Graphics
      name={`${props.id} ${props.name}`}
      visible={props.visible}
      ref={graphics}
      zIndex={sceneStore.inspecting === props.id ? 500 : props.order}
      position={props.position}
      interactive={!isAppInspecting() || isThisInspecting()}
      alpha={!isAppInspecting() || isThisInspecting() ? 1 : 0.5}
    >
      <SceneObjectChildren children={props.children} />
    </P.Graphics>
  );
};
