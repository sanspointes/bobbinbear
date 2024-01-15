use bb_vector_network::prelude::*;
use comfy::*;
use lyon_tessellation::{FillVertexConstructor, FillVertex, VertexBuffers, BuffersBuilder, FillTessellator, FillOptions, StrokeVertexConstructor, StrokeVertex, StrokeTessellator, StrokeOptions};

struct ComfyFillVertexConstructor(Color);

impl FillVertexConstructor<SpriteVertex> for ComfyFillVertexConstructor {
    fn new_vertex(&mut self, mut vertex: FillVertex) -> SpriteVertex {
        let p = vertex.position();
        let c = self.0;
        SpriteVertex {
            position: [p.x, p.y, 0.],
            color: [c.r, c.g, c.b, c.a],
            tex_coords: [p.x, p.y],
        }
    }
}

pub fn tessellate_fill(graph: &BBGraph) -> BBResult<Mesh> {
    let path_result = graph.generate_fill_path();

    let Ok(path) = path_result else {
        let reason = path_result.unwrap_err();
        return Err(reason);
    };

    let mut buffers: VertexBuffers<SpriteVertex, u32> = VertexBuffers::new();
    // let mut vertex_builder = simple_builder(&mut buffers);
    let mut vertex_builder = BuffersBuilder::new(&mut buffers, ComfyFillVertexConstructor(DARK_GRAY));
    let mut tess = FillTessellator::new();


    let result = tess.tessellate(
            &path,          // PositionStore
            &FillOptions::default(),
            &mut vertex_builder
        );
    let m = Mesh {
        vertices: buffers.vertices.into(),
        indices: buffers.indices.into(),
        z_index: 0,
        texture: None,
    };

    Ok(m)
}

struct ComfyStrokeVertexConstructor(Color);

impl StrokeVertexConstructor<SpriteVertex> for ComfyStrokeVertexConstructor {
    fn new_vertex(&mut self, mut vertex: StrokeVertex) -> SpriteVertex {
        let p = vertex.position();
        let c = self.0;
        SpriteVertex {
            position: [p.x, p.y, 0.],
            color: [c.r, c.g, c.b, c.a],
            tex_coords: [p.x, p.y],
        }
    }
}

pub fn tessellate_stroke(graph: &BBGraph) -> BBResult<Mesh> {
    let path_result = graph.generate_stroke_path();

    let Ok(path) = path_result else {
        let reason = path_result.unwrap_err();
        return Err(reason);
    };

    let mut buffers: VertexBuffers<SpriteVertex, u32> = VertexBuffers::new();
    // let mut vertex_builder = simple_builder(&mut buffers);
    let mut vertex_builder = BuffersBuilder::new(&mut buffers, ComfyStrokeVertexConstructor(WHITE));
    let mut tess = StrokeTessellator::new();


    let result = tess.tessellate(
            &path,          // PositionStore
            &StrokeOptions::default().with_line_width(0.08),
            &mut vertex_builder
        );

    let m = Mesh {
        vertices: buffers.vertices.into(),
        indices: buffers.indices.into(),
        z_index: 0,
        texture: None,
    };

    Ok(m)
}
