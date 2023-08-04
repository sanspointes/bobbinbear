import { ColorSource, Point } from "pixi.js";
import { Uuid } from "../utils/uuid";

export type BaseSceneObject = {
  id: Uuid<SceneObject>,
  name: string,
  position: Point,
  parent?: Uuid<SceneObject>,
  locked: boolean,
  children: SceneObject[],
}
export type GraphicSceneObject = BaseSceneObject & {
  type: 'graphic',
}
export type CanvasSceneObject = BaseSceneObject & {
  type: 'canvas',
  size: Point,
  backgroundColor: ColorSource,
}

export type SceneObject = (GraphicSceneObject | CanvasSceneObject);
export type SceneObjectType = SceneObject['type'];

export type SceneObjectPropsLookup = {
  'canvas': CanvasSceneObject,
  'graphic': GraphicSceneObject,
}


