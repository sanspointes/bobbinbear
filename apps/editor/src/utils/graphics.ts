import { NodeUtils, VectorNode, VectorNodeType } from "../emb-objects";
import {
    VectorShape,
} from "../emb-objects/vec-seg";
import { newUuid } from "./uuid";

export const createBoxGraphicsCommands = (
    width: number,
    height: number,
): VectorShape => {
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

    const shape = new VectorShape();
    shape.lineTo(topRight);
    shape.lineTo(bottomRight);
    shape.lineTo(bottomLeft);
    shape.lineTo(topleft);

    return shape;
};

const ELLIPSE_RATIO = 0.22;
export const createEllipseGraphicsCommands = (
    width: number,
    height: number,
): VectorShape => {
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

    const shape = new VectorShape();

    shape.setStart(t);
    shape.bezierTo(tr, rt, r);
    shape.bezierTo(rb, br, b);
    shape.bezierTo(bl, lb, l);
    shape.bezierTo(lt, tl, t);
    shape.close();

    return shape;
};
