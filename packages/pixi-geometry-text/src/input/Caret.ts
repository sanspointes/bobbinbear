import { ColorSource, Point, Texture } from '@pixi/core';
import { Graphics } from '@pixi/graphics';
import { Mesh, MeshGeometry, MeshMaterial } from '@pixi/mesh';

type CaretOptions = {
    color: ColorSource;
    alpha: number;
    rangeColor: ColorSource;
    rangeAlpha: number;
};
export class CaretView extends Graphics {
    constructor(private options: CaretOptions) {
        super();
    }

    isSelectionRange = false;
    updatePosition(origin: Point, height: number, width = 40) {
        this.isSelectionRange = width !== undefined;

        this.position.copyFrom(origin);

        console.log(origin, height, width);

        this.clear();

        this.beginFill(this.options.color, this.options.alpha);

        this.drawRect(0, origin.y, width, height);
    }
}
