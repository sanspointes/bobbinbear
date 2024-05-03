use std::collections::VecDeque;

use bevy::{ecs::system::SystemState, prelude::*};
use bevy_spts_uid::Uid;
use bevy_spts_vectorgraphic::components::Endpoint;

use crate::{
    ecs::{InternalObject, ObjectBundle}, materials::UiElMaterial, meshes::BobbinMeshes, plugins::{effect::Effect, selected::Selected, viewport::Viewport}
};

use super::BecauseInspected;

pub fn handle_inspect_vector_object(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    inspected: Uid,
) {
    handle_inspect_vector_object_endpoints(respond, world, inspected);
}

pub fn handle_uninspect_vector_object(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    uninspected: Uid,
) {
    let mut q_inspect_because = world.query::<(&Uid, &BecauseInspected)>();
    let parent_entity = uninspected.entity(world).unwrap();

    let mut changed = vec![uninspected];
    let to_despawn: Vec<Uid> = q_inspect_because
        .iter(world)
        .filter_map(|(uid, because_inspected)| {
            if because_inspected.0 == uninspected {
                Some(*uid)
            } else {
                None
            }
        })
        .collect();

    for uid in &to_despawn {
        if let Some(e) = uid.entity(world) {
            // if let Some(parent_entity) = world.get::<Parent>(e).map(|p| p.get()) {
            // };

            world.entity_mut(e).remove_parent();
            world.despawn(e);
        }
    }

    let mut q_endpoints = world.query_filtered::<(Entity, &Uid, &Parent), With<Endpoint>>();
    let (endpoint_entities, endpoint_uids): (Vec<Entity>, Vec<Uid>) = q_endpoints
        .iter(world)
        .filter_map(|(entity, uid, parent)| {
            if parent_entity == parent.get() {
                Some((entity, *uid))
            } else {
                None
            }
        })
        .unzip();
    for entity in endpoint_entities {
        world.entity_mut(entity).remove::<Visibility>();
        world.entity_mut(entity).remove::<Selected>();
        world.entity_mut(entity).remove::<Name>();
    }
    changed.extend(endpoint_uids);

    respond.push_back(Effect::EntitiesChanged(changed));
    respond.push_back(Effect::EntitiesDespawned(to_despawn));
}

pub fn handle_inspect_vector_object_endpoints(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    inspected: Uid,
) {
    let mut sys_state = SystemState::<(
        BobbinMeshes,
        ResMut<Assets<UiElMaterial>>,
        Query<(Entity, &Uid, &Parent, &Transform), With<Endpoint>>,
    )>::new(world);

    let parent_entity = inspected.entity(world).unwrap();
    let (meshes, mut materials, q_endpoints) = sys_state.get_mut(world);

    let mesh = meshes.endpoint_mesh();

    let mut changed = vec![inspected];
    let mut spawned = vec![];
    let mut to_spawn = vec![];

    let endpoints: Vec<_> = q_endpoints
        .iter()
        .map(|(e, uid, parent, transform)| (e, *uid, parent.get(), *transform, materials.add(UiElMaterial::default())))
        .collect();
    for (entity, endpoint_uid, parent, _, material) in endpoints {
        if parent != parent_entity {
            continue;
        }
        changed.push(endpoint_uid);

        let uid = Uid::default();
        spawned.push(uid);


        to_spawn.push((
            ObjectBundle::proxy_viewport(endpoint_uid).with_z_position(1.),
            BecauseInspected(inspected),
            InternalObject,
            uid,
            mesh.clone(),
            material,
        ));

        world
            .entity_mut(entity)
            .remove::<InternalObject>()
            .insert(Name::from("Endpoint"));
    }

    respond.push_back(Effect::EntitiesChanged(changed));
    respond.push_back(Effect::EntitiesSpawned(spawned));

    let viewport = world
        .query_filtered::<Entity, With<Viewport>>()
        .single(world);
    for bundle in to_spawn {
        world.spawn(bundle).set_parent(viewport);
    }
}

#[allow(dead_code)]
pub fn handle_uninspect_vector_object_endpoints(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    uid: Uid,
) {
}
