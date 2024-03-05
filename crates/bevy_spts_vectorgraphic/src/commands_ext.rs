use bevy::{ecs::system::{Command, EntityCommands, QueryLens}, prelude::*};

use crate::prelude::{Edge, EdgeVariant, Endpoint};

struct LinkEdgeCommand {
    pub edge: Entity,
    pub next_endpoint: Entity,
    pub prev_endpoint: Entity,
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

        if let Some(mut endpoint) = world.get_mut::<Endpoint>(next_endpoint) {
            endpoint.prev_edge = Some(edge);
        } else {
            warn!("LinkEdgeCommand: Attempted to get Endpoint component on entity {next_endpoint:?}, but none found.");
            return;
        };
        if let Some(mut endpoint) = world.get_mut::<Endpoint>(prev_endpoint) {
            endpoint.next_edge = Some(edge);
        } else {
            warn!("LinkEdgeCommand: Attempted to get Endpoint component on entity {prev_endpoint:?}, but none found.");
        };
    }
}

pub trait VectorGraphicCommandsExt {
    fn link_edge(&mut self, edge: Entity, prev_endpoint: Entity, next_endpoint: Entity);
    fn spawn_edge(
        &mut self,
        edge_variant: EdgeVariant,
        prev_endpoint: Entity,
        next_endpoint: Entity,
    ) -> EntityCommands;
    fn despawn_edge(
        &mut self,
        edge_entity: Entity,
        q_edge: QueryLens<&Edge>,
        q_endpoints: QueryLens<&mut Endpoint>,
    );
}

impl VectorGraphicCommandsExt for Commands<'_, '_> {
    fn link_edge(&mut self, edge: Entity, prev_endpoint: Entity, next_endpoint: Entity) {
        self.add(LinkEdgeCommand {
            edge,
            next_endpoint,
            prev_endpoint,
        })
    }

    fn spawn_edge(
        &mut self,
        edge_variant: EdgeVariant,
        prev_endpoint: Entity,
        next_endpoint: Entity,
    ) -> EntityCommands {
        let edge = self
            .spawn((
                Edge {
                    next_endpoint,
                    prev_endpoint,
                },
                edge_variant,
            ))
            .id();

        self.add(LinkEdgeCommand {
            edge,
            prev_endpoint,
            next_endpoint,
        });

        self.entity(edge)
    }

    fn despawn_edge(
        &mut self,
        edge_entity: Entity,
        mut q_edge: QueryLens<&Edge>,
        mut q_endpoints: QueryLens<&mut Endpoint>,
    ) {
        let edge = *q_edge
            .query()
            .get(edge_entity)
            .unwrap_or_else(|reason| panic!("Could not get edge to despawn {edge_entity:?}. Reason: {reason}"));

        let mut q_endpoints = q_endpoints.query();
        let mut endpoint = q_endpoints
            .get_mut(edge.next_endpoint)
            .unwrap_or_else(|reason| {
                panic!(
                    "Could not get endpoint of edge ({edge_entity:?}) to despawn {:?}. Reason: {reason}",
                    edge.next_endpoint
                )
            });
        endpoint.prev_edge = None;

        let mut endpoint = q_endpoints
            .get_mut(edge.prev_endpoint)
            .unwrap_or_else(|reason| {
                panic!(
                    "Could not get endpoint of edge ({edge_entity:?}) to despawn {:?}. Reason: {reason}",
                    edge.prev_endpoint
                )
            });
        endpoint.next_edge = None
    }
}

pub trait VectorGraphicWorldExt {
    fn link_edge(&mut self, edge: Entity, prev_endpoint: Entity, next_endpoint: Entity);
    fn spawn_edge(
        &mut self,
        edge_variant: EdgeVariant,
        prev_endpoint: Entity,
        next_endpoint: Entity,
    ) -> EntityWorldMut;
    fn despawn_edge(&mut self, edge_entity: Entity);
}

impl VectorGraphicWorldExt for World {
    fn link_edge(&mut self, edge: Entity, prev_endpoint: Entity, next_endpoint: Entity) {
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
        prev_endpoint: Entity,
        next_endpoint: Entity,
    ) -> EntityWorldMut<'_> {
        let edge = self
            .spawn((
                Edge {
                    next_endpoint,
                    prev_endpoint,
                },
                edge_variant,
            ))
            .id();

        let cmd = LinkEdgeCommand {
            edge,
            prev_endpoint,
            next_endpoint,
        };
        cmd.apply(self);

        self.entity_mut(edge)
    }

    fn despawn_edge(&mut self, edge_entity: Entity) {
        let edge = *self
            .get::<Edge>(edge_entity)
            .unwrap_or_else(|| panic!("Could not get edge to despawn {edge_entity:?}"));

        let mut endpoint = self
            .get_mut::<Endpoint>(edge.next_endpoint)
            .unwrap_or_else(|| {
                panic!(
                    "Could not get endpoint of edge ({edge_entity:?}) to despawn {:?}",
                    edge.next_endpoint
                )
            });
        endpoint.prev_edge = None;

        let mut endpoint = self
            .get_mut::<Endpoint>(edge.prev_endpoint)
            .unwrap_or_else(|| {
                panic!(
                    "Could not get endpoint of edge ({edge_entity:?}) to despawn {:?}",
                    edge.prev_endpoint
                )
            });
        endpoint.next_edge = None
    }
}
