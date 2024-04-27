use bevy::{
    ecs::system::{Command, EntityCommands, QueryLens},
    prelude::*,
};
use bevy_spts_uid::{Uid, UidRegistry};

use crate::prelude::{Edge, EdgeVariant, Endpoint};

#[derive(Debug)]
struct LinkEdgeCommand {
    pub edge: Uid,
    pub next_endpoint: Uid,
    pub prev_endpoint: Uid,
}

impl Command for LinkEdgeCommand {
    fn apply(self, world: &mut World) {
        let LinkEdgeCommand {
            edge,
            next_endpoint,
            prev_endpoint,
        } = self;
        //
        // if let Some(mut edge) = world.get_mut::<Edge>(edge) {
        //     edge.next_endpoint = next_endpoint;
        //     edge.prev_endpoint = prev_endpoint;
        // } else {
        //     warn!("LinkEdgeCommand: Attempted to get Edge component on enttiy {edge:?}, but none found.");
        // }
        let mut reg = world.resource_mut::<UidRegistry>();
        let next_endpoint_e = reg.entity(next_endpoint);
        let prev_endpoint_e = reg.entity(prev_endpoint);

        if let Some(mut endpoint) = world.get_mut::<Endpoint>(next_endpoint_e) {
            endpoint.prev_edge = Some(edge);
        } else {
            warn!("LinkEdgeCommand: Attempted to get Endpoint component on entity {next_endpoint:?}, but none found.");
            return;
        };
        if let Some(mut endpoint) = world.get_mut::<Endpoint>(prev_endpoint_e) {
            endpoint.next_edge = Some(edge);
        } else {
            warn!("LinkEdgeCommand: Attempted to get Endpoint component on entity {prev_endpoint:?}, but none found.");
        };
    }
}

pub trait VectorGraphicCommandsExt {
    fn link_edge(&mut self, edge: Uid, prev_endpoint: Uid, next_endpoint: Uid);
    fn spawn_edge(
        &mut self,
        edge_variant: EdgeVariant,
        prev_endpoint: Uid,
        next_endpoint: Uid,
    ) -> EntityCommands;
    fn despawn_edge(
        &mut self,
        reg: &mut UidRegistry,
        edge_entity: Uid,
        q_edge: QueryLens<&Edge>,
        q_endpoints: QueryLens<&mut Endpoint>,
    );
}

impl VectorGraphicCommandsExt for Commands<'_, '_> {
    fn link_edge(&mut self, edge: Uid, prev_endpoint: Uid, next_endpoint: Uid) {
        self.add(LinkEdgeCommand {
            edge,
            next_endpoint,
            prev_endpoint,
        })
    }

    fn spawn_edge(
        &mut self,
        edge_variant: EdgeVariant,
        prev_endpoint: Uid,
        next_endpoint: Uid,
    ) -> EntityCommands {
        let edge_uid = Uid::default();
        let edge = self
            .spawn((
                edge_uid,
                Edge {
                    next_endpoint,
                    prev_endpoint,
                },
                edge_variant,
            ))
            .id();

        self.add(LinkEdgeCommand {
            edge: edge_uid,
            prev_endpoint,
            next_endpoint,
        });

        self.entity(edge)
    }

    fn despawn_edge(
        &mut self,
        reg: &mut UidRegistry,
        edge_uid: Uid,
        mut q_edge: QueryLens<&Edge>,
        mut q_endpoints: QueryLens<&mut Endpoint>,
    ) {
        let edge_e = reg.entity(edge_uid);
        let edge = *q_edge.query().get(edge_e).unwrap_or_else(|reason| {
            panic!("Could not get edge to despawn {edge_uid:?}. Reason: {reason}")
        });

        let mut q_endpoints = q_endpoints.query();
        let mut endpoint = q_endpoints
            .get_mut(reg.entity(edge.next_endpoint))
            .unwrap_or_else(|reason| {
                panic!(
                    "Could not get endpoint of edge ({edge_uid:?}) to despawn {:?}. Reason: {reason}",
                    edge.next_endpoint
                )
            });
        endpoint.prev_edge = None;

        let mut endpoint = q_endpoints
            .get_mut(reg.entity(edge.prev_endpoint))
            .unwrap_or_else(|reason| {
                panic!(
                    "Could not get endpoint of edge ({edge_uid:?}) to despawn {:?}. Reason: {reason}",
                    edge.prev_endpoint
                )
            });
        endpoint.next_edge = None
    }
}

pub trait VectorGraphicWorldExt {
    fn link_edge(&mut self, edge: Uid, prev_endpoint: Uid, next_endpoint: Uid);
    fn spawn_edge(
        &mut self,
        edge_variant: EdgeVariant,
        prev_endpoint: Uid,
        next_endpoint: Uid,
    ) -> EntityWorldMut;
    fn despawn_edge(&mut self, reg: &mut UidRegistry, edge_entity: Uid);
}

impl VectorGraphicWorldExt for World {
    fn link_edge(&mut self, edge: Uid, prev_endpoint: Uid, next_endpoint: Uid) {
        let cmd = LinkEdgeCommand {
            edge,
            prev_endpoint,
            next_endpoint,
        };
        cmd.apply(self);
    }

    fn spawn_edge(
        &mut self,
        edge_variant: EdgeVariant,
        prev_endpoint: Uid,
        next_endpoint: Uid,
    ) -> EntityWorldMut<'_> {
        let edge_uid = Uid::default();
        let edge = self
            .spawn((
                edge_uid,
                Edge {
                    next_endpoint,
                    prev_endpoint,
                },
                edge_variant,
            ))
            .id();

        let cmd = LinkEdgeCommand {
            edge: edge_uid,
            prev_endpoint,
            next_endpoint,
        };
        cmd.apply(self);

        self.entity_mut(edge)
    }

    fn despawn_edge(&mut self, reg: &mut UidRegistry, edge_uid: Uid) {
        let edge = *self
            .get::<Edge>(reg.entity(edge_uid))
            .unwrap_or_else(|| panic!("Could not get edge to despawn {edge_uid:?}"));

        let mut endpoint = self
            .get_mut::<Endpoint>(reg.entity(edge.next_endpoint))
            .unwrap_or_else(|| {
                panic!(
                    "Could not get endpoint of edge ({edge_uid:?}) to despawn {:?}",
                    edge.next_endpoint
                )
            });
        endpoint.prev_edge = None;

        let mut endpoint = self
            .get_mut::<Endpoint>(reg.entity(edge.prev_endpoint))
            .unwrap_or_else(|| {
                panic!(
                    "Could not get endpoint of edge ({edge_uid:?}) to despawn {:?}",
                    edge.prev_endpoint
                )
            });
        endpoint.next_edge = None
    }
}


