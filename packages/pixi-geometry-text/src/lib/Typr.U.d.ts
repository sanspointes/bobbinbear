import { TyprFont, TyprPath } from './Typr';

type TyprGlyphShape = {
    /** GID of char **/
    g: number;
    /** X axis kerning of char **/
    ax: number;
    /** Y Axis kerning of char **/
    ay: number;
    /** TODO **/
    dx: number;
    /** TODO **/
    dy: number;
};

export declare class TyprU {
    static shape(font: TyprFont, str: string, ltr: boolean): TyprGlyphShape[];
    static shapeToPath(font: TyprFont, shape: TyprGlyphShape[]): TyprPath;

    static stringToGlyphs(font: TyprFont, str: string): number[];
    static codeToGlyph(font: TyprFont, unicode: number): number;
    static glyphToPath(font: TyprFont): TyprPath;
    static glyphsToPath(font: TyprFont, gids: number[]): TyprPath;
    // static _tabOffset(data: any, tab: any): any;
}
