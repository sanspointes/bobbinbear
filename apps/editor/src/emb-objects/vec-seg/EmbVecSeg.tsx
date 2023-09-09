import { P } from '@bearbroidery/solixi';
import { Point, Texture } from '@pixi/core';
import { Container } from '@pixi/display';
import {
    FillStyle,
    Graphics,
    GraphicsGeometry,
    ILineStyleOptions,
    LineStyle,
} from '@pixi/graphics';
import { MeshGeometry, MeshMaterial } from '@pixi/mesh';
import {
    createEffect,
    createMemo,
    createRenderEffect,
    Show,
    useContext,
} from 'solid-js';
import { AppContext } from '../../store';
import { EmbState } from '../shared';
import { BezierToVectorSegment, EmbVecSeg, VectorSegment } from './shared';
import { useTemporarySceneObject } from '../../composables/useVirtualSceneObjects';
import { EmbNode, EmbNodeView } from '../node';
import { SegmentUtils } from '.';

const updateGraphics = (
    g: Graphics,
    segment: VectorSegment,
    stroke: ILineStyleOptions,
) => {
    g.clear();

    g.lineStyle(stroke);
    if (segment.prev) {
        const { x, y } = segment.prev.to;
        g.moveTo(x, y);
    }

    if (SegmentUtils.isLine(segment)) {
        const { to } = segment;
        g.lineTo(to.x, to.y);
    } else if (SegmentUtils.isQuadratic(segment)) {
        const { c0, to } = segment;
        g.quadraticCurveTo(c0.x, c0.y, to.x, to.y);
    } else if (SegmentUtils.isBezier(segment)) {
        const { c0, c1, to } = segment;
        g.bezierCurveTo(c0.x, c0.y, c1.x, c1.y, to.x, to.y);
    }
    g.finishPoly();
    g.geometry.updateBatches();
};

const HIT_GEOMETRY_LINE_STYLE: ILineStyleOptions = {
    color: 0x000000,
    width: 3,
};
const HOVER_LINE_STYLE: ILineStyleOptions = {
    color: 0x41a3e9,
    width: 1,
};
const SELECT_LINE_STYLE: ILineStyleOptions = {
    color: 0x41a3e9,
    width: 2,
};
const HANDLE_LINE_STLE: ILineStyleOptions = {
    color: 0x000000,
    width: 1,
};

type EmbVecSegProps = EmbVecSeg &
    EmbState & {
        order: number;
    };
/**
 * Component that displays an EmbVecSeg model.
 */
export const EmbVecSegView = (props: EmbVecSegProps) => {
    const { sceneStore } = useContext(AppContext);
    let container: Container | undefined;
    const graphics = new Graphics();
    let highlightGraphics: Graphics | undefined;
    const geometry = new MeshGeometry();
    const material = new MeshMaterial(Texture.WHITE);
    createRenderEffect(() => {
        material.tint = props.line.color ?? 0xff0000;
    });

    // On line change, update the hitbox mesh
    createEffect(() => {
        if (!graphics) return;
        updateGraphics(graphics, props.segment, HIT_GEOMETRY_LINE_STYLE);
        if (geometry) {
            const posBuffer = geometry.getBuffer('aVertexPosition');
            posBuffer.data = new Float32Array(graphics.geometry.points);
            posBuffer.update();

            const coordBuffer = geometry.getBuffer('aTextureCoord');
            coordBuffer.data = new Float32Array(graphics.geometry.uvs);
            coordBuffer.update();

            const indexBuffer = geometry.getIndex();
            indexBuffer.data = new Float32Array(graphics.geometry.indices);
            indexBuffer.update();
        }
    });

    // Rerender the highlight graphic
    createEffect(() => {
        if (highlightGraphics) {
            if (props.selected) {
                updateGraphics(
                    highlightGraphics,
                    props.segment,
                    SELECT_LINE_STYLE,
                );
            } else if (props.hovered) {
                updateGraphics(
                    highlightGraphics,
                    props.segment,
                    HOVER_LINE_STYLE,
                );
            }
        }
    });

    const endNodeModel = createMemo(() => {
        if (props.inspecting) {
            const { to } = props.segment;
            const nodeData: EmbNode = {
                node: to,
                type: 'node',
                id: to.id,
                position: new Point(to.x, to.y),
                parent: props.id,
                children: [],
                relatesTo: props.id,
            };
            const model = useTemporarySceneObject(nodeData);
            return model;
        }
    });

    const c0NodeModel = createMemo(() => {
        const seg = props.segment as VectorSegment &
            Partial<BezierToVectorSegment>;
        if (props.inspecting && seg.c0) {
            const { c0 } = seg;
            const nodeData: EmbNode = {
                node: c0,
                type: 'node',
                id: c0.id,
                position: new Point(c0.x, c0.y),
                parent: props.id,
                children: [],
                relatesTo: props.id,
            };
            const model = useTemporarySceneObject(nodeData);
            return model;
        }
    });

    const c1NodeModel = createMemo(() => {
        const seg = props.segment as VectorSegment &
            Partial<BezierToVectorSegment>;
        if (props.inspecting && seg.c1) {
            const { c1 } = props.segment;
            const nodeData: EmbNode = {
                node: c1,
                type: 'node',
                id: c1.id,
                position: new Point(c1.x, c1.y),
                parent: props.id,
                children: [],
                relatesTo: props.id,
            };
            const model = useTemporarySceneObject(nodeData);
            return model;
        }
    });

    let lineGraphic: Graphics | undefined;
    createEffect(() => {
        if (props.inspecting && lineGraphic) {
            lineGraphic.clear();
            const polygon = SegmentUtils.generateControlPolygon(props.segment);
            if (polygon) {
                lineGraphic.lineStyle(HANDLE_LINE_STLE);
                lineGraphic.drawShape(polygon);
            }
        }
    });

    return (
        <P.Container
            ref={container}
            name={`${props.id} Container`}
            zIndex={sceneStore.inspecting === props.id ? 500 : props.order}
            position={props.position}
        >
            <P.Mesh
                args={[geometry, material]}
                id={props.id}
                soType={props.type}
                name={`${props.id} ${props.name}`}
                visible={props.visible}
                interactive={props.inspecting}
                alpha={0}
                onpointerover={(e) => console.log(e)}
            />
            <P.Graphics
                ref={highlightGraphics}
                visible={props.hovered || props.selected}
            />
            <Show when={props.inspecting}>
                <P.Graphics ref={lineGraphic} />
            </Show>

            <Show when={endNodeModel()}>
                {(props) => <EmbNodeView {...props()} order={0} />}
            </Show>
            <Show when={c0NodeModel()}>
                {(props) => <EmbNodeView {...props()} order={0} />}
            </Show>
            <Show when={c1NodeModel()}>
                {(props) => <EmbNodeView {...props()} order={0} />}
            </Show>
        </P.Container>
    );
};
