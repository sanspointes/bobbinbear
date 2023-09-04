import { NodeUtils, VectorNode, VectorNodeType } from "../emb-objects";
import {
    VectorSegment,
    VectorSegmentArrayBuilder,
} from "../emb-objects/vec-seg";
import { newUuid } from "./uuid";

export const createBoxGraphicsCommands = (
    width: number,
    height: number,
): VectorSegment[] => {
    const { Point } = VectorNodeType;
    const topleft: VectorNode = {
        id: newUuid(),
        type: Point,
        x: 0,
        y: 0,
    };
    const topRight: VectorNode = {
        id: newUuid(),
        type: Point,
        x: width,
        y: 0,
    };
    const bottomRight: VectorNode = {
        id: newUuid(),
        type: Point,
        x: width,
        y: height,
    };
    const bottomLeft: VectorNode = {
        id: newUuid(),
        type: Point,
        x: 0,
        y: height,
    };

    const builder = new VectorSegmentArrayBuilder();
    builder.lineTo(topRight);
    builder.lineTo(bottomRight);
    builder.lineTo(bottomLeft);
    builder.lineTo(topleft);

    return builder.buildAsClosed();
};

const ELLIPSE_RATIO = 0.22;
export const createEllipseGraphicsCommands = (
    width: number,
    height: number,
): VectorSegment[] => {
    const tl = NodeUtils.newControl(width * ELLIPSE_RATIO, 0);
    const t = NodeUtils.newPoint(width / 2, 0);
    const tr = NodeUtils.newControl(width - width * ELLIPSE_RATIO, 0);

    const rt = NodeUtils.newControl(width, height * ELLIPSE_RATIO);
    const r = NodeUtils.newPoint(width, height / 2);
    const rb = NodeUtils.newControl(width, height - height * ELLIPSE_RATIO);

    const bl = NodeUtils.newControl(width * ELLIPSE_RATIO, height);
    const b = NodeUtils.newPoint(width / 2, height);
    const br = NodeUtils.newControl(width - width * ELLIPSE_RATIO, height);

    const lt = NodeUtils.newControl(0, height * ELLIPSE_RATIO);
    const l = NodeUtils.newPoint(0, height / 2);
    const lb = NodeUtils.newControl(0, height - height * ELLIPSE_RATIO);

    const builder = new VectorSegmentArrayBuilder();

    builder.moveTo(t);
    builder.bezierTo(tr, rt, r);
    builder.bezierTo(rb, br, b);
    builder.bezierTo(bl, lb, l);
    builder.bezierTo(lt, tl, t);

    return builder.buildAsClosed();
};
