use bevy::{
    ecs::{
        entity::{EntityHashSet, MapEntities}, query::QueryEntityError, reflect::{ReflectComponent, ReflectMapEntities},
        system::QueryLens,
    },
    prelude::*,
};
use lyon_tessellation::path::Path;

use crate::lyon_components::{FillOptions, StrokeOptions};

#[derive(Component, Clone, Copy, Default, Debug)]
#[allow(dead_code)]
#[derive(Reflect)]
#[reflect(Component)]
#[reflect(MapEntities)]
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

impl MapEntities for Endpoint {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.next_edge = self.next_edge.map(|e| entity_mapper.map_entity(e));
        self.prev_edge = self.prev_edge.map(|e| entity_mapper.map_entity(e));
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
    pub transform: Transform,
}
impl EndpointBundle {
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.transform.translation = translation;
        self
    }
}

#[derive(Component, Clone, Copy, Debug)]
#[allow(dead_code)]
#[derive(Reflect)]
#[reflect(Component)]
#[reflect(MapEntities)]
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

impl MapEntities for Edge {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.next_endpoint = entity_mapper.map_entity(self.next_endpoint);
        self.prev_endpoint = entity_mapper.map_entity(self.prev_endpoint);
    }
}

#[derive(Component, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
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
    pub stroke_options: StrokeOptions,
    pub fill_options: FillOptions,
}
