import { TyprGlyphShape } from './lib/Typr.U';
import { Typr, TyprFont } from './lib/Typr';
import { Result } from './lib/result';

import {
    BBFace,
    BBFillOptions,
    BBFillRule,
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

type CharGeometry = {
    vertices: Float32Array;
    indices: Uint16Array;
};

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
        const res = this.font.shape_text(str);
        const result = new Array<TyprGlyphShape>(str.length);

        for (let i = 0; i < str.length; i++) {
            const v = res.glyph_at(i);

            if (!v) throw new Error(`Missing glyph at ${i}`);

            result[i] = {
                g: v.gid,
                ax: v.x_advance,
                ay: v.y_advance,
                dx: v.x_offset,
                dy: v.y_offset,
            };
        }

        return result;
    }
    getGidGeometry(gid: number, ltr = true): CharGeometry {
        const options = new BBFillOptions();
        options.fill_rule = BBFillRule.EvenOdd;
        options.tolerance = 1;
        const r = this.font.gid_to_fill_geometry(gid, options);
        const result: CharGeometry = {
            vertices: new Float32Array(r.vertices.length),
            indices: new Uint16Array(r.indices.length),
        };
        result.vertices.set(r.vertices);
        result.indices.set(r.indices);
        return result;
    }
    getCharGeometry(char: string, ltr = true): CharGeometry | undefined {
        const gid = this.font.gid_by_code_point(char);
        if (!gid) return undefined;
        return this.getGidGeometry(gid, ltr);
    }
}
