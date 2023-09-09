import { VectorNode } from '../node';
import {
    AnyVectorSegment,
    BezierToVectorSegment,
    LineToVectorSegment,
    MoveToVectorSegment,
    QuadraticToVectorSegment,
    VectorSegment,
    VectorSegmentType,
} from './shared';
import { Polygon } from '@pixi/core';

export function isMove(vecseg: VectorSegment): vecseg is MoveToVectorSegment {
    return vecseg.type === VectorSegmentType.MoveTo;
}

export function isLine(vecseg: VectorSegment): vecseg is LineToVectorSegment {
    return vecseg.type === VectorSegmentType.LineTo;
}
export function isQuadratic(
    vecseg: VectorSegment,
): vecseg is QuadraticToVectorSegment {
    return vecseg.type === VectorSegmentType.QuadraticTo;
}

export function isBezier(
    vecseg: VectorSegment,
): vecseg is BezierToVectorSegment {
    return vecseg.type === VectorSegmentType.BezierTo;
}

export function generateControlPolygon(
    vecseg: AnyVectorSegment,
): Polygon | undefined {
    const { c0, prev } = vecseg;
    if (c0 && prev) {
        const prevSegment = prev as unknown as AnyVectorSegment;
        const points = [c0, prevSegment.to];
        const prevNode = prevSegment.c1 ?? prevSegment.c0;
        if (prevNode) points.push(prevNode);
        const poly = new Polygon(points);
        poly.closeStroke = false;
        return poly;
    }
}

export function segmentHasNode(
    vecseg: VectorSegment,
    node: VectorNode,
): 'to' | 'c0' | 'c1' | false {
    const seg = vecseg as BezierToVectorSegment;
    if (seg.to.id === node.id) return 'to';
    if (seg.c0.id === node.id) return 'c0';
    if (seg.c1.id === node.id) return 'c1';
    return false;
}
