import { TyprGlyphShape, TyprU } from './lib/Typr.U';
import { Typr, TyprFont, TyprGlyphMeta, TyprPath } from './lib/Typr';
import { Result } from './lib/result';

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

type LoadFontResult = Promise<
    Result<TyprFont, NoFontLoadError | FetchLoadError>
>;

export class FontLoader {
    static cache: Record<string, TyprFont | LoadFontResult> = {};

    private async loadFont(url: string) {
        const response = await fetch(url);
        if (!response.ok) {
            return Result.err(
                new FetchLoadError(response.status, await response.text()),
            );
        }

        const arrayBuffer = await response.arrayBuffer();

        const font = parseOTFFont(arrayBuffer);
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
    constructor(private font: TyprFont) {}

    static async fromUrl(url: string) {
        const loadResult = await new FontLoader().getFont(url);
        if (!loadResult.ok) return loadResult;
        return Result.ok(new FontHandle(loadResult.value));
    }

    getStringShape(str: string, ltr = true): TyprGlyphShape[] {
        return TyprU.shape(this.font, str, ltr);
    }
    getStringPath(str: string, ltr = true): TyprPath {
        const shape = TyprU.shape(this.font, str, ltr);
        return TyprU.shapeToPath(this.font, shape) as TyprPath;
    }
    getStringPathFromShape(shape: TyprGlyphShape[]): TyprPath {
        return TyprU.shapeToPath(this.font, shape) as TyprPath;
    }
    getGlyphMeta(gid: number): TyprGlyphMeta | null {
        return this.font.glyf[gid];
    }
}
