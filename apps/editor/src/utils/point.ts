import { Point } from '@pixi/core'

export const pointDistance = (a: Point, b: Point) => {
  const dx = b.x - a.x;
  const dy = b.y - a.y;

  return Math.sqrt((dx * dx) + (dy * dy));
}
