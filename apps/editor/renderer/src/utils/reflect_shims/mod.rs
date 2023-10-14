mod reflectable_fill;
mod reflectable_path;

use bevy::ecs::query::QueryEntityError;
use bevy::{ecs::system::SystemState, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::utils::scene::get_all_children_recursive;

pub use self::reflectable_fill::ReflectableFill;
pub use self::reflectable_path::ReflectablePath;

pub fn patch_world_entities_for_reflection(
    world: &mut World,
    entities: &Vec<Entity>,
) -> Result<(), QueryEntityError> {
    // Collect the transforms to apply
    let mut q_to_patch = world.query::<(Entity, Option<&Path>, Option<&Fill>)>();
    let to_apply: Vec<_> = q_to_patch
        .iter_many(world, entities)
        .map(|(e, maybe_path, maybe_fill)| {
            let refl_path: Option<ReflectablePath> = match maybe_path {
                Some(lyon_path) => Some(lyon_path.0.clone().into()),
                None => None,
            };
            let refl_fill: Option<ReflectableFill> = match maybe_fill {
                Some(lyon_fill) => Some(lyon_fill.clone().into()),
                None => None,
            };

            (e, refl_path, refl_fill)
        })
        .collect();

    // Apply to world
    for (e, refl_path, refl_fill) in to_apply {
        match refl_path {
            Some(refl_path) => {
                println!("\tPatching Path -> ReflectablePath for {:?}", e);
                world.entity_mut(e).insert(refl_path);
            }
            None => (),
        }
        match refl_fill {
            Some(refl_fill) => {
                println!("\tPatching Fill -> ReflectableFill for {:?}", e);
                world.entity_mut(e).insert(refl_fill);
            }
            None => (),
        }
    }

    Ok::<_, QueryEntityError>(())
}

pub fn patch_world_subhierarchy_for_reflection(
    world: &mut World,
    entity: Entity,
) -> Result<Vec<Entity>, QueryEntityError> {
    println!("\nPatching world for reflection/serialisation.");

    println!("\tpre-patch: {entity:?}");
    let cmps = world.inspect_entity(entity);
    for cmp in cmps {
        println!("\t\t - {:?}", cmp.name());
    }

    let mut sys_state: SystemState<Query<Option<&Children>>> = SystemState::new(world);
    let children_query = sys_state.get(world);
    let mut entities: Vec<Entity> = Vec::new();
    get_all_children_recursive(entity, &children_query, &mut entities);

    patch_world_entities_for_reflection(world, &entities)?;

    println!("\tpost-patch: {entity:?}");
    let cmps = world.inspect_entity(entity);
    for cmp in cmps {
        println!("\t\t - {:?}", cmp.name());
    }
    println!("\n");

    Ok(entities)
}

pub fn patch_world_entities_for_playback(
    world: &mut World,
    entities: &Vec<Entity>,
) -> Result<(), QueryEntityError> {
    // Collect the transforms to apply
    let mut q_to_patch = world.query::<(Entity, Option<&ReflectablePath>, Option<&ReflectableFill>)>();
    let to_apply: Vec<_> = q_to_patch
        .iter_many(world, entities)
        .map(|(e, maybe_path, maybe_fill)| {
            let lyon_path: Option<Path> = match maybe_path {
                Some(refl_path) => Some(refl_path.clone().into()),
                None => None,
            };
            let lyon_fill: Option<Fill> = match maybe_fill {
                Some(refl_fill) => Some(refl_fill.clone().into()),
                None => None,
            };

            (e, lyon_path, lyon_fill)
        })
        .collect();

    // Apply to world
    for (e, lyon_path, lyon_fill) in to_apply {
        match lyon_path {
            Some(lyon_path) => {
                println!("\tPatching ReflectablePath -> Path for {:?}", e);
                world.entity_mut(e).remove::<ReflectablePath>().insert(lyon_path);
            }
            None => (),
        }
        match lyon_fill {
            Some(lyon_fill) => {
                println!("\tPatching ReflectableFill -> Fill for {:?}", e);
                world.entity_mut(e).remove::<ReflectableFill>().insert(lyon_fill);
            }
            None => (),
        }
    }

    Ok::<_, QueryEntityError>(())
}

pub fn patch_world_subhierarchy_for_playback(
    world: &mut World,
    entity: Entity,
) -> Result<Vec<Entity>, QueryEntityError> {
    println!("\nPatching world for playback/deserialisation.");

    println!("\tpre-patch: {entity:?}");
    let cmps = world.inspect_entity(entity);
    for cmp in cmps {
        println!("\t\t - {:?}", cmp.name());
    }

    let mut sys_state: SystemState<Query<Option<&Children>>> = SystemState::new(world);
    let children_query = sys_state.get(world);
    let mut entities: Vec<Entity> = Vec::new();
    get_all_children_recursive(entity, &children_query, &mut entities);

    patch_world_entities_for_playback(world, &entities)?;

    println!("\tpost-patch: {entity:?}");
    let cmps = world.inspect_entity(entity);
    for cmp in cmps {
        println!("\t\t - {:?}", cmp.name());
    }
    println!("\n");

    Ok(entities)
}
