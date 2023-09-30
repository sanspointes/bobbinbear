import { TyprGlyphShape } from './lib/Typr.U';
import { Typr, TyprFont } from './lib/Typr';
import { Result } from './lib/result';

import {
    BBFace,
    BBFillOptions,
    BBFillRule,
    BBGeometry,
} from '@bearbroidery/bobbin-wasm-utils';

export function parseOTFFont(buffer: ArrayBuffer): TyprFont | TyprFont[] {
    return Typr.parse(buffer);
}

/**
 * Errors
 */

class FetchLoadError extends Error {
    constructor(statuscode: number, message: string) {
        super(
            `Failed to fetch font.  Code: "${statuscode}", message: "${message}"`,
        );
    }
}
class NoFontLoadError extends Error {
    constructor(path: string) {
        super(`No font found in "${path}".`);
    }
}

type LoadFontResult = Promise<Result<BBFace, NoFontLoadError | FetchLoadError>>;

export class FontLoader {
    static cache: Record<string, BBFace | LoadFontResult> = {};

    private async loadFont(url: string) {
        const response = await fetch(url);
        if (!response.ok) {
            return Result.err(
                new FetchLoadError(response.status, await response.text()),
            );
        }

        const arrayBuffer = await response.arrayBuffer();

        const font = BBFace.from_buffer(new Uint8Array(arrayBuffer));
        if (Array.isArray(font)) {
            const first = font[0];
            if (first) {
                FontLoader.cache[url] = first;
                return Result.ok(first);
            } else {
                return Result.err(new NoFontLoadError(url));
            }
        } else {
            FontLoader.cache[url] = font;
            return Result.ok(font);
        }
    }

    getFont(url: string) {
        const cached = FontLoader.cache[url];
        if (cached) {
            if (cached instanceof Promise) return cached;
            return Promise.resolve(Result.ok(cached));
        }

        const promise: LoadFontResult = new Promise((res) => {
            this.loadFont(url).then((result) => {
                res(result);
            });
        });
        FontLoader.cache[url] = promise;
        return promise;
    }
}

/**
 * Wrapper around TyprU that provides the OTF font to the utility functions
 */
export class FontHandle {
    constructor(public font: BBFace) {}

    static async fromUrl(url: string) {
        const loadResult = await new FontLoader().getFont(url);
        if (!loadResult.ok) return loadResult;
        return Result.ok(new FontHandle(loadResult.value));
    }

    getStringShape(str: string, ltr = true): TyprGlyphShape[] {
        const result = new Array<TyprGlyphShape>(str.length);
        for (let i = 0; i < str.length; i++) {
            const c = str.charAt(i);
            const cshape = this.getCharShape(c, ltr);
            if (!cshape) {
                console.warn(
                    `FontHandle.getStringShape() - No shape for char "${c}".`,
                );
                continue;
            }
            result[i] = cshape;
        }
        return result;
    }
    getCharShape(char: string, ltr = true): TyprGlyphShape | undefined {
        const gid = this.font.gid_by_code_point(char);
        if (!gid) return undefined;
        const ax = this.font.x_advance_by_gid(gid) ?? 0;
        const ay = this.font.y_advance_by_gid(gid) ?? 0;
        const dx = this.font.x_side_bearing_by_gid(gid) ?? 0;
        const dy = this.font.y_side_bearing_by_gid(gid) ?? 0;

        return {
            g: gid,
            ax,
            ay,
            dx,
            dy,
        };
    }
    getGidGeometry(gid: number, ltr = true): BBGeometry {
        const options = new BBFillOptions();
        options.fill_rule = BBFillRule.EvenOdd;
        options.tolerance = 20;
        const r = this.font.gid_to_fill_geometry(gid, options);
        return r.geometry;
    }
    getCharGeometry(char: string, ltr = true): BBGeometry | undefined {
        const gid = this.font.gid_by_code_point(char);
        if (!gid) return undefined;
        return this.getGidGeometry(gid, ltr);
    }
    // getStringPathFromShape(shape: TyprGlyphShape[]): TyprPath {
    //     return TyprU.shapeToPath(this.font, shape) as TyprPath;
    // }
    // getGlyphMeta(gid: number): TyprGlyphMeta | null {
    //     return this.font.glyf[gid];
    // }
}
