import { IShape, Matrix } from "@pixi/core";
import { FillStyle, GraphicsData, LineStyle } from "@pixi/graphics";
import { AttributeGraphicsEntry } from "./AttributeGraphics";

/**
 * Graphics data modified to also allow arbitrary attribute data for each vertex.
 */
export class AttributeGraphicsData extends GraphicsData {
  /**
   * @param {PIXI.Circle|PIXI.Ellipse|PIXI.Polygon|PIXI.Rectangle|PIXI.RoundedRectangle} shape - The shape object to draw.
   * @param fillStyle - the width of the line to draw
   * @param lineStyle - the color of the line to draw
   * @param matrix - Transform matrix
   */
  constructor(
    shape: IShape,
    fillStyle: FillStyle | undefined = undefined,
    lineStyle: LineStyle | undefined = undefined,
    matrix: Matrix | undefined = undefined,
    private attributeEntries: AttributeGraphicsEntry[] | undefined = undefined,
  ) {
    super(shape, fillStyle, lineStyle, matrix);
  }
  /**
   * Creates a new AttributeGraphicsData object with the same values as this one.
   * @returns - Cloned AttributeGraphicsData object
   */
  public clone(): AttributeGraphicsData {
    return new AttributeGraphicsData(
      this.shape,
      this.fillStyle,
      this.lineStyle,
      this.matrix,
      this.attributeEntries,
    );
  }

  public destroy(): void {
    super.destroy();

    if (this.attributeEntries) this.attributeEntries.length = 0;
  }
}
