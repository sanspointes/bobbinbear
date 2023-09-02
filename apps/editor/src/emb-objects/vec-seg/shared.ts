import { Uuid } from "../../utils/uuid";
import { VectorNode } from "../node";
import { EmbBase, EmbHasStroke } from "../shared";

export enum VectorSegmentType {
  MoveTo,
  LineTo,
  QuadraticTo,
  BezierTo,
}

type MoveToVecSeg = {
  id: Uuid<EmbVecSeg>;
  type: VectorSegmentType.MoveTo;
  from: VectorNode;
  to: VectorNode;
};
type LineToVecSeg = {
  id: Uuid<EmbVecSeg>;
  type: VectorSegmentType.LineTo;
  from: VectorNode;
  to: VectorNode;
};
type QuadraticToVecSeg = {
  id: Uuid<EmbVecSeg>;
  type: VectorSegmentType.QuadraticTo;
  from: VectorNode;
  c0: VectorNode;
  to: VectorNode;
};
type BezierToVecSeg = {
  id: Uuid<EmbVecSeg>;
  type: VectorSegmentType.BezierTo;
  from: VectorNode;
  c0: VectorNode;
  c1: VectorNode;
  to: VectorNode;
};

export const isMoveVecSeg = (vecseg: VectorSegment): vecseg is MoveToVecSeg => {
  return (vecseg.type === VectorSegmentType.MoveTo);
}
export const isLineVecSeg = (vecseg: VectorSegment): vecseg is LineToVecSeg => {
  return (vecseg.type === VectorSegmentType.LineTo);
}
export const isQuadraticVecSeg = (vecseg: VectorSegment): vecseg is QuadraticToVecSeg => {
  return (vecseg.type === VectorSegmentType.QuadraticTo);
}
export const isBezierVecSeg = (vecseg: VectorSegment): vecseg is BezierToVecSeg => {
  return (vecseg.type === VectorSegmentType.BezierTo);
}

export type VectorSegment = {
  id: Uuid<EmbVecSeg>;
} & (MoveToVecSeg | LineToVecSeg | QuadraticToVecSeg | BezierToVecSeg);

export type EmbVecSeg = EmbBase & EmbHasStroke & {
  id: Uuid<EmbVecSeg>;
  type: 'vec-seg';
  segment: VectorSegment;
}
