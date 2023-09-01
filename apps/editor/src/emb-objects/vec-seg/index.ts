import { Uuid } from "../../utils/uuid";
import { VectorNode } from "../node";
import { EmbBase } from "../shared";

export enum EmbVecSegType {
  MoveTo,
  LineTo,
  QuadraticTo,
  BezierTo,
}

type MoveToVecSeg = {
  id: Uuid<EmbVecSeg>;
  type: EmbVecSegType.MoveTo;
  from: VectorNode;
  to: VectorNode;
};
type LineToVecSeg = {
  id: Uuid<EmbVecSeg>;
  type: EmbVecSegType.LineTo;
  from: VectorNode;
  to: VectorNode;
};
type QuadraticToVecSeg = {
  id: Uuid<EmbVecSeg>;
  type: EmbVecSegType.QuadraticTo;
  from: VectorNode;
  c0: VectorNode;
  to: VectorNode;
};
type BezierToVecSeg = {
  id: Uuid<EmbVecSeg>;
  type: EmbVecSegType.BezierTo;
  from: VectorNode;
  c0: VectorNode;
  c1: VectorNode;
  to: VectorNode;
};

export const isMoveVecSeg = (vecseg: EmbVecSeg): vecseg is MoveToVecSeg => {
  return (vecseg.type === EmbVecSegType.MoveTo);
}
export const isLineVecSeg = (vecseg: EmbVecSeg): vecseg is LineToVecSeg => {
  return (vecseg.type === EmbVecSegType.LineTo);
}
export const isQuadraticVecSeg = (vecseg: EmbVecSeg): vecseg is QuadraticToVecSeg => {
  return (vecseg.type === EmbVecSegType.QuadraticTo);
}
export const isBezierVecSeg = (vecseg: EmbVecSeg): vecseg is BezierToVecSeg => {
  return (vecseg.type === EmbVecSegType.BezierTo);
}

export type EmbVecSeg = EmbBase & {
  id: Uuid<EmbVecSeg>;
} & (MoveToVecSeg | LineToVecSeg | QuadraticToVecSeg | BezierToVecSeg);
