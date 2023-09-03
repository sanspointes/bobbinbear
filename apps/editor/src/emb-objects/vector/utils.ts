import { Graphics, ILineStyleOptions } from "@pixi/graphics";
import { IFillStyleOptions } from "@pixi/graphics";
import {
    isBezierVecSeg,
    isLineVecSeg,
    isMoveVecSeg,
    isQuadraticVecSeg,
    VectorSegment,
} from "../vec-seg";
import { arrayFirst } from "../../utils/array";

export const updateGraphicsWithSegments = (
    g: Graphics,
    segments: VectorSegment[],
    fill: IFillStyleOptions,
    line: ILineStyleOptions,
) => {
    g.clear();
    if (segments.length <= 1) return;

    g.beginFill(fill.color, fill.alpha);
    g.lineStyle(line);

    const first = arrayFirst(segments)!;
    g.moveTo(first.from.x, first.from.y);

    for (let i = 0; i < segments.length; i++) {
        const seg = segments[i]!;
        const { to } = seg;
        if (isMoveVecSeg(seg)) {
            g.moveTo(to.x, to.y);
        } else if (isLineVecSeg(seg)) {
            g.lineTo(to.x, to.y);
        } else if (isQuadraticVecSeg(seg)) {
            const { c0 } = seg;
            g.quadraticCurveTo(c0.x, c0.y, to.x, to.y);
        } else if (isBezierVecSeg(seg)) {
            const { c0, c1 } = seg;
            g.bezierCurveTo(c0.x, c0.y, c1.x, c1.y, to.x, to.y);
        }
    }
    // if (extra.close) {
    //   const lastSeg = arrayLast(segments);
    //   if (stackIndex === 0) {
    //     g.lineTo(node.x, node.y);
    //   } else if (stackIndex === 1) {
    //     const c0 = stack[0]!;
    //     g.quadraticCurveTo(c0.x, c0.y, node.x, node.y);
    //   } else if (stackIndex === 2) {
    //     const c0 = stack[0]!;
    //     const c1 = stack[1]!;
    //     g.bezierCurveTo(c0.x, c0.y, c1.x, c1.y, node.x, node.y);
    //   }
    // }

    g.endFill();
};
