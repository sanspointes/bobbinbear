use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum CoordinateSpace {
    Viewport,
    World,
    Local,
}

#[derive(Component, Clone, Copy, Debug)]
/// Copies a position from one entity to another, applying the necessary transforms to switch
/// coordinate space if necessary.
///
/// * `source_space`:
/// * `target_space`:
/// * `target`:
pub struct SyncedPosition {
    pub world_position: Vec3,
    pub target_space: CoordinateSpace,
    pub target: Entity,
}

impl SyncedPosition {
    pub fn new_to_viewport(target: Entity) -> Self {
        Self {
            world_position: Vec3::ZERO,
            target_space: CoordinateSpace::Viewport,
            target,
        }
    }
}

pub fn sys_sync_position_collect_changes(
    mut q_changed: Query<(Entity, &mut SyncedPosition, &GlobalTransform)>,
) -> Vec<SyncedPosition> {
    let mut changed = vec![];

    for (e, mut synced_pos, global_transform) in q_changed.iter_mut() {
        let world_position = global_transform.translation();
        let world_position_changed = world_position != synced_pos.world_position;
        synced_pos.world_position = world_position;

        if world_position_changed {
            changed.push(synced_pos.clone());
        }
    }

    changed
}

pub fn sys_update_sync_position(
    In(to_update): In<Vec<SyncedPosition>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut q_target: Query<(&mut Transform, &GlobalTransform, Option<&Parent>), Without<Camera>>,
) {
    let (camera, camera_global_position) = q_camera.single();
    for synced_position in to_update {
        match synced_position.target_space {
            CoordinateSpace::World => {
                todo!()
            }
            CoordinateSpace::Local => {
                todo!()
            }
            CoordinateSpace::Viewport => {
                let Ok((mut transform, _, _)) = q_target.get_mut(synced_position.target) else {
                    continue;
                };
                let Some(position) = camera
                    .world_to_viewport(camera_global_position, synced_position.world_position) else {
                    continue;
                };
                transform.translation.x = position.x;
                transform.translation.y = position.y;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn sys_sync_positions(
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut param_set: ParamSet<(
        // q_changed
        Query<(Entity, &SyncedPosition, &Transform), Changed<Transform>>,
        // q_other
        Query<&mut Transform>,
    )>,
) {
    let camera = q_camera.single();

    // let moved: Vec<_> = param_set
    //     .p0()
    //     .iter()
    //     .map(|(e, synced_position, transform)| (e, *synced_position, transform.translation))
    //     .collect();
    //
    // let mut q_other = param_set.p1();
    // for (e, synced_with, position) in moved {
    //     let other_entity = synced_with.get_target();
    //     let Some(mut transform) = q_other.get_mut(other_entity).ok() else {
    //         warn!("Entity with SyncedPosition({e:?}) references entity without a Transform component.");
    //         continue;
    //     };
    //
    //     transform.translation = position;
    // }
}
