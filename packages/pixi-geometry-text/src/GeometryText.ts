import { FontHandle } from '.';
import { Mesh, MeshMaterial } from '@pixi/mesh';
import { Geometry, Rectangle, Texture } from '@pixi/core';
import { Container } from '@pixi/display';
import { type TyprGlyphShape } from './lib/Typr.U';

type FontCache = Record<string, Geometry>;

class CharGeometryCache {
    cache: WeakMap<FontHandle, FontCache> = new WeakMap();

    has(handle: FontHandle, char: string) {
        const handleCache = this.cache.get(handle);
        return handleCache ? handleCache[char] !== undefined : false;
    }

    set(handle: FontHandle, char: string, geometry: Geometry) {
        if (!this.cache.has(handle)) {
            this.cache.set(handle, {});
        }
        this.cache.get(handle)![char] = geometry;
    }

    get(handle: FontHandle, char: string) {
        const handleCache = this.cache.get(handle);
        return handleCache ? handleCache[char] : undefined;
    }
}

export class GeometryCharacter extends Mesh {
    constructor(
        public char: string,
        geometry: Geometry,
    ) {
        super(geometry, new MeshMaterial(Texture.WHITE));
        // this.scale.y = -1;
    }
    // debugGraphic?: Graphics;
    // showDebug(handle: FontHandle, gid: number) {
    //     const meta = handle.getGlyphMeta(gid);
    //     if (meta) {
    //         this.debugGraphic = this.debugGraphic ?? new Graphics();
    //
    //         this.debugGraphic.clear();
    //         this.debugGraphic.beginFill(0x00ff00, 0.1);
    //         this.debugGraphic.drawRect(
    //             meta.xMin,
    //             meta.yMin,
    //             meta.xMax,
    //             meta.yMax,
    //         );
    //         this.addChild(this.debugGraphic);
    //     }
    // }
}

export class GeometryText extends Container {
    static cache = new CharGeometryCache();
    public hitArea: Rectangle;

    constructor(
        public handle: FontHandle,
        private ltr = true,
    ) {
        super();
        this.hitArea = new Rectangle();
    }

    private charChildren: GeometryCharacter[] = [];

    private _value: string = '';
    get value(): string {
        return this._value;
    }
    set value(value: string) {
        if (this._value !== value) {
            this._value = value;

            if (value === '') {
                this.removeChildren();
                this.charChildren.splice(0);
                return;
            }

            const valueShape = this.handle.getStringShape(value);
            let offsetx = 0;
            let offsety = 0;

            if (value.length < this.charChildren.length) {
                this.removeChildren(value.length);
                this.charChildren.splice(value.length);
            }

            for (let i = 0; i < value.length; i++) {
                const char = value[i]!;

                let currentChild = this.charChildren[i];
                const shape = valueShape[i]!;
                if (!currentChild || currentChild.char !== char) {
                    const geometry = GeometryText.getCharGeometry(
                        GeometryText.cache,
                        this.handle,
                        shape,
                        char,
                    );

                    if (!geometry) {
                        console.warn(
                            `Could not get geometry for ${char}.  This should never happen.`,
                        );
                        continue;
                    }

                    if (currentChild) {
                        currentChild.char = char;
                        currentChild.geometry = geometry;
                    } else {
                        currentChild = new GeometryCharacter(char, geometry);
                        this.charChildren.push(currentChild);
                        this.addChild(currentChild);
                    }
                }
                currentChild.position.set(
                    offsetx + shape.dx,
                    offsety + shape.dy,
                );
                // currentChild.showDebug(this.handle, shape.g);
                offsetx += shape.ax;
                offsety += shape.ay;
            }

            if (this.interactive) {
                this.calculateBounds();
                this.hitArea = new Rectangle(0, 0, this.width, this.height);
            }
        }
    }

    public hitTestCharIndex(x: number, y: number) {
        const nearestResult = this.charChildren.reduce<{
            distx: number;
            i: number | undefined;
        }>(
            (acc, char, i) => {
                const distx = Math.abs(char.x - x);
                // const disty = Math.abs(tempRect.y - y);
                if (distx < acc.distx) return { distx, i };
                else return acc;
            },
            { distx: Number.MAX_VALUE, i: undefined },
        );

        return nearestResult.i;
    }

    public getCharAtIndex(index: number) {
        return this.charChildren[index];
    }

    /**
     * Gets geometry for a given character using the CharGeometryCache if possible.
     */
    private static getCharGeometry(
        cache: CharGeometryCache,
        handle: FontHandle,
        shape: TyprGlyphShape,
        char: string,
    ) {
        let geometry: Geometry | undefined;
        if (cache.has(handle, char)) {
            geometry = GeometryText.cache.get(handle, char);
        } else {
            const result = handle.getGidGeometry(shape.g);

            geometry = new Geometry();
            geometry.addAttribute('aVertexPosition', result.vertices);
            geometry.addAttribute('aTextureCoord', result.vertices);
            geometry.addIndex(result.indices);

            cache.set(handle, char, geometry);
        }
        return geometry;
    }
}
