import { Graphics } from '@pixi/graphics';
import { BezierUtils, QuadraticUtils } from '@pixi/graphics/lib/utils';

export type AttributeGraphicsEntryKey = `a${string}`;
export type AttributeGraphicsEntry = {
    [K: AttributeGraphicsEntryKey]:
        | number
        | [x: number, y: number]
        | [x: number, y: number, z: number];
};

/**
 * Graphics object that allows you to add arbitrary attribute data to each
 * polygon line.
 */
export class AttributeGraphics<
    TAttributes extends AttributeGraphicsEntry,
> extends Graphics {
    attributeData: TAttributes[] = [];
    currentAttributes?: TAttributes;
    setAttributes(attributes: TAttributes) {
        this.currentAttributes = attributes;
    }
    clearAttributes() {
        this.currentAttributes = undefined;
    }
    /**
     * Moves the current drawing position to x, y.
     * @param x - the X coordinate to move to
     * @param y - the Y coordinate to move to
     * @returns - This Graphics object. Good for chaining method calls
     */
    moveTo(x: number, y: number): this {
        this.startPoly();
        this.currentPath.points[0] = x;
        this.currentPath.points[1] = y;

        if (this.currentAttributes)
            this.attributeData.push(this.currentAttributes);

        return this;
    }

    /**
     * Draws a line using the current line style from the current drawing position to (x, y);
     * The current drawing position is then set to (x, y).
     * @param x - the X coordinate to draw to
     * @param y - the Y coordinate to draw to
     * @returns - This Graphics object. Good for chaining method calls
     */
    lineTo(x: number, y: number): this {
        if (!this.currentPath) {
            this.moveTo(0, 0);
        }

        // remove duplicates..
        const points = this.currentPath.points;
        const fromX = points[points.length - 2];
        const fromY = points[points.length - 1];

        if (fromX !== x || fromY !== y) {
            points.push(x, y);
        }

        if (this.currentAttributes)
            this.attributeData.push(this.currentAttributes);

        return this;
    }

    /**
     * Calculate the points for a quadratic bezier curve and then draws it.
     * Based on: https://stackoverflow.com/questions/785097/how-do-i-implement-a-bezier-curve-in-c
     * @param cpX - Control point x
     * @param cpY - Control point y
     * @param toX - Destination point x
     * @param toY - Destination point y
     * @returns - This Graphics object. Good for chaining method calls
     */
    public quadraticCurveTo(
        cpX: number,
        cpY: number,
        toX: number,
        toY: number,
    ): this {
        this._initCurve();

        const points = this.currentPath.points;

        if (points.length === 0) {
            this.moveTo(0, 0);
        }

        const preLength = points.length;
        QuadraticUtils.curveTo(cpX, cpY, toX, toY, points);
        const numberToAdd = points.length - preLength;
        if (this.currentAttributes) {
            for (let i = 0; i < numberToAdd; i += 2) {
                this.attributeData.push(this.currentAttributes);
            }
        }

        return this;
    }

    /**
     * Calculate the points for a bezier curve and then draws it.
     * @param cpX - Control point x
     * @param cpY - Control point y
     * @param cpX2 - Second Control point x
     * @param cpY2 - Second Control point y
     * @param toX - Destination point x
     * @param toY - Destination point y
     * @returns This Graphics object. Good for chaining method calls
     */
    public bezierCurveTo(
        cpX: number,
        cpY: number,
        cpX2: number,
        cpY2: number,
        toX: number,
        toY: number,
    ): this {
        this._initCurve();

        const { points } = this.currentPath;
        const preLength = points.length;

        BezierUtils.curveTo(
            cpX,
            cpY,
            cpX2,
            cpY2,
            toX,
            toY,
            this.currentPath.points,
        );

        const numberToAdd = points.length - preLength;
        if (this.currentAttributes) {
            for (let i = 0; i < numberToAdd; i += 2) {
                this.attributeData.push(this.currentAttributes);
            }
        }

        return this;
    }

    clear(): this {
        super.clear();
        this.attributeData.length = 0;
        this.currentAttributes = undefined;
        return this;
    }
}
