import { Uuid } from '../utils/uuid';
import { EmbBase, EmbState } from '../emb-objects/shared';
import { EmbObjectType } from '../emb-objects';

declare module '@pixi/display' {
    export interface DisplayObject {
        id: Uuid;
        soType: EmbObjectType;
    }
}
declare module '@pixi/mesh' {
    export interface Mesh {
        id: Uuid;
        soType: EmbObjectType;
    }
}
declare module '@pixi/sprite' {
    export interface Sprite {
        id: Uuid;
        soType: EmbObjectType;
    }
}
declare module '@pixi/graphics' {
    export interface Graphics {
        id: Uuid;
        soType: EmbObjectType;
    }
}
