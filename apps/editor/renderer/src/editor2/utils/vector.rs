use bevy::{ecs::{system::SystemState, query::QueryEntityError}, prelude::*};

use crate::editor2::entities::{vector::{VectorObjectTag, VecNodeTag}, Bounded};


/// Given an entity, it will reset the object so that the origin (top left corner) is at 0,0 local
/// coordinates.
///
/// * `world`: 
/// * `entity`: 
pub fn reset_vector_object_origin(world: &mut World, entity: Entity) -> Result<(), QueryEntityError> {
    let mut sys_state: SystemState<(
        // Vector objects
        Query<(&Children, &mut Transform, &mut Bounded), (With<VectorObjectTag>, Without<VecNodeTag>)>,
        // Vector nodes
        Query<&mut Transform, (With<VecNodeTag>, Without<VectorObjectTag>)>,
    )> = SystemState::new(world);
    let (mut q_vec_obs, mut q_all_nodes) = sys_state.get_mut(world);

    let (children, mut transform, mut bounds) = q_vec_obs.get_mut(entity)?;
    let offset = q_all_nodes.iter_many(children).fold(Vec3::splat(f32::MAX), |acc, transform| acc.min(transform.translation));

    transform.translation += offset;
    *bounds = Bounded::NeedsCalculate;

    let mut it = q_all_nodes.iter_many_mut(children);
    while let Some(mut transform) = it.fetch_next() {
        transform.translation -= offset;
    }

    Ok(())
}
