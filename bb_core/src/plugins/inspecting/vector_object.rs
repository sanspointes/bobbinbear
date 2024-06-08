use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_spts_uid::Uid;
use bevy_spts_vectorgraphic::components::{Edge, Endpoint};

use crate::{
    ecs::InternalObject,
    plugins::effect::Effect,
    views::{vector_edge::VectorEdgeVM, vector_endpoint::VectorEndpointVM},
};

pub fn handle_inspect_vector_object(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    inspected: Uid,
) {
    let parent_entity = inspected.entity(world).unwrap();
    warn!("handle_inspect_vector_object {parent_entity:?}-{inspected}");
    let mut changed = vec![inspected];

    // Create inspection views for vector endpoints
    let mut q_endpoints = world.query_filtered::<(Entity, &Parent, &Uid), With<Endpoint>>();
    let endpoint_entities: Vec<(Entity, Uid)> = q_endpoints
        .iter(world)
        .filter_map(|(e, parent, uid)| {
            if parent.get() == parent_entity {
                Some((e, *uid))
            } else {
                None
            }
        })
        .collect();

    for (entity, uid) in endpoint_entities {
        warn!("Inserting VectorEndpointVM on {entity:?}");
        world
            .entity_mut(entity)
            .insert(VectorEndpointVM)
            .remove::<InternalObject>();
        changed.push(uid);
    }

    // Create inspection views for vector edges
    let mut q_edges = world.query_filtered::<(Entity, &Parent, &Uid), With<Edge>>();
    let edge_entities: Vec<(Entity, Uid)> = q_edges
        .iter(world)
        .filter_map(|(e, parent, uid)| {
            if parent.get() == parent_entity {
                Some((e, *uid))
            } else {
                None
            }
        })
        .collect();

    for (entity, uid) in edge_entities {
        warn!("Inserting VectorEdgeVM on {entity:?}");
        world
            .entity_mut(entity)
            .insert(VectorEdgeVM)
            .remove::<InternalObject>();
        changed.push(uid);
    }

    respond.push_back(Effect::EntitiesChanged(changed));
}

pub fn handle_uninspect_vector_object(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    uninspected: Uid,
) {
    let parent_entity = uninspected.entity(world).unwrap();
    warn!("handle_uninspect_vector_object {parent_entity:?}-{uninspected}");
    let mut changed = vec![uninspected];

    // Create inspection views for vector endpoints
    let mut q_endpoints = world.query_filtered::<(Entity, &Parent, &Uid), With<Endpoint>>();
    let endpoint_entities: Vec<(Entity, Uid)> = q_endpoints
        .iter(world)
        .filter_map(|(e, parent, uid)| {
            if parent.get() == parent_entity {
                Some((e, *uid))
            } else {
                None
            }
        })
        .collect();

    for (entity, uid) in endpoint_entities {
        warn!("Removing VectorEndpointVM on {entity:?}");
        world
            .entity_mut(entity)
            .remove::<VectorEndpointVM>()
            .insert(InternalObject);
        changed.push(uid);
    }

    // Create inspection views for vector edges
    let mut q_edges = world.query_filtered::<(Entity, &Parent, &Uid), With<Edge>>();
    let edge_entities: Vec<(Entity, Uid)> = q_edges
        .iter(world)
        .filter_map(|(e, parent, uid)| {
            if parent.get() == parent_entity {
                Some((e, *uid))
            } else {
                None
            }
        })
        .collect();

    for (entity, uid) in edge_entities {
        warn!("Removing VectorEdgeVM on {entity:?}");
        world
            .entity_mut(entity)
            .remove::<VectorEdgeVM>()
            .insert(InternalObject);
        changed.push(uid);
    }

    respond.push_back(Effect::EntitiesChanged(changed));
}
