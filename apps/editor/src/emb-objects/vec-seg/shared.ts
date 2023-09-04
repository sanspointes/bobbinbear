import { arrayFirst, arrayLast } from "../../utils/array";
import { newUuid, Uuid } from "../../utils/uuid";
import { VectorNode } from "../node";
import { EmbBase, EmbHasLine, EmbState } from "../shared";
import { EmbVector } from "../vector";

export enum VectorSegmentType {
    MoveTo,
    LineTo,
    QuadraticTo,
    BezierTo,
}

type BaseVectorSegment = {
    id: Uuid<EmbVecSeg>;
    to: VectorNode;
    prev?: VectorSegment;
};
export type MoveToVectorSegment = BaseVectorSegment & {
    type: VectorSegmentType.MoveTo;
};
export type LineToVectorSegment = BaseVectorSegment & {
    type: VectorSegmentType.LineTo;
};
export type QuadraticToVectorSegment = BaseVectorSegment & {
    type: VectorSegmentType.QuadraticTo;
    c0: VectorNode;
};
export type BezierToVectorSegment = BaseVectorSegment & {
    type: VectorSegmentType.BezierTo;
    c0: VectorNode;
    c1: VectorNode;
};

/**
 * Checks if a vector segment contains a given node
 */
export type VectorSegment =
    | MoveToVectorSegment
    | LineToVectorSegment
    | QuadraticToVectorSegment
    | BezierToVectorSegment;

export type AnyVectorSegment =
    & BaseVectorSegment
    & { type: VectorSegment['type'] }
    & Partial<Omit<LineToVectorSegment, 'type'>>
    & Partial<Omit<MoveToVectorSegment, 'type'>>
    & Partial<Omit<QuadraticToVectorSegment, 'type'>>
    & Partial<Omit<BezierToVectorSegment, 'type'>>;

export type EmbVecSeg = EmbBase & EmbHasLine & {
    id: Uuid<EmbVecSeg & EmbBase>;
    type: "vec-seg";
    segment: VectorSegment;
    relatesTo: Uuid<EmbVector & EmbState>;
};

export class VectorSegmentArrayBuilder {
    private prev?: VectorSegment;
    private segments: VectorSegment[] = [];

    push(seg: VectorSegment) {
        this.segments.push(seg);
        this.prev = seg;
    }

    moveTo(to: VectorNode) {
        const seg: MoveToVectorSegment = {
            id: newUuid<EmbVecSeg>(),
            type: VectorSegmentType.MoveTo,
            to,
            prev: this.prev,
        };
        this.push(seg);
    }
    lineTo(to: VectorNode) {
        const seg: LineToVectorSegment = {
            id: newUuid<EmbVecSeg>(),
            type: VectorSegmentType.LineTo,
            to,
            prev: this.prev,
        };
        this.push(seg);
    }
    quadTo(c0: VectorNode, to: VectorNode) {
        const seg: QuadraticToVectorSegment = {
            id: newUuid<EmbVecSeg>(),
            type: VectorSegmentType.QuadraticTo,
            c0,
            to,
            prev: this.prev,
        };
        this.push(seg);
    }
    bezierTo(c0: VectorNode, c1: VectorNode, to: VectorNode) {
        const seg: BezierToVectorSegment = {
            id: newUuid<EmbVecSeg>(),
            type: VectorSegmentType.BezierTo,
            c0,
            c1,
            to,
            prev: this.prev,
        };
        this.push(seg);
    }

    build() {
        return this.segments;
    }

    buildAsClosed() {
        const first = arrayFirst(this.segments)!;
        const last = arrayLast(this.segments)!;
        first.prev = last;
        return this.build();
    }
}
