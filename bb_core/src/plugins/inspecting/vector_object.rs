use std::collections::VecDeque;

use bevy::{ecs::system::SystemState, prelude::*};
use bevy_spts_uid::Uid;
use bevy_spts_vectorgraphic::components::Endpoint;

use crate::{
    ecs::InternalObject, plugins::effect::Effect, views::vector_endpoint::VectorEndpointVM,
};

pub fn handle_inspect_vector_object(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    inspected: Uid,
) {
    handle_inspect_vector_object_endpoints(respond, world, inspected);
}

pub fn handle_inspect_vector_object_endpoints(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    inspected: Uid,
) {
    let mut sys_state = SystemState::<Query<(Entity, &Parent, &Uid), With<Endpoint>>>::new(world);

    let parent_entity = inspected.entity(world).unwrap();
    let q_endpoints = sys_state.get_mut(world);
    let mut changed = vec![inspected];

    let endpoint_entities: Vec<(Entity, Uid)> = q_endpoints
        .iter()
        .filter_map(|(e, parent, uid)| {
            if parent.get() == parent_entity {
                Some((e, *uid))
            } else {
                None
            }
        })
        .collect();

    for (entity, uid) in endpoint_entities {
        world
            .entity_mut(entity)
            .insert(VectorEndpointVM)
            .remove::<InternalObject>();
        changed.push(uid);
    }

    respond.push_back(Effect::EntitiesChanged(changed));
}

pub fn handle_uninspect_vector_object(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    uid: Uid,
) {
    handle_uninspect_vector_object_endpoints(respond, world, uid);
}

#[allow(dead_code)]
pub fn handle_uninspect_vector_object_endpoints(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    uninspected: Uid,
) {
    let mut sys_state = SystemState::<Query<(Entity, &Parent, &Uid), With<Endpoint>>>::new(world);

    let parent_entity = uninspected.entity(world).unwrap();
    let q_endpoints = sys_state.get_mut(world);
    let mut changed = vec![uninspected];

    let endpoint_entities: Vec<(Entity, Uid)> = q_endpoints
        .iter()
        .filter_map(|(e, parent, uid)| {
            if parent.get() == parent_entity {
                Some((e, *uid))
            } else {
                None
            }
        })
        .collect();

    for (entity, uid) in endpoint_entities {
        world
            .entity_mut(entity)
            .remove::<VectorEndpointVM>()
            .insert(InternalObject);
        changed.push(uid);
    }

    respond.push_back(Effect::EntitiesChanged(changed));
}
