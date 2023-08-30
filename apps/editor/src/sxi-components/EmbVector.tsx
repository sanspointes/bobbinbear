import { P } from "@bearbroidery/solixi";
import { SceneObjectChildren } from "./general";
import {
  RealNode,
  EmbNodeType,
  EmbVector as EmbVector,
  VectorNode,
  EmbNode,
  EmbHasVirtual,
} from "../types/scene";
import { Point } from "@pixi/core";
import { Graphics, IFillStyleOptions, ILineStyleOptions } from "@pixi/graphics";
import { createEffect, createMemo, createRenderEffect, createSignal, For, on, onMount, useContext } from "solid-js";
import { AppContext } from "../store";
import { useHoverSelectOutline } from "../composables/useHoverSelectOutline";
import { Show } from "solid-js";
import { sceneObjectDefaults } from "../store/helpers";
import { arrayFirst, arrayIterCircularEndInclusive, arrayIterPairs } from "../utils/array";
import { lerp } from "../utils/math";
import { newUuid, Uuid } from "../utils/uuid";
import { EmbNodeView } from "./EmbNode";
import { MutateSceneObjectArrayFieldCommand } from "../store/commands";
import { mapTemporarySceneObjects } from "../composables/useVirtualSceneObjects";
import { Container } from "@pixi/display";

type ExtraOptions = {
  close: boolean;
};
const updateGraphics = (
  g: Graphics,
  shape: VectorNode[],
  fill: IFillStyleOptions,
  stroke: ILineStyleOptions,
  extra: ExtraOptions,
) => {
  g.clear();

  // Stack stores up to 2 previous control points.
  let stackIndex = 0;
  const stack: [VectorNode | undefined, VectorNode | undefined] = [
    undefined,
    undefined,
  ];

  g.beginFill(fill.color, fill.alpha);
  g.lineStyle(stroke);

  for (const node of shape) {
    // Jump nodes jump to a new position
    if (node.type === EmbNodeType.Jump) {
      g.moveTo(node.x, node.y);
      // Control points are stored to be used for curves
    } else if (node.type === EmbNodeType.Control) {
      if (stackIndex >= stack.length) {
        throw new Error(
          "updateGraphics: Received more than 2 control nodes in a row.",
        );
      }
      stack[stackIndex] = node;
      stackIndex += 1;
      // When a Point is found either straight line or curve to it.
    } else if (node.type === EmbNodeType.Point) {
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
  if (extra.close) {
    const node = arrayFirst(shape)!;
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
  }

  g.endFill();
};

type EmbVectorProps = EmbVector & {
  order: number;
};
export const EmbVectorView = (props: EmbVectorProps) => {
  let container: Container | undefined;
  let graphics: Graphics | undefined;
  onMount(() => {
    if (!graphics) return;
    graphics.filters = [];

    useHoverSelectOutline(graphics, props);
  });

  const { sceneStore } = useContext(AppContext);

  createEffect(() => {
    if (graphics) {
      updateGraphics(graphics, props.shape, props.fill, props.stroke, {
        close: props.close,
      });
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
        ...sceneObjectDefaults<EmbNode>(),
        id: node.id as unknown as Uuid<EmbNode>,
        type: "node",
        node,
        name: `${node.type} Node`,
        position: new Point(node.x, node.y),
        relatesTo: props.id as Uuid<EmbVector>,
      } as EmbNode;
    },
  );

  const pointNodePairs = createMemo(() => {
    return isThisInspecting()
      ? [...arrayIterPairs(props.shape, true)]
      : undefined;
  });

  const virtualNodes = mapTemporarySceneObjects(
    () => pointNodePairs(),
    ([prev, node], i) => {
      if (
        node.type === EmbNodeType.Control ||
        prev.type === EmbNodeType.Control
      ) return undefined;
      const midX = lerp(prev.x, node.x, 0.5);
      const midY = lerp(prev.y, node.y, 0.5);
      const id = newUuid<EmbNode>();
      const midNode: VectorNode = {
        type: EmbNodeType.Point,
        x: midX,
        y: midY,
        id: id as unknown as Uuid<VectorNode>,
      };

      const midObject: EmbNode & EmbHasVirtual = {
        ...sceneObjectDefaults(),
        id,
        virtual: true,
        virtualCreator: () => {
          const cmd = new MutateSceneObjectArrayFieldCommand<
            EmbVector
          >(
            props.id as Uuid<EmbVector>,
            "shape",
            i() + 1,
            {
              toDelete: 0,
              toInsert: [midNode],
              circularInsert: true,
            }
          );
          return cmd;
        },
        position: new Point(midX, midY),
        type: "node",
        node: midNode,
        name: `Virtual ${i()}`,
        relatesTo: props.id as Uuid<EmbVector>,
      };
      return midObject;
      // });
      // return data;
    },
  );

  let overlayGraphics: Graphics | undefined;
  createRenderEffect(() => {
    if (isThisInspecting() && overlayGraphics) {
      overlayGraphics.clear();
      overlayGraphics.lineStyle(1, 0x000000, 1);

      let needsLineNext = false;
      let prev: VectorNode | undefined;
      for (const node of arrayIterCircularEndInclusive(props.shape)) {
      // for (const node of props.shape) {
        const n = node as RealNode;
        if ((n.ownsPrev || needsLineNext) && prev ) {
          needsLineNext = false;
          overlayGraphics.moveTo(n.x, n.y);
          overlayGraphics.lineTo(prev.x, prev.y);
        }
        if (n.ownsNext) {
          needsLineNext = true;
        }
        prev = node;
      }
    } 
  });

  return (
    <P.Container
      id={props.id}
      ref={container}
      zIndex={sceneStore.inspecting === props.id ? 500 : props.order}
      position={props.position}
    >
      <P.Graphics
        id={props.id}
        soType={props.type}
        name={`${props.id} ${props.name}`}
        visible={props.visible}
        ref={graphics}
        interactive={!isAppInspecting() || isThisInspecting()}
        alpha={!isAppInspecting() || isThisInspecting() ? 1 : 0.5}
       />
      <SceneObjectChildren children={props.children} />
      <P.Graphics
        ref={overlayGraphics}
        visible={isThisInspecting()}
      />
      {/* Shows nodes to edit the shape */}
      <Show when={editableNodes()}>
        {(editableNodes) => (
          <For each={editableNodes()}>
            {(nodeSceneObject, i) => (
              <EmbNodeView {...nodeSceneObject()} order={i()} />
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
                {(props) => <EmbNodeView {...props()} order={0} />}
              </Show>
            )}
          </For>
        )}
      </Show>
    </P.Container>
  );
};
