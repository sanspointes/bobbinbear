use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::{Entity, EntityHashSet},
    query::QueryEntityError,
    system::QueryLens,
};
use bevy_math::Vec2;
use lyon_tessellation::path::Path;

#[derive(Component, Clone, Copy, Default)]
#[allow(dead_code)]
pub struct Endpoint {
    /// Previous edge in loop
    pub(crate) next_edge: Option<Entity>,
    /// Next edge in loop
    pub(crate) prev_edge: Option<Entity>,
}
impl Endpoint {
    pub fn next_edge_entity(&self) -> Option<Entity> {
        self.next_edge
    }
    pub fn next_edge(
        &self,
        q_edges: &mut QueryLens<&Edge>,
    ) -> Option<Result<Edge, QueryEntityError>> {
        self.next_edge.map(|entity| {
            let q_edges = q_edges.query();
            q_edges.get(entity).copied()
        })
    }
    pub fn prev_edge_entity(&self) -> Option<Entity> {
        self.prev_edge
    }
    pub fn prev_edge(
        &self,
        q_edges: &mut QueryLens<&Edge>,
    ) -> Option<Result<Edge, QueryEntityError>> {
        self.prev_edge.map(|entity| {
            let q_edges = q_edges.query();
            q_edges.get(entity).copied()
        })
    }
}
//
// #[derive(Component, Clone, Default)]
// #[allow(dead_code)]
// pub struct EndpointPosition {
//     position: Vec2,
//     /// Any endpoints that this endpoint is "joined" with.
//     joined_endpoints: SmallVec<[Entity; 4]>,
// }

#[derive(Bundle, Default)]
pub struct EndpointBundle {
    pub endpoint: Endpoint,
}

#[derive(Component, Clone, Copy)]
#[allow(dead_code)]
pub struct Edge {
    /// Entity of start point
    pub(crate) next_endpoint: Entity,
    /// Entity of end point
    pub(crate) prev_endpoint: Entity,
}
impl Edge {
    pub fn next_endpoint_entity(&self) -> Entity {
        self.next_endpoint
    }

    pub fn prev_endpoint(
        &self,
        q_endpoints: &mut QueryLens<&Endpoint>,
    ) -> Result<Endpoint, QueryEntityError> {
        q_endpoints.query().get(self.prev_endpoint).copied()
    }

    pub fn prev_endpoint_entity(&self) -> Entity {
        self.prev_endpoint
    }

    pub fn next_endpoint(
        &self,
        q_endpoints: &mut QueryLens<&Endpoint>,
    ) -> Result<Endpoint, QueryEntityError> {
        q_endpoints.query().get(self.next_endpoint).copied()
    }
}

#[derive(Component, Clone, Copy, Default)]
pub enum EdgeVariant {
    #[default]
    Line,
    Quadratic {
        ctrl1: Vec2,
    },
    Cubic {
        ctrl1: Vec2,
        ctrl2: Vec2,
    },
}

#[derive(Bundle)]
pub struct EdgeBundle {
    edge: Edge,
    edge_variant: EdgeVariant,
}

#[derive(Component, Default)]
pub struct VectorGraphic {
    pub endpoints: EntityHashSet,
    pub edges: EntityHashSet,
}

#[derive(Component, Default)]
pub enum VectorGraphicPathStorage {
    #[default]
    NeedsRecalculate,
    Calculated(Path),
}

#[derive(Bundle, Default)]
pub struct VectorGraphicBundle {
    pub vector_graphic: VectorGraphic,
    pub path_storage: VectorGraphicPathStorage,
}
