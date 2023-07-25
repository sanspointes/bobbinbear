import {
    Attribute,
  Color as PixiColor,
  ColorSource,
  Container,
  Geometry,
  IMeshMaterialOptions,
  Mesh,
  MeshGeometry,
  MeshMaterial,
  PlaneGeometry,
  Point as PixiPoint,
  Program,
  Shader,
  Sprite,
  Texture,
} from "pixi.js";
import { EventHandlers } from "./core/events";
import { AttachType } from "./core/renderer";
import { JSX } from "solid-js";
export type NonFunctionKeys<T> = {
  [K in keyof T]: T[K] extends Function ? never : K;
}[keyof T];
export type Overwrite<T, O> = Omit<T, NonFunctionKeys<O>> & O;

/**
 * If **T** contains a constructor, @see ConstructorParameters must be used, otherwise **T**.
 */
type Args<T> = T extends new (...args: any) => any ? ConstructorParameters<T>
  : T;

export type Point = PixiPoint | Parameters<PixiPoint["set"]>;
export type Color =
  | ConstructorParameters<typeof PixiColor>
  | PixiColor
  | number
  | string; // Parameters<T> will not work here because of multiple function signatures in three.js types
// export type Layers = THREE.Layers | Parameters<THREE.Layers["set"]>[0];
// export type Quaternion = THREE.Quaternion | Parameters<THREE.Quaternion["set"]>;

export type AttachCallback =
  | string
  | ((child: any, parentInstance: any) => void);

export interface NodeProps<T, P> {
  attach?: AttachType;
  /** Constructor arguments */
  args?: Args<P>;
  children?: any;
  ref?: T | ((instance: T) => void);
  // key?: React.Key;
  onUpdate?: (self: T) => void;
}

export type Node<T, P> = Overwrite<Partial<T>, NodeProps<T, P>>;

export type ContainerNode<T, P> =
  & Overwrite<
    Node<T, P>,
    {
      position?: Point;
      scale?: Point;
      rotation?: number;
      // transform?: Transform;
      // quaternion?: Quaternion;
      // layers?: Layers;
      dispose?: (() => void) | null;

      geometry?: JSX.Element | Geometry | null;
      material?: JSX.Element | Shader | MeshMaterial | null;
    }
  >
  & EventHandlers;

export type GeometryNode<T extends Geometry, P> = Overwrite<
  Node<T, P>,
  {}
>;
export type MaterialNode<T extends MeshMaterial | Shader, P> = Overwrite<
  Node<T, P>,
  { color?: Color }
>;

// export type AudioProps = Object3DNode<THREE.Audio, typeof THREE.Audio>
// export type AudioListenerProps = ContainerNode<
//   THREE.AudioListener,
//   typeof THREE.AudioListener
// >;
// export type PositionalAudioProps = ContainerNode<
//   THREE.PositionalAudio,
//   typeof THREE.PositionalAudio
// >;

export type ContainerProps = ContainerNode<Container, typeof Container>;
export type MeshProps = ContainerNode<Mesh, typeof Mesh>;
// export type InstancedMeshProps = ContainerNode<
//   THREE.InstancedMesh,
//   typeof THREE.InstancedMesh
// >;
// export type SceneProps = ContainerNode<THREE.Scene, typeof THREE.Scene>;
export type SpriteProps = ContainerNode<Sprite, typeof Sprite>;
// export type LODProps = ContainerNode<THREE.LOD, typeof THREE.LOD>;
// export type SkinnedMeshProps = ContainerNode<
//   THREE.SkinnedMesh,
//   typeof THREE.SkinnedMesh
// >;
//
// export type SkeletonProps = ContainerNode<THREE.Skeleton, typeof THREE.Skeleton>;
// export type BoneProps = ContainerNode<THREE.Bone, typeof THREE.Bone>;
// export type LineSegmentsProps = ContainerNode<
//   THREE.LineSegments,
//   typeof THREE.LineSegments
// >;
// export type LineLoopProps = ContainerNode<THREE.LineLoop, typeof THREE.LineLoop>;
// // export type LineProps = Object3DNode<THREE.Line, typeof THREE.Line>
// export type PointsProps = ContainerNode<THREE.Points, typeof THREE.Points>;
// export type GroupProps = ContainerNode<THREE.Group, typeof THREE.Group>;

export type MeshGeometryProps = GeometryNode<
  MeshGeometry,
  typeof MeshGeometry
>;
export type PlaneGeometryProps = GeometryNode<
  PlaneGeometry,
  typeof PlaneGeometry
>;

export type MeshMaterialProps = MaterialNode<
  MeshMaterial,
  [Texture, IMeshMaterialOptions]
>;
export type ShaderProps = MaterialNode<
  Shader,
  [Program, ]
>;
// export type SpriteMaterialProps = MaterialNode<
//   THREE.SpriteMaterial,
//   [THREE.SpriteMaterialParameters]
// >;
// export type RawShaderMaterialProps = MaterialNode<
//   THREE.RawShaderMaterial,
//   [THREE.ShaderMaterialParameters]
// >;
// export type ShaderMaterialProps = MaterialNode<
//   THREE.ShaderMaterial,
//   [THREE.ShaderMaterialParameters]
// >;
// export type PointsMaterialProps = MaterialNode<
//   THREE.PointsMaterial,
//   [THREE.PointsMaterialParameters]
// >;
// export type MeshPhysicalMaterialProps = MaterialNode<
//   THREE.MeshPhysicalMaterial,
//   [THREE.MeshPhysicalMaterialParameters]
// >;
// export type MeshStandardMaterialProps = MaterialNode<
//   THREE.MeshStandardMaterial,
//   [THREE.MeshStandardMaterialParameters]
// >;
// export type MeshPhongMaterialProps = MaterialNode<
//   THREE.MeshPhongMaterial,
//   [THREE.MeshPhongMaterialParameters]
// >;
// export type MeshToonMaterialProps = MaterialNode<
//   THREE.MeshToonMaterial,
//   [THREE.MeshToonMaterialParameters]
// >;
// export type MeshNormalMaterialProps = MaterialNode<
//   THREE.MeshNormalMaterial,
//   [THREE.MeshNormalMaterialParameters]
// >;
// export type MeshLambertMaterialProps = MaterialNode<
//   THREE.MeshLambertMaterial,
//   [THREE.MeshLambertMaterialParameters]
// >;
// export type MeshDepthMaterialProps = MaterialNode<
//   THREE.MeshDepthMaterial,
//   [THREE.MeshDepthMaterialParameters]
// >;
// export type MeshDistanceMaterialProps = MaterialNode<
//   THREE.MeshDistanceMaterial,
//   [THREE.MeshDistanceMaterialParameters]
// >;
// export type MeshBasicMaterialProps = MaterialNode<
//   THREE.MeshBasicMaterial,
//   [THREE.MeshBasicMaterialParameters]
// >;
// export type MeshMatcapMaterialProps = MaterialNode<
//   THREE.MeshMatcapMaterial,
//   [THREE.MeshMatcapMaterialParameters]
// >;
// export type LineDashedMaterialProps = MaterialNode<
//   THREE.LineDashedMaterial,
//   [THREE.LineDashedMaterialParameters]
// >;
// export type LineBasicMaterialProps = MaterialNode<
//   THREE.LineBasicMaterial,
//   [THREE.LineBasicMaterialParameters]
// >;

export type PrimitiveProps = { object: any } & { [properties: string]: any };

// export type LightProps = LightNode<THREE.Light, typeof THREE.Light>;
// export type SpotLightShadowProps = Node<
//   THREE.SpotLightShadow,
//   typeof THREE.SpotLightShadow
// >;
// export type SpotLightProps = LightNode<THREE.SpotLight, typeof THREE.SpotLight>;
// export type PointLightProps = LightNode<
//   THREE.PointLight,
//   typeof THREE.PointLight
// >;
// export type RectAreaLightProps = LightNode<
//   THREE.RectAreaLight,
//   typeof THREE.RectAreaLight
// >;
// export type HemisphereLightProps = LightNode<
//   THREE.HemisphereLight,
//   typeof THREE.HemisphereLight
// >;
// export type DirectionalLightShadowProps = Node<
//   THREE.DirectionalLightShadow,
//   typeof THREE.DirectionalLightShadow
// >;
// export type DirectionalLightProps = LightNode<
//   THREE.DirectionalLight,
//   typeof THREE.DirectionalLight
// >;
// export type AmbientLightProps = LightNode<
//   THREE.AmbientLight,
//   typeof THREE.AmbientLight
// >;
// export type LightShadowProps = Node<
//   THREE.LightShadow,
//   typeof THREE.LightShadow
// >;
// export type AmbientLightProbeProps = LightNode<
//   THREE.AmbientLightProbe,
//   typeof THREE.AmbientLightProbe
// >;
// export type HemisphereLightProbeProps = LightNode<
//   THREE.HemisphereLightProbe,
//   typeof THREE.HemisphereLightProbe
// >;
// export type LightProbeProps = LightNode<
//   THREE.LightProbe,
//   typeof THREE.LightProbe
// >;
//
// export type SpotLightHelperProps = ContainerNode<
//   THREE.SpotLightHelper,
//   typeof THREE.SpotLightHelper
// >;
// export type SkeletonHelperProps = ContainerNode<
//   THREE.SkeletonHelper,
//   typeof THREE.SkeletonHelper
// >;
// export type PointLightHelperProps = ContainerNode<
//   THREE.PointLightHelper,
//   typeof THREE.PointLightHelper
// >;
// export type HemisphereLightHelperProps = ContainerNode<
//   THREE.HemisphereLightHelper,
//   typeof THREE.HemisphereLightHelper
// >;
// export type GridHelperProps = ContainerNode<
//   THREE.GridHelper,
//   typeof THREE.GridHelper
// >;
// export type PolarGridHelperProps = ContainerNode<
//   THREE.PolarGridHelper,
//   typeof THREE.PolarGridHelper
// >;
// export type DirectionalLightHelperProps = ContainerNode<
//   THREE.DirectionalLightHelper,
//   typeof THREE.DirectionalLightHelper
// >;
// export type CameraHelperProps = ContainerNode<
//   THREE.CameraHelper,
//   typeof THREE.CameraHelper
// >;
// export type BoxHelperProps = ContainerNode<
//   THREE.BoxHelper,
//   typeof THREE.BoxHelper
// >;
// export type Box3HelperProps = ContainerNode<
//   THREE.Box3Helper,
//   typeof THREE.Box3Helper
// >;
// export type PlaneHelperProps = ContainerNode<
//   THREE.PlaneHelper,
//   typeof THREE.PlaneHelper
// >;
// export type ArrowHelperProps = ContainerNode<
//   THREE.ArrowHelper,
//   typeof THREE.ArrowHelper
// >;
// export type AxesHelperProps = ContainerNode<
//   THREE.AxesHelper,
//   typeof THREE.AxesHelper
// >;
//
export type TextureProps = Node<Texture, typeof Texture>;
// export type VideoTextureProps = Node<
//   THREE.VideoTexture,
//   typeof THREE.VideoTexture
// >;
// export type DataTextureProps = Node<
//   THREE.DataTexture,
//   typeof THREE.DataTexture
// >;
// export type DataTexture3DProps = Node<
//   THREE.DataTexture3D,
//   typeof THREE.DataTexture3D
// >;
// export type CompressedTextureProps = Node<
//   THREE.CompressedTexture,
//   typeof THREE.CompressedTexture
// >;
// export type CubeTextureProps = Node<
//   THREE.CubeTexture,
//   typeof THREE.CubeTexture
// >;
// export type CanvasTextureProps = Node<
//   THREE.CanvasTexture,
//   typeof THREE.CanvasTexture
// >;
// export type DepthTextureProps = Node<
//   THREE.DepthTexture,
//   typeof THREE.DepthTexture
// >;

export type PointProps = Node<PixiPoint, typeof PixiPoint>;
// export type Vector3Props = Node<THREE.Vector3, typeof THREE.Vector3>;
// export type Vector4Props = Node<THREE.Vector4, typeof THREE.Vector4>;
// export type EulerProps = Node<THREE.Euler, typeof THREE.Euler>;
// export type Matrix3Props = Node<THREE.Matrix3, typeof THREE.Matrix3>;
// export type Matrix4Props = Node<THREE.Matrix4, typeof THREE.Matrix4>;
// export type QuaternionProps = Node<THREE.Quaternion, typeof THREE.Quaternion>;
export type AttributeProps = Node<
  Attribute,
  typeof Attribute
>;
// export type Float32BufferAttributeProps = Node<
//   THREE.Float32BufferAttribute,
//   typeof THREE.Float32BufferAttribute
// >;
// export type InstancedBufferAttributeProps = Node<
//   THREE.InstancedBufferAttribute,
//   typeof THREE.InstancedBufferAttribute
// >;
export type ColorProps = Node<PixiColor, ColorSource>;
// export type FogProps = Node<THREE.Fog, typeof THREE.Fog>;
// export type FogExp2Props = Node<THREE.FogExp2, typeof THREE.FogExp2>;
// export type ShapeProps = Node<THREE.Shape, typeof THREE.Shape>;

declare module "solid-js" {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace JSX {
    interface IntrinsicElements {
      // `audio` works but conflicts with @types/react. Try using Audio from react-three-fiber/components instead
      container: ContainerProps;
      sprite: SpriteProps;
      // Mesh
      mesh: MeshProps;
      meshGeometry: MeshGeometryProps;
      meshMaterial: MeshMaterialProps;
      planeGeometry: PlaneGeometryProps;
      shader: ShaderProps;
      
      attribute: AttributeProps;

      // primitive
      primitive: PrimitiveProps;

      // textures
      texture: TextureProps;

      // misc
      point: PointProps;
      color: ColorProps;
    }
  }
}
