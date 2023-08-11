import { GraphicNodeTypes, GraphicsNode } from "../types/scene";
import { newUuid } from "./uuid";

export const createBoxGraphicsCommands = (
  width: number,
  height: number,
): GraphicsNode[] => {
  return [{
    id: newUuid<GraphicsNode>(),
    type: GraphicNodeTypes.Jump,
    x: 0,
    y: 0,
  }, {
    id: newUuid<GraphicsNode>(),
    type: GraphicNodeTypes.Point,
    x: width,
    y: 0,
  }, {
    id: newUuid<GraphicsNode>(),
    type: GraphicNodeTypes.Point,
    x: width,
    y: height,
  }, {
    id: newUuid<GraphicsNode>(),
    type: GraphicNodeTypes.Point,
    x: 0,
    y: height,
    close: true,
  }];
};
