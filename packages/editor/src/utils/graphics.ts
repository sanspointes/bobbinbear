import { GraphicNodeTypes, GraphicsNodes } from "../types/scene";

export const createBoxGraphicsCommands = (
  width: number,
  height: number,
): GraphicsNodes[] => {
  return [{
    type: GraphicNodeTypes.Jump,
    x: 0,
    y: 0,
  }, {
    type: GraphicNodeTypes.Point,
    x: width,
    y: 0,
  }, {
    type: GraphicNodeTypes.Point,
    x: width,
    y: height,
  }, {
    type: GraphicNodeTypes.Point,
    x: 0,
    y: height,
    close: true,
  }];
};
