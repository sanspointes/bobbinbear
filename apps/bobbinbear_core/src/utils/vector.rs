use bb_vector_network::bb_graph::BBGraph;
use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::plugins::vector_graph_plugin::VectorGraph;

use super::W;

#[derive(Bundle)]
pub struct BBObjectVectorBundle {
    pub vector_graph: VectorGraph,
    pub mesh: Mesh2dHandle,
    pub material: Handle<ColorMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
impl Default for BBObjectVectorBundle {
    fn default() -> Self {
        Self {
            vector_graph: VectorGraph::default(),
            mesh: Mesh2dHandle::default(),
            material: Handle::<ColorMaterial>::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
        }
    }
}
#[allow(dead_code)]
impl BBObjectVectorBundle {
    pub fn from_vector_graph(vector_graph: VectorGraph) -> Self {
        Self {
            vector_graph,
            ..Default::default()
        }
    }
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}
