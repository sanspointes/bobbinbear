import { Container } from "@pixi/display";
import { Mesh, MeshMaterial, MeshGeometry } from "@pixi/mesh";
import { PlaneGeometry } from "@pixi/mesh-extras";
import { Texture } from "@pixi/core";
import { GraphicsGeometry } from "@pixi/graphics";
import { Solixi } from "./state";
import { HasPositionFragment, HasRotationFragment, HasScaleFragment, HasVisibilityFragment } from "./prop-fragments";

export const PContainer = Solixi.wrapConstructable(Container, {
  // @ts-expect-error ; Hard to type parent of attach function
  attach: (_, b: Container, c) => {
    b.addChild(c);
    return () => b.removeChild(c);
  },
  defaultArgs: [],
  extraProps: {
    ...HasPositionFragment,
    ...HasScaleFragment,
    ...HasVisibilityFragment,
    ...HasRotationFragment,
  }
});

export const PMesh = Solixi.wrapConstructable(Mesh<MeshMaterial>, {
  // @ts-expect-error ; Hard to type parent of attach function
  attach: (_, b: Container, c) => {
    b.addChild(c);
    return () => b.removeChild(c);
  },
  defaultArgs: [new PlaneGeometry(), new MeshMaterial(Texture.WHITE)],
  // @ts-expect-error; Erroring because it's not wrapping a Container (base of Mesh, used for Position/scale/visibility/rotation props).
  // Need to figure out how to permit classes that extend the base class of the prop fragments.
  extraProps: {
    ...HasPositionFragment,
    ...HasScaleFragment,
    ...HasVisibilityFragment,
    ...HasRotationFragment,
  }
});

export const PPlaneGeometry = Solixi.wrapConstructable(PlaneGeometry, {
  attach: 'geometry',
  defaultArgs: [1, 1, 1, 1],
  extraProps: {

  }
})

export const PGraphicsGeometry = Solixi.wrapConstructable(GraphicsGeometry, {
  attach: 'geometry',
  defaultArgs: [],
  extraProps: {
  }
})

export const PMeshGeometry = Solixi.wrapConstructable(MeshGeometry, {
  attach: 'geometry',
  defaultArgs: [],
  extraProps: {},
});

export const PMeshMaterial = Solixi.wrapConstructable(MeshMaterial, {
  attach: 'material',
  defaultArgs: [Texture.WHITE],
  extraProps: {

  }
})
