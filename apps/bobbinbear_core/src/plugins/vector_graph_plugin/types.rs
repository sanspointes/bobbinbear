use bb_vector_network::bb_graph::BBGraph;
use bevy::prelude::*;

use lyon_tessellation::{
    FillOptions, FillVertexConstructor, FillVertex, StrokeOptions, StrokeVertexConstructor, StrokeVertex,
};

#[derive(Resource, Deref, DerefMut)]
pub struct FillTessellator(pub lyon_tessellation::FillTessellator);

#[derive(Resource, Deref, DerefMut)]
pub struct StrokeTessellator(pub lyon_tessellation::StrokeTessellator);

/// Defines the stroke options for the tessellator and color of the
/// generated vertices.
#[derive(Component, Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fill {
    pub options: FillOptions,
    pub color: Color,
}

impl Fill {
    pub fn color(color: Color) -> Self {
        Self {
            options: FillOptions::default(),
            color,
        }
    }
}

/// Defines the stroke options for the tessellator and color of the
/// generated vertices.
#[allow(missing_docs)]
#[derive(Component, Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Stroke {
    pub options: StrokeOptions,
    pub color: Color,
}

#[derive(Debug, Clone, Component, serde::Serialize, serde::Deserialize)]
/// Second type wrapping the BBGraph struct
pub struct VectorGraph(pub BBGraph);

impl Default for VectorGraph {
    fn default() -> Self {
        Self(BBGraph::new())
    }
}

/// The index type of a Bevy [`Mesh`](bevy::render::mesh::Mesh).
type IndexType = u32;
/// Lyon's [`VertexBuffers`] generic data type defined for [`Vertex`].
pub type VertexBuffers = lyon_tessellation::VertexBuffers<Vertex, IndexType>;

/// Zero-sized type used to implement various vertex construction traits from
/// Lyon.
pub struct VertexConstructor {
    pub color: Color,
}

// A vertex with all the necessary attributes to be inserted into a Bevy
/// [`Mesh`](bevy::render::mesh::Mesh).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
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
