import { createEffect, onMount } from "solid-js";
import { Sprite } from "@pixi/sprite";
import { P } from "@bearbroidery/solixi";

import { GraphicNodeTypes, NodeSceneObject } from "../types/scene";
import { useTexture } from "../composables/useAsset";

import NodePointSrc from "../assets/node_point.png";
import NodeControlSrc from "../assets/node_point.png";
import { Circle, ObservablePoint, Point, Texture } from "@pixi/core";
import { metadata } from "../utils/metadata";
import { useHoverSelectOutline } from "../composables/useHoverSelectOutline";

const NodeTypeImageMap: Record<GraphicNodeTypes, string> = {
  [GraphicNodeTypes.Jump]: NodePointSrc,
  [GraphicNodeTypes.Point]: NodePointSrc,
  [GraphicNodeTypes.Control]: NodeControlSrc,
};

type NodeSceneObjectViewProps = NodeSceneObject & {
  order: number;
};
const CENTER_ANCHOR = new Point(0.5, 0.5) as unknown as ObservablePoint;
const HIT_AREA = new Circle(0, 0, 128);

export function NodeSceneObjectView(props: NodeSceneObjectViewProps) {
  const [texture] = useTexture({
    src: NodeTypeImageMap[props.node.type],
    fallback: Texture.EMPTY,
  });
  let sprite: Sprite | undefined;
  onMount(() => {
    if (!sprite) return;
    metadata.set(sprite, {
      id: props.id,
      type: 'node',
    })
    useHoverSelectOutline(sprite, props);
  });

  return (
    <P.Sprite
      ref={sprite}
      anchor={CENTER_ANCHOR}
      scale-x={0.1}
      scale-y={0.1}
      zIndex={props.order}
      position={props.position}
      hitArea={HIT_AREA}
      texture={texture()}
      interactive={true}
    />
  );
}
