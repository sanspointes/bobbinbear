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
