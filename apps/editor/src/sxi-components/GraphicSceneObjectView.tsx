import { P } from "@bearbroidery/solixi";
import { SceneObjectChildren } from "./general";
import {
  BaseSceneObject,
  GraphicNodeTypes,
  GraphicSceneObject,
  GraphicsNode,
  NodeSceneObject,
  VirtualSceneObject,
} from "../types/scene";
import { Point } from "@pixi/core";
import { Graphics, IFillStyleOptions, ILineStyleOptions } from "@pixi/graphics";
import { createEffect, createMemo, For, onMount, useContext } from "solid-js";
import { AppContext } from "../store";
import { useHoverSelectOutline } from "../composables/useHoverSelectOutline";
import { Show } from "solid-js";
import { sceneObjectDefaults } from "../store/helpers";
import { arrayIterPairs } from "../utils/array";
import { lerp } from "../utils/math";
import { newUuid, Uuid } from "../utils/uuid";
import { NodeSceneObjectView } from "./NodeSceneObjectView";
import { Command, MutateSceneObjectArrayFieldCommand } from "../store/commands";
import { mapTemporarySceneObjects } from "../composables/useVirtualSceneObjects";

type ExtraOptions = {
  close: boolean;
}
const updateGraphics = (
  g: Graphics,
  shape: GraphicsNode[],
  fill: IFillStyleOptions,
  stroke: ILineStyleOptions,
  extra: ExtraOptions,
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
      } else if (stackIndex === 2) {
        const c0 = stack[0]!;
        const c1 = stack[1]!;
        g.bezierCurveTo(c0.x, c0.y, c1.x, c1.y, node.x, node.y);
      }

      stackIndex = 0;
    }
  }
  if (extra.close) g.closePath();
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

    useHoverSelectOutline(graphics, props);
  });

  const { sceneStore } = useContext(AppContext);

  createEffect(() => {
    if (graphics) {
      updateGraphics(graphics, props.shape, props.fill, props.stroke, { close: props.close });
    }
  });

  const isAppInspecting = createMemo(() => sceneStore.inspecting !== undefined);
  const isThisInspecting = createMemo(() =>
    isAppInspecting() && sceneStore.inspecting === props.id
  );

  const editableNodes = mapTemporarySceneObjects(
    () => isThisInspecting() ? props.shape : undefined,
    (node) => {
      return {
        ...sceneObjectDefaults<NodeSceneObject>(),
        id: node.id as unknown as Uuid<NodeSceneObject>,
        type: "node",
        node,
        name: `${node.type} Node`,
        position: new Point(node.x, node.y),
        relatesTo: props.id as Uuid<GraphicSceneObject>,
      } as NodeSceneObject;
    },
  );

  const pointNodePairs = createMemo(() => {
    return isThisInspecting() ? [...arrayIterPairs(props.shape, true)] : undefined;
  });

  createEffect(() => {
    console.log(pointNodePairs());
  })

  const virtualNodes = mapTemporarySceneObjects(
    () => pointNodePairs(),
    ([prev, node], i) => {
      if (
        node.type === GraphicNodeTypes.Control ||
        prev.type === GraphicNodeTypes.Control
      ) return undefined;
      const midX = lerp(prev.x, node.x, 0.5);
      const midY = lerp(prev.y, node.y, 0.5);
      console.log(prev.x, midX, node.x);
      console.log(prev.y, midY, node.y);
      const id = newUuid<NodeSceneObject>();
      const midNode: GraphicsNode = {
        type: GraphicNodeTypes.Point,
        x: midX,
        y: midY,
        id: id as unknown as Uuid<GraphicsNode>,
      };

      const midObject: NodeSceneObject & VirtualSceneObject = {
        ...sceneObjectDefaults(),
        id,
        virtual: true,
        virtualCreator: () => {
          const cmd = new MutateSceneObjectArrayFieldCommand<
            GraphicSceneObject
          >(
            props.id as Uuid<GraphicSceneObject>,
            "shape",
            (i() + 1),
            0,
            [midNode],
          );
          return cmd as Command<BaseSceneObject>;
        },
        position: new Point(midX, midY),
        type: "node",
        node: midNode,
        name: `Virtual ${i()}`,
        relatesTo: props.id as Uuid<GraphicSceneObject>,
      };
      return midObject;
      // });
      // return data;
    },
  );

  return (
    <P.Graphics
      id={props.id}
      soType={props.type}
      name={`${props.id} ${props.name}`}
      visible={props.visible}
      ref={graphics}
      zIndex={sceneStore.inspecting === props.id ? 500 : props.order}
      position={props.position}
      interactive={!isAppInspecting() || isThisInspecting()}
      alpha={!isAppInspecting() || isThisInspecting() ? 1 : 0.5}
    >
      <SceneObjectChildren children={props.children} />
      {/* Shows nodes to edit the shape */}
      <Show when={editableNodes()}>
        {(editableNodes) => (
          <For each={editableNodes()}>
            {(nodeSceneObject, i) => (
              <NodeSceneObjectView {...nodeSceneObject()} order={i()} />
            )}
          </For>
        )}
      </Show>
      {/* Shows virtual nodes that can be created to add nodes to shape */}
      <Show when={virtualNodes()}>
        {(virtualNodes) => (
          <For each={virtualNodes()}>
            {(nodeSceneObject) => (
              <Show when={nodeSceneObject()}>
                {(props) => <NodeSceneObjectView {...props()} order={0} />}
              </Show>
            )}
          </For>
        )}
      </Show>
    </P.Graphics>
  );
};
