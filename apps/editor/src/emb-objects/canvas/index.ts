import { Point } from '@pixi/core';
import { Uuid } from "../../utils/uuid";
import { EmbBase, EmbHasFill } from "../shared";

export * from './EmbCanvas';

/**
 * CANVAS SCENE OBJECT
 */
export type EmbCanvas = EmbBase & EmbHasFill & {
  /** Internal States */
  /** Unique ID for each scene object */
  id: Uuid<EmbCanvas>;

  type: "canvas";
  size: Point;
};
