import { Uuid } from "../utils/uuid";
import { EmbBase, EmbObjectType } from "../emb-objects/shared";

declare module '@pixi/display' {
  export interface DisplayObject {
    id: Uuid<EmbBase>;
    soType: EmbObjectType,
  }
}
declare module '@pixi/mesh' {
  export interface Mesh {
    id: Uuid<EmbBase>;
    soType: EmbObjectType,
  }
}
declare module '@pixi/sprite' {
  export interface Sprite {
    id: Uuid<EmbBase>;
    soType: EmbObjectType,
  }
}
declare module '@pixi/graphics' {
  export interface Graphics {
    id: Uuid<EmbBase>;
    soType: EmbObjectType,
  }
}
