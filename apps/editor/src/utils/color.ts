export type RgbColor = {
    r: number;
    g: number;
    b: number;
};

export type HslColor = {
    h: number;
    s: number;
    l: number;
};
export type HsvColor = {
    h: number;
    s: number;
    v: number;
};

const round = (
    number: number,
    digits = 0,
    base = Math.pow(10, digits),
): number => {
    return Math.round(base * number) / base;
};

const format = (number: number) => {
    const hex = number.toString(16);
    return hex.length < 2 ? '0' + hex : hex;
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
        s: round(
            hh > 0 && hh < 200
                ? ((s * v) / 100 / (hh <= 100 ? hh : 200 - hh)) * 100
                : 0,
        ),
        l: round(hh / 2),
    };
};

export const hslFromRgb = ({ r, g, b }: RgbColor): HslColor => {
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
};

export const hslToCssString = ({ h, s, l }: HslColor): string =>
    `hsl(${h}, ${s}%, ${l}%)`;
export const hsvToCssString = (color: HsvColor): string => {
    const { h, s, l } = hslFromHsv(color);
    return `hsl(${h}, ${s}, ${l})`;
};

export const rgbFromHsv = ({ h, s, v }: HsvColor): RgbColor => {
    h = (h / 360) * 6;
    s = s / 100;
    v = v / 100;

    const hh = Math.floor(h),
        b = v * (1 - s),
        c = v * (1 - (h - hh) * s),
        d = v * (1 - (1 - h + hh) * s),
        module = hh % 6;

    return {
        r: round([v, c, b, b, d, v][module]! * 255),
        g: round([d, v, v, c, b, b][module]! * 255),
        b: round([b, b, d, v, v, c][module]! * 255),
    };
};

export const hexFromRgb = ({ r, g, b }: RgbColor): string => {
    return '#' + format(r) + format(g) + format(b);
};

export const hexFromHsv = (color: HsvColor) => hexFromRgb(rgbFromHsv(color));

export const hsvFromRgb = ({ r, g, b }: RgbColor): HsvColor => {
    const max = Math.max(r, g, b);
    const delta = max - Math.min(r, g, b);

    // prettier-ignore
    const hh = delta
    ? max === r
      ? (g - b) / delta
      : max === g
        ? 2 + (b - r) / delta
        : 4 + (r - g) / delta
    : 0;

    return {
        h: round(60 * (hh < 0 ? hh + 6 : hh)),
        s: round(max ? (delta / max) * 100 : 0),
        v: round((max / 255) * 100),
    };
};
