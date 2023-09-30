import { ColorSource, Point, Texture } from '@pixi/core';
import { Mesh, MeshGeometry, MeshMaterial } from '@pixi/mesh';

type CaretOptions = {
    color: ColorSource;
    alpha: number;
    rangeColor: ColorSource;
    rangeAlpha: number;
};
class CaretView extends Mesh {
    constructor(private options: CaretOptions) {
        const geometry = new MeshGeometry();
        super(geometry, new MeshMaterial(Texture.WHITE));

        this.material.tint = options.color;
        this.material.alpha = options.alpha;
    }

    isSelectionRange = false;
    updatePosition(origin: Point, height: number, width?: number) {
        this.isSelectionRange = width !== undefined;

        this.getBounds;
    }
}

class CaretManager extends Mesh {

}
