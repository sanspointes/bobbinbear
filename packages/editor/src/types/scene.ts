import { ColorSource, Point } from "@pixi/core";
import { Uuid } from "../utils/uuid";
import { IFillStyleOptions, ILineStyleOptions, LINE_CAP } from "@pixi/graphics";

export type BaseSceneObject = {
  id: Uuid<SceneObject>,
  visible: boolean,
  name: string,
  position: Point,
  parent?: Uuid<SceneObject>,
  locked: boolean,
  children: SceneObject[],
  hovered: boolean,
  selected: boolean,
}

export enum GraphicNodeTypes {
  Jump = 0,
  Control = 1,
  Point = 2,
}

export type BasicGraphicsNode = {
  type: GraphicNodeTypes.Jump|GraphicNodeTypes.Control,
  x: number,
  y: number,
}
export type CloseableGraphicsNode = {
  type: GraphicNodeTypes.Point,
  x: number,
  y: number,
  close?: boolean,
}

export type GraphicsNode = BasicGraphicsNode | CloseableGraphicsNode;

export type GraphicSceneObject = BaseSceneObject & {
  type: 'graphic',
  shape: GraphicsNode[],
  fill: IFillStyleOptions,
  stroke: ILineStyleOptions,
}
export type CanvasSceneObject = BaseSceneObject & {
  type: 'canvas',
  size: Point,
  fill: IFillStyleOptions,
}

export type SceneObject = (GraphicSceneObject | CanvasSceneObject);
export type SceneObjectType = SceneObject['type'];

export type SceneObjectPropsLookup = {
  'canvas': CanvasSceneObject,
  'graphic': GraphicSceneObject,
}


