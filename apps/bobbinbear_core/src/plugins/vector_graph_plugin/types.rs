
#[derive(Resource, Deref, DerefMut)]
struct FillTessellator(lyon_tessellation::FillTessellator);

#[derive(Resource, Deref, DerefMut)]
struct StrokeTessellator(lyon_tessellation::StrokeTessellator);

/// Defines the stroke options for the tessellator and color of the
/// generated vertices.
#[derive(Component, Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fill {
    options: FillOptions,
    color: Color,
}


/// Defines the stroke options for the tessellator and color of the
/// generated vertices.
#[allow(missing_docs)]
#[derive(Component, Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Stroke {
    pub options: StrokeOptions,
    pub color: Color,
}

#[derive(Component, serde::Serialize, serde::Deserialize)]
/// Second type wrapping the BBGraph struct
pub struct VectorGraph(BBGraph);

/// Zero-sized type used to implement various vertex construction traits from
/// Lyon.
pub struct VertexConstructor {
    pub color: Color,
}

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y],
            color: self.color.as_linear_rgba_f32(),
        }
    }
}

/// Enables the construction of a [`Vertex`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y],
            color: self.color.as_linear_rgba_f32(),
        }
    }
}
