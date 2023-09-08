import { Graphics, ILineStyleOptions } from "@pixi/graphics";
import { IFillStyleOptions } from "@pixi/graphics";
import {
    SegmentUtils,
    VectorShape,
} from "../vec-seg";

export const drawVectorShapeToGraphic = (
    g: Graphics,
    shape: VectorShape,
    fill: IFillStyleOptions,
    line: ILineStyleOptions,
) => {
    g.clear();
    if (shape.length <= 1) return;

    g.beginFill(fill.color, fill.alpha);
    g.lineStyle(line);

    const start = shape.getStartNode();
    if (start) g.moveTo(start.x, start.y);
    console.log(start);

    for (let i = 0; i < shape.length; i++) {
        const seg = shape[i]!;
        const { to } = seg;
        if (SegmentUtils.isMove(seg)) {
            g.moveTo(to.x, to.y);
        } else if (SegmentUtils.isLine(seg)) {
            g.lineTo(to.x, to.y);
        } else if (SegmentUtils.isQuadratic(seg)) {
            const { c0 } = seg;
            g.quadraticCurveTo(c0.x, c0.y, to.x, to.y);
        } else if (SegmentUtils.isBezier(seg)) {
            const { c0, c1 } = seg;
            g.bezierCurveTo(c0.x, c0.y, c1.x, c1.y, to.x, to.y);
        }
    }

    g.endFill();
};
