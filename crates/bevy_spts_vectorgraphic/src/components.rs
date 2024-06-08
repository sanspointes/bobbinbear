use bevy::utils::thiserror;
use bevy::{
    ecs::{query::QueryEntityError, reflect::ReflectComponent, system::QueryLens},
    prelude::*,
    utils::HashSet,
};
use bevy_spts_uid::{Uid, UidRegistry};
use lyon_tessellation::path::Path;

use crate::lyon_components::{FillOptions, StrokeOptions};

#[derive(thiserror::Error, Debug)]
#[error("The edge with uid: {uid} is not linked to this endpoint.")]
pub struct RemoveEdgeError {
    pub uid: Uid,
}

#[derive(thiserror::Error, Debug)]
#[error("Tried to link new edge {tried_to_link} but endpoint already linked to two edges ({prev_edge}, {next_edge})")]
pub struct LinkEdgeError {
    pub tried_to_link: Uid,
    pub prev_edge: Uid,
    pub next_edge: Uid,
}

#[derive(Component, Clone, Copy, Default, Debug)]
#[allow(dead_code)]
#[derive(Reflect)]
#[reflect(Component)]
pub struct Endpoint {
    /// Previous edge in loop
    pub(crate) next_edge: Option<Uid>,
    /// Next edge in loop
    pub(crate) prev_edge: Option<Uid>,
}
impl Endpoint {
    pub fn with_next_edge(mut self, next_edge: Uid) -> Self {
        self.next_edge = Some(next_edge);
        self
    }
    pub fn with_prev_edge(mut self, prev_edge: Uid) -> Self {
        self.prev_edge = Some(prev_edge);
        self
    }

    pub fn next_edge_entity(&self) -> Option<Uid> {
        self.next_edge
    }
    pub fn next_edge(
        &self,
        q_edges: &mut QueryLens<&Edge>,
        reg: &mut UidRegistry,
    ) -> Option<Result<Edge, QueryEntityError>> {
        self.next_edge.map(|uid| {
            let q_edges = q_edges.query();
            let entity = reg.entity(uid);
            q_edges.get(entity).copied()
        })
    }
    pub fn prev_edge_entity(&self) -> Option<Uid> {
        self.prev_edge
    }
    pub fn prev_edge(
        &self,
        q_edges: &mut QueryLens<&Edge>,
        reg: &mut UidRegistry,
    ) -> Option<Result<Edge, QueryEntityError>> {
        self.prev_edge.map(|uid| {
            let q_edges = q_edges.query();
            let entity = reg.entity(uid);
            q_edges.get(entity).copied()
        })
    }

    /// Returns the other edge when given the Uid of one edge.
    pub fn other_edge_uid(&self, edge_uid: &Uid) -> Option<&Uid> {
        if let Some(edge) = self.next_edge {
            if edge == *edge_uid {
                return self.prev_edge.as_ref();
            }
        }
        if let Some(edge) = self.prev_edge {
            if edge == *edge_uid {
                return self.next_edge.as_ref();
            }
        }
        None
    }
    pub fn other_edge(
        &self,
        q_edges: &mut QueryLens<&Edge>,
        reg: &mut UidRegistry,
        edge_uid: &Uid,
    ) -> Option<Result<Edge, QueryEntityError>> {
        let other_edge_uid = self.other_edge_uid(edge_uid)?;
        let q_edges = q_edges.query();
        let entity = reg.entity(*other_edge_uid);
        Some(q_edges.get(entity).copied())
    }

    pub fn can_link_edge(&mut self) -> bool {
        self.prev_edge.is_none() || self.next_edge.is_none()
    }

    pub fn link_edge(&mut self, edge_uid: &Uid) -> Result<(), LinkEdgeError> {
        if self.next_edge.is_none() {
            self.next_edge = Some(*edge_uid);
            Ok(())
        } else if self.prev_edge.is_none() {
            self.prev_edge = Some(*edge_uid);
            Ok(())
        } else {
            Err(LinkEdgeError {
                tried_to_link: *edge_uid,
                prev_edge: self.prev_edge.unwrap(),
                next_edge: self.next_edge.unwrap(),
            })
        }
    }

    pub fn unlink_edge(&mut self, edge_uid: &Uid) -> Result<(), RemoveEdgeError> {
        let mut did_remove = false;
        if let Some(edge) = self.next_edge {
            if edge == *edge_uid {
                self.next_edge = None;
                did_remove = true;
            }
        }
        if let Some(edge) = self.prev_edge {
            if edge == *edge_uid {
                self.prev_edge = None;
                did_remove = true;
            }
        }

        if did_remove {
            Ok(())
        } else {
            Err(RemoveEdgeError {
                uid: *edge_uid,
            })
        }
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
    pub uid: Uid,
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
pub struct Edge {
    /// Entity of start point
    pub(crate) next_endpoint: Uid,
    /// Entity of end point
    pub(crate) prev_endpoint: Uid,
}
impl Edge {
    pub fn next_endpoint_uid(&self) -> Uid {
        self.next_endpoint
    }

    pub fn prev_endpoint(
        &self,
        q_endpoints: &mut QueryLens<&Endpoint>,
        reg: &UidRegistry,
    ) -> Result<Endpoint, QueryEntityError> {
        let entity = reg.entity(self.prev_endpoint);
        q_endpoints.query().get(entity).copied()
    }

    pub fn prev_endpoint_uid(&self) -> Uid {
        self.prev_endpoint
    }

    pub fn next_endpoint(
        &self,
        q_endpoints: &mut QueryLens<&Endpoint>,
        reg: &UidRegistry,
    ) -> Result<Endpoint, QueryEntityError> {
        let entity = reg.entity(self.next_endpoint);
        q_endpoints.query().get(entity).copied()
    }

    pub fn other_endpoint_uid(&self, endpoint_uid: &Uid) -> Option<Uid> {
        if self.next_endpoint == *endpoint_uid {
            Some(self.prev_endpoint)
        } else if self.prev_endpoint == *endpoint_uid {
            Some(self.next_endpoint)
        } else {
            None
        }
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

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct VectorGraphic {
    pub endpoints: HashSet<Entity>,
    pub edges: HashSet<Entity>,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct VectorGraphicPathStorage {
    #[reflect(ignore)]
    path: Option<Path>,
}
impl Default for VectorGraphicPathStorage {
    fn default() -> Self {
        Self { path: None }
    }
}
impl VectorGraphicPathStorage {
    pub fn needs_recalculate(&self) -> bool {
        self.path.is_none()
    }
    pub fn set_path(&mut self, path: Path) {
        self.path = Some(path);
    }
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref()
    }
    pub fn set_dirty(&mut self) {
        self.path = None;
    }
}

#[derive(Bundle, Default)]
pub struct VectorGraphicBundle {
    pub vector_graphic: VectorGraphic,
    pub path_storage: VectorGraphicPathStorage,
    pub stroke_options: StrokeOptions,
    pub fill_options: FillOptions,
}
