import { Uuid } from "../../utils/uuid";
import { EmbBase } from "../shared";
import { EmbVector } from "../vector";

export enum EmbNodeType {
  Jump = 0,
  Control = 1,
  Point = 2,
}

type NodeBase = {
  x: number;
  y: number;
};
export type NodePoint = NodeBase & {
  id: Uuid<EmbNode>;
  type: EmbNodeType.Point;
  ownsNext?: true;
  ownsPrev?: true;
  isControlPaired?: true;
};
export type NodePointVirtual = NodeBase & {
  id: Uuid<EmbNode>;
  virtual: true;
  type: EmbNodeType.Point;
  after: Uuid<VectorNode>;
  close: never;
};
export type NodeControl = NodeBase & {
  id: Uuid<EmbNode>;
  type: EmbNodeType.Control;
};
export type NodeJump = NodeBase & {
  id: Uuid<EmbNode>;
  type: EmbNodeType.Jump;
};

export type VectorNode = NodePoint | NodePointVirtual | NodeControl | NodeJump;
/**
 * NODE SCENE OBJECT
 */
export type EmbNode = EmbBase & {
  /** Internal States */
  /** Unique ID for each scene object */
  id: Uuid<EmbNode>;
  type: "node";
  node: VectorNode;
  /** The uuid this node object is bound to (i.e. makes up part of a GraphicSceneObject path) */
  relatesTo: Uuid<EmbVector>;

  data: number[];
};

export const isNodeJump = (node: VectorNode): node is NodeJump => {
  return node.type === EmbNodeType.Jump;
};
export const isNodePoint = (node: VectorNode): node is NodePoint => {
  return node.type === EmbNodeType.Point;
};
export const isNodePointVirtual = (
  node: VectorNode,
): node is NodePointVirtual => {
  return node.type === EmbNodeType.Point && (node as NodePointVirtual).virtual;
};
export const isNodeControl = (node: VectorNode): node is NodeControl => {
  return node.type === EmbNodeType.Control;
};

