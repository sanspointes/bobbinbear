type TyprGlyphMeta = {
    noc: number;
    xMin: number;
    yMin: number;
    xMax: number;
    yMax: number;
    endPts: number[];
    instructions: number[];
    flags: number[];
    xs: number[];
    ys: number[];
};

type TyprHorizontalHeaderTable = {
    ascender: number;
    descender: number;
    lineGap: number;
    advanceWidthMax: number;
    minLeftSideBearing: number;
    minRightSideBearing: number;
    xMaxExtent: number;
    caretSlopeRise: number;
    caretSlopeRun: number;
    caretOffset: number;
    res0: number;
    res1: number;
    res2: number;
    res3: number;
    metricDataFormat: number;
    numberOfHMetrics: number;
};

type TyprFont = {
    CPOS: unknown;
    _data: unknown;
    cmap: unknown;
    head: unknown;
    hhea: TyprHorizontalHeaderTable;
    maxp: unknown;
    hmtx: unknown;
    name: unknown;
    'OS/2': unknown;
    post: unknown;
    loca: unknown;
    glyf: (TyprGlyphMeta | null)[];
    GPOS: unknown;
    GSUB: unknown;
};

type TyprPathCmd = 'M' | 'L' | 'Q' | 'C' | 'Z' | 'X';
type TyprPath = {
    cmds: TyprPathCmd[];
    crds: number[];
};

export declare class Typr {
    static parse(buff: BufferSource): TyprFont;
    // static _tabOffset(data: any, tab: any): any;
}
