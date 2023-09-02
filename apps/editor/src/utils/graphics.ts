import { VectorNode, VectorNodeType } from "../emb-objects";
import { VectorSegment, VectorSegmentType } from "../emb-objects/vec-seg";
import { newUuid } from "./uuid";

export const createBoxGraphicsCommands = (
  width: number,
  height: number,
): VectorSegment[] => {
  const { Point } = VectorNodeType;
  const topleft: VectorNode = {
    id: newUuid(),
    type: Point,
    x: 0,
    y: 0,
  };
  const topRight: VectorNode = {
    id: newUuid(),
    type: Point,
    x: width,
    y: 0,
  };
  const bottomRight: VectorNode = {
    id: newUuid(),
    type: Point,
    x: width,
    y: height,
  };
  const bottomLeft: VectorNode = {
    id: newUuid(),
    type: Point,
    x: 0,
    y: height,
  };

  return [{
    id: newUuid(),
    type: VectorSegmentType.LineTo,
    from: topleft,
    to: topRight,
  }, {
    id: newUuid(),
    type: VectorSegmentType.LineTo,
    from: topRight,
    to: bottomRight,
  }, {
    id: newUuid(),
    type: VectorSegmentType.LineTo,
    from: bottomRight,
    to: bottomLeft,
  }, {
    id: newUuid(),
    type: VectorSegmentType.LineTo,
    from: bottomLeft,
    to: topleft,
  }];
};
