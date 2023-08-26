
/**
 * Treating the range as a circle 0 === maxValue, gets the distance from value to other.
 */
export const circularDifference = (
  value: number,
  other: number,
  maxValue: number,
) => {
  const d0 = other - maxValue - value;
  const d1 = other - value;
  const d2 = other + maxValue - value;
  const ad0 = Math.abs(d0);
  const ad1 = Math.abs(d1);
  const ad2 = Math.abs(d2);
  console.log(ad0, ad1, ad2)
  if (ad0 < ad1) return d0;
  if (ad1 < ad2) return d1;
  else return d2;
};

/**
 * Get the t value (0-1) of `amount` between `a` and `b`
 */
export const unmapLinear = (amount: number, a: number, b: number) => {
  const dist = (b - a)
  return (amount - a) / dist;
}

/**
 * Maps a value `x` from [a1-a2] to [b1-b2]
 */
export const mapLinear = ( x: number, a1: number, a2: number, b1: number, b2: number ) => {
    return b1 + ( x - a1 ) * ( b2 - b1 ) / ( a2 - a1 );
}

export const clamp = (x: number, min: number, max: number) => {
  return Math.max(min, Math.min(x, max));
}

export const lerp = (a: number, b: number, t: number) => {
 return a * (1 - t) + b * t;
}

/**
 * Point / Vec2 Math
 *
 * MinimalPoint is the minimum type necessary for point maths.
 */
type MinimalPoint = {
  x: number;
  y: number;
}
export const lerpPoint = (a: MinimalPoint, b: MinimalPoint, t: number, out: MinimalPoint) => {
  out.x = a.x * (1 - t) + b.x * t;
  out.y = a.y * (1 - t) + b.y * t;
}
export const subPoint = (a: MinimalPoint, b: MinimalPoint, out: MinimalPoint) => {
  out.x = a.x - b.x;
  out.y = a.y - b.y;
}
export const mulPoint = (a: MinimalPoint, b: MinimalPoint, out: MinimalPoint) => {
  out.x = a.x * b.x;
  out.y = a.y * b.y;
}
export const divPoint = (a: MinimalPoint, b: MinimalPoint, out: MinimalPoint) => {
  out.x = a.x / b.x;
  out.y = a.y / b.y;
}
export const addPoint = (a: MinimalPoint, b: MinimalPoint, out: MinimalPoint) => {
  out.x = a.x + b.x;
  out.y = a.y + b.y;
}

