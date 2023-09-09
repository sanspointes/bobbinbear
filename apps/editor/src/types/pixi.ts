import { Uuid } from '../utils/uuid';
import { EmbBase, EmbState } from '../emb-objects/shared';
import { EmbObjectType } from '../emb-objects';

declare module '@pixi/display' {
    export interface DisplayObject {
        id: Uuid<EmbBase & EmbState>;
        soType: EmbObjectType;
    }
}
declare module '@pixi/mesh' {
    export interface Mesh {
        id: Uuid<EmbBase & EmbState>;
        soType: EmbObjectType;
    }
}
declare module '@pixi/sprite' {
    export interface Sprite {
        id: Uuid<EmbBase & EmbState>;
        soType: EmbObjectType;
    }
}
declare module '@pixi/graphics' {
    export interface Graphics {
        id: Uuid<EmbBase & EmbState>;
        soType: EmbObjectType;
    }
}
