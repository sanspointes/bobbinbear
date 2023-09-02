import { P } from "@bearbroidery/solixi";
import { Geometry, Texture } from "@pixi/core";
import { Container } from "@pixi/display";
import { Graphics, ILineStyleOptions } from "@pixi/graphics";
import { MeshGeometry, MeshMaterial } from "@pixi/mesh";
import { createEffect, createRenderEffect, useContext } from "solid-js";
import { AppContext } from "../../store";
import { EmbState } from "../shared";
import {
  EmbVecSeg,
  isBezierVecSeg,
  isLineVecSeg,
  isQuadraticVecSeg,
  VectorSegment,
} from "./shared";

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
  const geometry = new MeshGeometry();
  const graphics = new Graphics();
  const material = new MeshMaterial(Texture.WHITE);
  createRenderEffect(() => {
    material.tint = props.stroke.color ?? 0xffffff;
  });

  createEffect(() => {
    updateGraphics(graphics, props.segment, props.stroke);
    if (geometry) {
      geometry.addAttribute("aVertexPosition", graphics.geometry.points, 2);
      geometry.addAttribute("aTextureCoord", graphics.geometry.uvs, 2);
      geometry.addIndex(graphics.geometry.indices);
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
        interactive={true}
      />
    </P.Container>
  );
};
