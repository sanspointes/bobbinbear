import { P } from '@bearbroidery/solixi';
import { Sprite } from '@pixi/sprite';
import { onMount } from 'solid-js';

import { useTexture } from '../../composables/useAsset';
import { EmbHasVirtual } from '../shared';

import { ObservablePoint, Point, Texture } from '@pixi/core';
import NodeControlSrc from '../../assets/node_control.png';
import NodePointSrc from '../../assets/node_point.png';
import { useHoverSelectOutline } from '../../composables/useHoverSelectOutline';
import { EmbNode, VectorNodeType } from './shared';

const NODE_Z_INDEX = -100;

const NodeTypeImageMap: Record<VectorNodeType, string> = {
    [VectorNodeType.Point]: NodePointSrc,
    [VectorNodeType.Control]: NodeControlSrc,
};

const CENTER_ANCHOR = new Point(0.5, 0.5) as unknown as ObservablePoint;

type EmbNodeProps = EmbNode & {
    order: number;
} & Partial<EmbHasVirtual>;

export function EmbNodeView(props: EmbNodeProps) {
    const [texture] = useTexture({
        src: NodeTypeImageMap[props.node.type],
        fallback: Texture.EMPTY,
    });
    let sprite: Sprite | undefined;
    onMount(() => {
        if (!sprite) return;
        useHoverSelectOutline(sprite, props);
    });

    return (
        <P.Sprite
            id={props.id}
            soType={props.type}
            ref={sprite}
            anchor={CENTER_ANCHOR}
            scale-x={0.1}
            scale-y={0.1}
            alpha={props.virtual ? 0.5 : 1}
            zIndex={NODE_Z_INDEX + props.order}
            position={props.position}
            texture={texture()}
            interactive={true}
        >
            {/*<P.Text text={`${props.node.type}:${props.id} ${props.order}`} scale={[5, 5]} /> */}
        </P.Sprite>
    );
}
