import { P } from "@bearbroidery/solixi";
import { Point, Texture } from "@pixi/core";
import { Container } from "@pixi/display";
import { Graphics, ILineStyleOptions } from "@pixi/graphics";
import { MeshGeometry, MeshMaterial } from "@pixi/mesh";
import {
    createEffect,
    createMemo,
    createRenderEffect,
    Show,
    useContext,
} from "solid-js";
import { AppContext } from "../../store";
import { EmbState } from "../shared";
import {
    EmbVecSeg,
    isBezierVecSeg,
    isLineVecSeg,
    isQuadraticVecSeg,
    VectorSegment,
} from "./shared";
import { useTemporarySceneObject } from "../../composables/useVirtualSceneObjects";
import { EmbNode, EmbNodeView } from "../node";

const updateGraphics = (
    g: Graphics,
    segment: VectorSegment,
    stroke: ILineStyleOptions,
) => {
    g.clear();

    g.lineStyle(stroke);
    const { x, y } = segment.from;
    g.moveTo(x, y);

    if (isLineVecSeg(segment)) {
        const { to } = segment;
        g.lineTo(to.x, to.y);
    } else if (isQuadraticVecSeg(segment)) {
        const { c0, to } = segment;
        g.quadraticCurveTo(c0.x, c0.y, to.x, to.y);
    } else if (isBezierVecSeg(segment)) {
        const { c0, c1, to } = segment;
        g.bezierCurveTo(c0.x, c0.y, c1.x, c1.y, to.x, to.y);
    }
    g.closePath();
    g.geometry.updateBatches();
};

const HIT_GEOMETRY_LINE_STYLE: ILineStyleOptions = {
    color: 0x000000,
    width: 3,
};
const HOVER_LINE_STYLE: ILineStyleOptions = {
    color: 0x41A3E9,
    width: 1,
};
const SELECT_LINE_STYLE: ILineStyleOptions = {
    color: 0x41A3E9,
    width: 2,
};

type EmbVecSegProps = EmbVecSeg & EmbState & {
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
            const posBuffer = geometry.getBuffer("aVertexPosition");
            posBuffer.data = new Float32Array(graphics.geometry.points);
            posBuffer.update();

            const coordBuffer = geometry.getBuffer("aTextureCoord");
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
                updateGraphics(highlightGraphics, props.segment, SELECT_LINE_STYLE);
            } else if (props.hovered) {
                updateGraphics(highlightGraphics, props.segment, HOVER_LINE_STYLE);
            }
        }
    });

    const endNodeModel = createMemo(() => {
        if (props.inspecting) {
            const { to } = props.segment;
            const nodeData: EmbNode = {
                node: to,
                type: "node",
                id: to.id,
                position: new Point(to.x, to.y),
                parent: props.id,
                children: [],
                relatesTo: props.id,
            };
            const model = useTemporarySceneObject(nodeData);
            console.log("end node model ", model);
            return model;
        }
    });

    return (
        <P.Container
            id={props.id}
            ref={container}
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
            />
            <P.Graphics
                ref={highlightGraphics}
                visible={props.hovered || props.selected}
            />
            <Show when={endNodeModel()}>
                {(props) => <EmbNodeView {...props()} order={0} />}
            </Show>
        </P.Container>
    );
};
