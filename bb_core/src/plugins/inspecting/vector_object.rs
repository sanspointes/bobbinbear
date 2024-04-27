use std::collections::VecDeque;

use bevy::{ecs::system::SystemState, prelude::*, sprite::Mesh2dHandle};
use bevy_spts_uid::Uid;
use bevy_spts_vectorgraphic::components::Endpoint;

use crate::{
    ecs::position::CalcPosition,
    plugins::{effect::Effect, selected::Selected, viewport::Viewport},
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

    respond.push_back(Effect::EntitiesChanged(vec![uninspected]));
    respond.push_back(Effect::EntitiesDespawned(to_despawn));
}

pub fn handle_inspect_vector_object_endpoints(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    inspected: Uid,
) {
    let mut sys_state = SystemState::<(
        ResMut<Assets<Mesh>>,
        ResMut<Assets<ColorMaterial>>,
        Query<(Entity, &Uid, &Parent, &Transform), With<Endpoint>>,
    )>::new(world);

    let parent_entity = inspected.entity(world).unwrap();
    let (mut meshes, mut materials, q_endpoints) = sys_state.get_mut(world);

    let mesh = Mesh2dHandle(meshes.add(Rectangle::new(5., 5.)));
    let material = materials.add(Color::WHITE);

    let mut spawned = vec![];
    let mut to_spawn = vec![];
    for (entity, _uid, parent, transform) in q_endpoints.iter() {
        if parent.get() != parent_entity {
            continue;
        }

        let uid = Uid::default();
        spawned.push(uid);

        to_spawn.push((
            Name::from("Node"),
            BecauseInspected(inspected),
            CalcPosition::ViewportOfWorld {
                target: entity,
                target_world_position: Vec3::ZERO,
            },
            uid,
            *transform,
            GlobalTransform::default(),
            Visibility::default(),
            ViewVisibility::default(),
            InheritedVisibility::default(),
            mesh.clone(),
            material.clone(),
            Selected::Deselected,
        ));
    }

    respond.push_back(Effect::EntitiesChanged(vec![inspected]));
    respond.push_back(Effect::EntitiesSpawned(spawned));

    let viewport = world.query_filtered::<Entity, With<Viewport>>().single(world);
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
