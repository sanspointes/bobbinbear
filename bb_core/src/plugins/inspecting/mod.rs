mod components;
mod vector_object;

use std::collections::VecDeque;

use bevy::prelude::*;

use bevy_spts_uid::Uid;
use bevy_spts_vectorgraphic::components::VectorGraphic;
pub use components::*;

use crate::plugins::effect::Effect;

use self::vector_object::{handle_inspect_vector_object, handle_uninspect_vector_object};

pub struct InspectingPlugin;

impl Plugin for InspectingPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn handle_inspection_changed(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    inspected: Option<Uid>,
    uninspected: Option<Uid>,
) {
    if let Some(uninspected) = uninspected {
        cleanup_previous_inspection(respond, world, uninspected);
    }

    if let Some(inspected) = inspected {
        inspect_object(respond, world, inspected);
    }

    // let mut sys_state = SystemState::<(
    //     ResMut<Assets<Mesh>>,
    //     ResMut<Assets<ColorMaterial>>,
    //     Query<(&Uid, &Endpoint, &Transform)>,
    // )>::new(world);
    //
    // let (mut meshes, mut materials, mut q_endpoints) = sys_state.get_mut(world);
    //
    // let mesh = meshes.add(Circle::new(5.));
    // let material = materials.add(Color::WHITE);
    //
    // let mut spawned = vec![];
    // let mut to_spawn = vec![];
    // for (uid, ep, transform) in q_endpoints.iter() {
    //     let uid = Uid::default();
    //     spawned.push(uid);
    //
    //     to_spawn.push((
    //         Name::from("Node"),
    //         BecauseInspected(uid),
    //         uid,
    //         transform.clone(),
    //         GlobalTransform::default(),
    //         Visibility::default(),
    //         ViewVisibility::default(),
    //         InheritedVisibility::default(),
    //         mesh.clone(),
    //         material.clone(),
    //         Selected::Deselected,
    //     ));
    // }
    //
    // for bundle in to_spawn {
    //     world.spawn(bundle);
    // }
    //
    // respond.push_back(Effect::EntitiesSpawned(spawned));
}

fn inspect_object(respond: &mut VecDeque<Effect>, world: &mut World, inspected: Uid) {
    let entity = inspected.entity(world).unwrap();

    if world.get::<VectorGraphic>(entity).is_some() {
        handle_inspect_vector_object(respond, world, inspected);
    }
}

fn cleanup_previous_inspection(
    respond: &mut VecDeque<Effect>,
    world: &mut World,
    uninspected: Uid,
) {
    let entity = uninspected.entity(world).unwrap();

    if world.get::<VectorGraphic>(entity).is_some() {
        handle_uninspect_vector_object(respond, world, uninspected);
    }
}
