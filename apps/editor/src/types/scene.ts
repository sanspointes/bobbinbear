import { Point } from "@pixi/core";
import { Uuid } from "../utils/uuid";
import { IFillStyleOptions, ILineStyleOptions } from "@pixi/graphics";
import { Command } from "../store/commands";

export type BaseSceneObject = {
  /** Internal States */
  /** Unique ID for each scene object */
  id: Uuid<BaseSceneObject>;
  /** Internal locking used for blocking the user from interacting with this element (but not children) */
  shallowLocked: boolean;
  /** Hover state */
  hovered: boolean;
  /** Selected state */
  selected: boolean;

  /** User controlled States */
  /** Whether the scene object is visible */
  visible: boolean;
  /** User-displaying name of object */
  name: string;
  /** X-Y position of object */
  position: Point;
  /** Optional parent, if no parent provided, it is at the top level. */
  parent: Uuid<BaseSceneObject>;
  /** User controls locking, disables interacitivity */
  locked: boolean;
  /** Children ids */
  children: Uuid<BaseSceneObject>[];
};

export type VirtualSceneObject = {
  virtual: true,
  virtualCreator: () => Command,
}

export type HasFillSceneObject = {
  fill: IFillStyleOptions;
};

export type HasStrokeSceneObject = {
  stroke: ILineStyleOptions;
};
export type HasInspectSceneObject = {
  inspecting: boolean;
  inspectingObject: Uuid<BaseSceneObject>;
};

/**
 * GRAPHICS SCENE OBJECT
 */
export enum GraphicNodeTypes {
  Jump = 0,
  Control = 1,
  Point = 2,
}

type BaseGraphicsNode = {
  id: Uuid<GraphicsNode>;
  x: number;
  y: number;
}
export type VirtualGraphicsNode = BaseGraphicsNode & {
  virtual: true,
  type: GraphicNodeTypes.Point;
  after: Uuid<GraphicsNode>;
  close: never;
}
export type BasicGraphicsNode = BaseGraphicsNode & {
  type: GraphicNodeTypes.Jump | GraphicNodeTypes.Control | GraphicNodeTypes.Point;
};

export type GraphicsNode = BasicGraphicsNode | VirtualGraphicsNode;

export type GraphicSceneObject =
  & BaseSceneObject
  & HasFillSceneObject
  & HasStrokeSceneObject
  & HasInspectSceneObject
  & {
    type: "graphic";
    shape: GraphicsNode[];
    close: boolean;
  };
/**
 * NODE SCENE OBJECT
 */
export type NodeSceneObject = BaseSceneObject & {
  type: "node";
  node: GraphicsNode;
  /** The uuid this node object is bound to (i.e. makes up part of a GraphicSceneObject path) */
  relatesTo: Uuid<GraphicSceneObject>;
};
/**
 * CANVAS SCENE OBJECT
 */
export type CanvasSceneObject = BaseSceneObject & HasFillSceneObject & {
  type: "canvas";
  size: Point;
};

/**
 * GROUP SCENE OBJECT
 */
export type GroupSceneObject = BaseSceneObject & {
  type: "group";
};

export type SceneObject =
  (GraphicSceneObject
  | CanvasSceneObject
  | NodeSceneObject
  | GroupSceneObject) & VirtualSceneObject;
export type SceneObjectType = SceneObject["type"];

export type SceneObjectPropsLookup = {
  "canvas": CanvasSceneObject;
  "graphic": GraphicSceneObject;
};
