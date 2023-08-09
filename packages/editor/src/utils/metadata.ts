import { DisplayObject } from "@pixi/display"
import { SceneObject } from "../types/scene";

type SceneObjectMap = WeakMap<DisplayObject, {
  id: SceneObject['id'],
  type: SceneObject['type'],
}>

type MetadataMap = SceneObjectMap;

export const metadata: MetadataMap = new WeakMap();
