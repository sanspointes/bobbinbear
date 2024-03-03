use bevy_ecs::{
    component::Component,
    entity::{Entity, EntityHashSet},
    query::QueryEntityError,
    system::QueryLens,
};
use bevy_math::Vec2;
use lyon_tessellation::path::{traits::PathBuilder, Path};
use smallvec::SmallVec;

#[derive(Component, Clone, Copy)]
#[allow(dead_code)]
pub struct Endpoint {
    /// Previous edge in loop
    next_edge: Option<Entity>,
    /// Next edge in loop
    prev_edge: Option<Entity>,
}
impl Endpoint {
    pub fn next_edge_entity(&self) -> Option<Entity> {
        self.next_edge
    }
    pub fn next_edge(
        &self,
        q_edges: &mut QueryLens<&Edge>,
    ) -> Option<Result<&Edge, QueryEntityError>> {
        self.next_edge.map(|entity| {
            let q_edges = q_edges.query();
            q_edges.get(entity)
        })
    }
    pub fn prev_edge_entity(&self) -> Option<Entity> {
        self.prev_edge
    }
    pub fn prev_edge(
        &self,
        q_edges: &mut QueryLens<&Edge>,
    ) -> Option<Result<&Edge, QueryEntityError>> {
        self.prev_edge.map(|entity| {
            let q_edges = q_edges.query();
            q_edges.get(entity)
        })
    }
}

#[derive(Component, Clone)]
#[allow(dead_code)]
pub struct EndpointPosition {
    position: Vec2,
    /// Any endpoints that this endpoint is "joined" with.
    joined_endpoints: SmallVec<[Entity; 4]>,
}

#[derive(Component, Clone, Copy)]
#[allow(dead_code)]
pub struct Edge {
    /// Entity of start point
    pub(crate) next_endpoint: Entity,
    /// Entity of end point
    pub(crate) prev_endpoint: Entity,
    /// Data relating to the curve between the two points
    pub variant: EdgeVariant,
}
impl Edge {
    pub fn next_endpoint_entity(&self) -> Entity {
        self.next_endpoint
    }

    pub fn prev_endpoint(
        &self,
        q_endpoints: &mut QueryLens<&Endpoint>,
    ) -> Result<&Endpoint, QueryEntityError> {
        let q_endpoints = q_endpoints.query();
        q_endpoints.get(self.prev_endpoint)
    }

    pub fn prev_endpoint_entity(&self) -> Entity {
        self.prev_endpoint
    }

    pub fn next_endpoint(
        &self,
        q_endpoints: &mut QueryLens<&Endpoint>,
    ) -> Result<&Endpoint, QueryEntityError> {
        let q_endpoints = q_endpoints.query();
        q_endpoints.get(self.next_endpoint)
    }
}

#[derive(Clone, Copy)]
pub enum EdgeVariant {
    Line,
    Quadratic { ctrl1: Vec2 },
    Cubic { ctrl1: Vec2, ctlr2: Vec2 },
}

#[derive(Component)]
pub struct VectorGraphic {
    pub endpoints: EntityHashSet,
    pub edges: EntityHashSet,
}

#[derive(Component)]
pub struct VectorGraphicPathStorage {
    paths: Vec<Path>,
}
