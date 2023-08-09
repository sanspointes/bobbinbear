import { Container as PixiContainer } from "@pixi/display";
import {
  Mesh as PixiMesh,
  MeshGeometry as PixiMeshGeometry,
  MeshMaterial as PixiMeshMaterial,
} from "@pixi/mesh";
import { PlaneGeometry as PixiPlaneGeometry } from "@pixi/mesh-extras";
import { Texture } from "@pixi/core";
import { Graphics as PixiGraphics, GraphicsGeometry as PixiGraphicsGeometry } from "@pixi/graphics";
import { Solixi, SolixiState } from "./state";
import {
  HasPositionFragment,
  HasRotationFragment,
  HasScaleFragment,
  HasVisibilityFragment,
} from "./prop-fragments";
import { ClassProps } from "@bearbroidery/constructables";

export const temp = (a: number) => a + 1;

const ContainerExtraProps = {
  ...HasPositionFragment,
  ...HasScaleFragment,
  ...HasVisibilityFragment,
  ...HasRotationFragment,
};
export type ContainerProps = ClassProps<SolixiState, typeof PixiContainer, typeof ContainerExtraProps>;
export const Container = Solixi.wrapConstructable(PixiContainer, {
  // @ts-expect-error ; Hard to type parent of attach function
  attach: (_, b: PixiContainer, c) => {
    b.addChild(c);
    return () => b.removeChild(c);
  },
  defaultArgs: [],
  extraProps: ContainerExtraProps,
});

/**
 * Mesh
 */
const MeshExtraProps = {
  ...HasPositionFragment,
  ...HasScaleFragment,
  ...HasVisibilityFragment,
  ...HasRotationFragment,
};
export type MeshProps = ClassProps<SolixiState, typeof PixiMesh, typeof MeshExtraProps>;
export const Mesh = Solixi.wrapConstructable(PixiMesh<PixiMeshMaterial>, {
  // @ts-expect-error ; Hard to type parent of attach function
  attach: (_, b: PixiContainer, c) => {
    b.addChild(c);
    return () => b.removeChild(c);
  },
  defaultArgs: [new PixiPlaneGeometry(), new PixiMeshMaterial(Texture.WHITE)],
  // Need to figure out how to permit classes that extend the base class of the prop fragments.
  extraProps: {
    ...HasPositionFragment,
    ...HasScaleFragment,
    ...HasVisibilityFragment,
    ...HasRotationFragment,
  },
});

export const PlaneGeometry = Solixi.wrapConstructable(PixiPlaneGeometry, {
  attach: "geometry",
  defaultArgs: [1, 1, 1, 1],
  extraProps: {},
});

export const GraphicsGeometry = Solixi.wrapConstructable(PixiGraphicsGeometry, {
  attach: "geometry",
  defaultArgs: [],
  extraProps: {},
});

export const MeshGeometry = Solixi.wrapConstructable(PixiMeshGeometry, {
  attach: "geometry",
  defaultArgs: [],
  extraProps: {},
});

export const MeshMaterial = Solixi.wrapConstructable(PixiMeshMaterial, {
  attach: "material",
  defaultArgs: [Texture.WHITE],
  extraProps: {},
});

export const Graphics = Solixi.wrapConstructable(PixiGraphics, {
  // @ts-expect-error ; Hard to type parent of attach function
  attach: (_, b: PixiContainer, c) => {
    b.addChild(c);
    return () => b.removeChild(c);
  },
  defaultArgs: [new PixiGraphicsGeometry()],
  extraProps: {
    ...HasPositionFragment,
    ...HasScaleFragment,
    ...HasVisibilityFragment,
    ...HasRotationFragment,
  },
})
