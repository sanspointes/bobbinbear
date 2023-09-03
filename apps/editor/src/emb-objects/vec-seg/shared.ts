import { Uuid } from "../../utils/uuid";
import { VectorNode } from "../node";
import { EmbBase, EmbHasLine, EmbState } from "../shared";
import { EmbVector } from "../vector";

export enum VectorSegmentType {
    MoveTo,
    LineTo,
    QuadraticTo,
    BezierTo,
}

export type MoveToVectorSegment = {
    id: Uuid<EmbVecSeg>;
    type: VectorSegmentType.MoveTo;
    from: VectorNode;
    to: VectorNode;
};
export type LineToVectorSegment = {
    id: Uuid<EmbVecSeg>;
    type: VectorSegmentType.LineTo;
    from: VectorNode;
    to: VectorNode;
};
export type QuadraticToVectorSegment = {
    id: Uuid<EmbVecSeg>;
    type: VectorSegmentType.QuadraticTo;
    from: VectorNode;
    c0: VectorNode;
    to: VectorNode;
};
export type BezierToVectorSegment = {
    id: Uuid<EmbVecSeg>;
    type: VectorSegmentType.BezierTo;
    from: VectorNode;
    c0: VectorNode;
    c1: VectorNode;
    to: VectorNode;
};

export function isMoveVecSeg(vecseg: VectorSegment): vecseg is MoveToVectorSegment {
    return (vecseg.type === VectorSegmentType.MoveTo);
}
export function isLineVecSeg(vecseg: VectorSegment): vecseg is LineToVectorSegment {
    return (vecseg.type === VectorSegmentType.LineTo);
}
export function isQuadraticVecSeg(vecseg: VectorSegment): vecseg is QuadraticToVectorSegment {
    return (vecseg.type === VectorSegmentType.QuadraticTo);
}
export function isBezierVecSeg(vecseg: VectorSegment): vecseg is BezierToVectorSegment {
    return (vecseg.type === VectorSegmentType.BezierTo);
}

/**
 * Checks if a vector segment contains a given node
 */
export function segmentHasNode(vecseg: VectorSegment, node: VectorNode): 'to' | 'c0' | 'c1' | false {
    const seg = vecseg as BezierToVectorSegment;
    if (seg.to.id === node.id ) return 'to';
    if (seg.c0.id === node.id ) return 'c0';
    if (seg.c1.id === node.id ) return 'c1';
    return false;
}

export type VectorSegment = {
    id: Uuid<EmbVecSeg>;
} & (MoveToVectorSegment | LineToVectorSegment | QuadraticToVectorSegment | BezierToVectorSegment);

export type EmbVecSeg = EmbBase & EmbHasLine & {
    id: Uuid<EmbVecSeg & EmbBase>;
    type: "vec-seg";
    segment: VectorSegment;
    relatesTo: Uuid<EmbVector & EmbState>;
};
