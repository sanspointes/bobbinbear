import { EmbNodeType, VectorNode } from "../types/scene";
import { newUuid } from "./uuid";

export const createBoxGraphicsCommands = (
  width: number,
  height: number,
): VectorNode[] => {
  return [{
    id: newUuid<VectorNode>(),
    type: EmbNodeType.Jump,
    x: 0,
    y: 0,
  }, {
    id: newUuid<VectorNode>(),
    type: EmbNodeType.Point,
    x: width,
    y: 0,
  }, {
    id: newUuid<VectorNode>(),
    type: EmbNodeType.Point,
    x: width,
    y: height,
  }, {
    id: newUuid<VectorNode>(),
    type: EmbNodeType.Point,
    x: 0,
    y: height,
  }];
};
