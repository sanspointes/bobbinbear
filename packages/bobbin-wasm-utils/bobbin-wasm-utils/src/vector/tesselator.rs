use lyon::{
    lyon_tessellation::{
        geometry_builder::simple_builder, FillOptions, FillTessellator, VertexBuffers, TessellationError,
    },
    math::Point,
};

use super::{BBVector, BBVectorCommand, BBGeometryInternal};

pub fn tesselate_bb_vector_fill(vector: BBVector, options: &FillOptions) -> Result<BBGeometryInternal, TessellationError> {
    let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();
    let mut geometry_builder = simple_builder(&mut geometry);
    let mut tesselator = FillTessellator::new();
    let mut builder = tesselator.builder(options, &mut geometry_builder);

    let mut is_path_active = false;

    use BBVectorCommand::{C, L, M, Q, Z};
    for cmd in vector.commands {
        match cmd {
            M { x, y } => {
                if is_path_active {
                    builder.end(false);
                }
                builder.begin(Point::new(x, -y));
            }
            L { x, y } => {
                builder.line_to(Point::new(x, -y));
            }
            Q { c0x, c0y, x, y } => {
                builder.quadratic_bezier_to(Point::new(c0x, -c0y), Point::new(x, -y));
            }
            C {
                c0x,
                c0y,
                c1x,
                c1y,
                x,
                y,
            } => {
                builder.cubic_bezier_to(
                    Point::new(c0x, -c0y),
                    Point::new(c1x, -c1y),
                    Point::new(x, -y),
                );
            }
            Z => {
                builder.end(true);
                is_path_active = false;
            }
        }
    }


    builder.build()?;
    return Ok(geometry.into());
}
