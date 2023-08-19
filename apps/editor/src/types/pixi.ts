import { Uuid } from "../utils/uuid";
import { BaseSceneObject, SceneObjectType } from "./scene";

declare module '@pixi/display' {
  export interface DisplayObject {
    id: Uuid<BaseSceneObject>;
    soType: SceneObjectType,
  }
}
declare module '@pixi/mesh' {
  export interface Mesh {
    id: Uuid<BaseSceneObject>;
    soType: SceneObjectType,
  }
}
declare module '@pixi/sprite' {
  export interface Sprite {
    id: Uuid<BaseSceneObject>;
    soType: SceneObjectType,
  }
}
declare module '@pixi/graphics' {
  export interface Graphics {
    id: Uuid<BaseSceneObject>;
    soType: SceneObjectType,
  }
}
