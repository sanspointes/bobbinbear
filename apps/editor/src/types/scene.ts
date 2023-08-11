import { Point } from "@pixi/core";
import { Uuid } from "../utils/uuid";
import { IFillStyleOptions, ILineStyleOptions, LINE_CAP } from "@pixi/graphics";

export type BaseSceneObject = {
  /** Internal States */
  /** Unique ID for each scene object */
  id: Uuid<SceneObject>;
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
  parent: Uuid<SceneObject>;
  /** User controls locking, disables interacitivity */
  locked: boolean;
  /** Children (recursive) */
  children: SceneObject[];
}

/**
 * GRAPHICS SCENE OBJECT
 */
export enum GraphicNodeTypes {
  Jump = 0,
  Control = 1,
  Point = 2,
}

export type BasicGraphicsNode = {
  id: Uuid<GraphicsNode>
  type: GraphicNodeTypes.Jump|GraphicNodeTypes.Control;
  x: number;
  y: number;
}
export type CloseableGraphicsNode = {
  id: Uuid<GraphicsNode>
  type: GraphicNodeTypes.Point;
  x: number;
  y: number;
  close?: boolean;
}

export type GraphicsNode = BasicGraphicsNode | CloseableGraphicsNode;

export type GraphicSceneObject = BaseSceneObject & {
  type: 'graphic';
  shape: GraphicsNode[];
  fill: IFillStyleOptions;
  stroke: ILineStyleOptions;
  inspecting: boolean;
  inspectingRoot: Uuid<SceneObject> | undefined,
}
/**
 * NODE SCENE OBJECT
 */
export type NodeSceneObject = BaseSceneObject & {
  type: 'node';
  node: GraphicsNode;
  /** The uuid this node object is bound to (i.e. makes up part of a GraphicSceneObject path) */
  relatesTo: Uuid<SceneObject>,
}
/**
 * CANVAS SCENE OBJECT 
 */
export type CanvasSceneObject = BaseSceneObject & {
  type: 'canvas';
  size: Point;
  fill: IFillStyleOptions;
}

/**
 * GROUP SCENE OBJECT
 */
export type GroupSceneObject = BaseSceneObject & {
  type: 'group',
}

export type SceneObject = (GraphicSceneObject | CanvasSceneObject | NodeSceneObject | GroupSceneObject);
export type SceneObjectType = SceneObject['type'];

export type SceneObjectPropsLookup = {
  'canvas': CanvasSceneObject;
  'graphic': GraphicSceneObject;
}


