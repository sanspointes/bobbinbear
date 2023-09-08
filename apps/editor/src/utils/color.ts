
export type RgbColor = {
    r: number;
    g: number;
    b: number;
}

export type HslColor = {
  h: number;
  s: number;
  l: number;
}
export type HsvColor = {
  h: number;
  s: number;
  v: number;
}

const round = (number: number, digits = 0, base = Math.pow(10, digits)): number => {
  return Math.round(base * number) / base;
};

export const hsvFromHsl = ({ h, s, l }: HslColor): HsvColor => {
  s *= (l < 50 ? l : 100 - l) / 100;

  return {
    h: h,
    s: s > 0 ? ((2 * s) / (l + s)) * 100 : 0,
    v: l + s,
  };
};

export const hslFromHsv = ({ h, s, v }: HsvColor): HslColor => {
  const hh = ((200 - s) * v) / 100;

  return {
    h: round(h),
    s: round(hh > 0 && hh < 200 ? ((s * v) / 100 / (hh <= 100 ? hh : 200 - hh)) * 100 : 0),
    l: round(hh / 2),
  };
};

export const hslFromRgb = ({r, g, b}: RgbColor): HslColor => {
  r /= 255;
  g /= 255;
  b /= 255;
  const l = Math.max(r, g, b);
  const s = l - Math.min(r, g, b);
  const h = s
    ? l === r
      ? (g - b) / s
      : l === g
      ? 2 + (b - r) / s
      : 4 + (r - g) / s
    : 0;
  return {
    h: 60 * h < 0 ? 60 * h + 360 : 60 * h,
    s: 100 * (s ? (l <= 0.5 ? s / (2 * l - s) : s / (2 - (2 * l - s))) : 0),
    l: (100 * (2 * l - s)) / 2,
};
}

export const hslToCssString = ({h, s, l}: HslColor): string => `hsl(${h}, ${s}%, ${l}%)`;
export const hsvToCssString = (color: HsvColor): string => {
    const { h, s, l} = hslFromHsv(color);
    return `hsl(${h}, ${s}, ${l})`
};
