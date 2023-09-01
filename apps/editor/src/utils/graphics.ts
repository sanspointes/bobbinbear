import { EmbNode, EmbNodeType, VectorNode } from "../emb-objects";
import { newUuid } from "./uuid";

export const createBoxGraphicsCommands = (
  width: number,
  height: number,
): VectorNode[] => {
  return [{
    id: newUuid<EmbNode>(),
    type: EmbNodeType.Jump,
    x: 0,
    y: 0,
  }, {
    id: newUuid<EmbNode>(),
    type: EmbNodeType.Point,
    x: width,
    y: 0,
  }, {
    id: newUuid<EmbNode>(),
    type: EmbNodeType.Point,
    x: width,
    y: height,
  }, {
    id: newUuid<EmbNode>(),
    type: EmbNodeType.Point,
    x: 0,
    y: height,
  }];
};
