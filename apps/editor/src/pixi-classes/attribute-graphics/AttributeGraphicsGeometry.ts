import { IShape, Matrix, WRAP_MODES } from '@pixi/core';
import { FillStyle, GraphicsGeometry, LineStyle } from '@pixi/graphics';
import { BatchPart, FILL_COMMANDS, BATCH_POOL } from '@pixi/graphics/lib/utils';
import { AttributeGraphicsData } from './AttributeGraphicsData';
import { AttributeGraphicsEntry } from './AttributeGraphics';

export class AttributeGraphicsGeometry extends GraphicsGeometry {
    /**
     * Draws the given shape to this Graphics object. Can be any of Circle, Rectangle, Ellipse, Line or Polygon.
     * @param {PIXI.Circle|PIXI.Ellipse|PIXI.Polygon|PIXI.Rectangle|PIXI.RoundedRectangle} shape - The shape object to draw.
     * @param fillStyle - Defines style of the fill.
     * @param lineStyle - Defines style of the lines.
     * @param matrix - Transform applied to the points of the shape.
     * @returns - Returns geometry for chaining.
     */
    public drawShape(
        shape: IShape,
        fillStyle: FillStyle | undefined = undefined,
        lineStyle: LineStyle | undefined = undefined,
        matrix: Matrix | undefined = undefined,
        attributeEntries: AttributeGraphicsEntry[] | undefined = undefined,
    ): AttributeGraphicsGeometry {
        const data = new AttributeGraphicsData(
            shape,
            fillStyle,
            lineStyle,
            matrix,
            attributeEntries,
        );

        this.graphicsData.push(data);
        this.dirty++;

        return this;
    }
    /**
     * Generates intermediate batch data. Either gets converted to drawCalls
     * or used to convert to batch objects directly by the Graphics object.
     */
    updateBatches(): void {
        if (!this.graphicsData.length) {
            this.batchable = true;

            return;
        }

        if (!this.validateBatching()) {
            return;
        }

        this.cacheDirty = this.dirty;

        const uvs = this.uvs;
        const graphicsData = this.graphicsData;

        let batchPart: BatchPart | null = null;

        let currentStyle = null;

        if (this.batches.length > 0) {
            batchPart = this.batches[this.batches.length - 1]!;
            currentStyle = batchPart.style!;
        }

        for (let i = this.shapeIndex; i < graphicsData.length; i++) {
            this.shapeIndex++;

            const data = graphicsData[i]!;
            const fillStyle = data.fillStyle;
            const lineStyle = data.lineStyle;
            const command = FILL_COMMANDS[data.type];

            // build out the shapes points..
            command.build(data);

            if (data.matrix) {
                this.transformPoints(data.points, data.matrix);
            }

            if (fillStyle.visible || lineStyle.visible) {
                this.processHoles(data.holes);
            }

            for (let j = 0; j < 2; j++) {
                const style = j === 0 ? fillStyle : lineStyle;

                if (!style.visible) continue;

                const nextTexture = style.texture.baseTexture;
                const index = this.indices.length;
                const attribIndex = this.points.length / 2;

                nextTexture.wrapMode = WRAP_MODES.REPEAT;

                if (j === 0) {
                    this.processFill(data);
                } else {
                    this.processLine(data);
                }

                const size = this.points.length / 2 - attribIndex;

                if (size === 0) continue;
                // close batch if style is different
                if (batchPart && !this._compareStyles(currentStyle!, style)) {
                    batchPart.end(index, attribIndex);
                    batchPart = null;
                }
                // spawn new batch if its first batch or previous was closed
                if (!batchPart) {
                    batchPart = BATCH_POOL.pop() || new BatchPart();
                    batchPart.begin(style, index, attribIndex);
                    this.batches.push(batchPart);
                    currentStyle = style;
                }

                this.addUvs(
                    this.points,
                    uvs,
                    style.texture,
                    attribIndex,
                    size,
                    style.matrix,
                );
            }
        }

        const index = this.indices.length;
        const attrib = this.points.length / 2;

        if (batchPart) {
            batchPart.end(index, attrib);
        }

        if (this.batches.length === 0) {
            // there are no visible styles in GraphicsData
            // its possible that someone wants Graphics just for the bounds
            this.batchable = true;

            return;
        }

        const need32 = attrib > 0xffff;

        // prevent allocation when length is same as buffer
        if (
            this.indicesUint16 &&
            this.indices.length === this.indicesUint16.length &&
            need32 === this.indicesUint16.BYTES_PER_ELEMENT > 2
        ) {
            this.indicesUint16.set(this.indices);
        } else {
            this.indicesUint16 = need32
                ? new Uint32Array(this.indices)
                : new Uint16Array(this.indices);
        }

        // TODO make this a const..
        this.batchable = this.isBatchable();

        if (this.batchable) {
            this.packBatches();
        } else {
            this.buildDrawCalls();
        }
    }

    /** Packs attributes to single buffer. */
    protected packAttributes(): void {
        const verts = this.points;
        const uvs = this.uvs;
        const colors = this.colors;
        const textureIds = this.textureIds;

        // verts are 2 positions.. so we * by 3 as there are 6 properties.. then 4 cos its bytes
        const glPoints = new ArrayBuffer(verts.length * 3 * 4);
        const f32 = new Float32Array(glPoints);
        const u32 = new Uint32Array(glPoints);

        let p = 0;

        for (let i = 0; i < verts.length / 2; i++) {
            f32[p++] = verts[i * 2];
            f32[p++] = verts[i * 2 + 1];

            f32[p++] = uvs[i * 2];
            f32[p++] = uvs[i * 2 + 1];

            u32[p++] = colors[i];

            f32[p++] = textureIds[i];
        }

        this._buffer.update(glPoints);
        this._indexBuffer.update(this.indicesUint16);
    }
}
