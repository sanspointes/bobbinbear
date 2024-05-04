//! # Position
//! 
//! Module contains logic for a smart `Position` component that represents the position of an
//! object via a number of different methods.

use bevy::{ecs::reflect::ReflectComponent, prelude::*};
use bevy_spts_uid::{Uid, UidRegistry};

use crate::plugins::viewport::BobbinViewport;

#[derive(Component, Clone, Copy, Reflect)]
#[reflect(Component)]
pub enum Position {
    Local(Vec2),
    /// ProxyViewport automatically positions this element in the camera viewport (in the same position visually) 
    /// as the target entity.
    ProxyViewport {
        target: Uid,
        target_world_position: Vec3,
    },
}

impl Default for Position {
    fn default() -> Self {
        Self::Local(Vec2::default())
    }
}

#[allow(clippy::single_match)]
pub fn sys_pre_update_positions(
    mut q_positioned: Query<&mut Position, Without<Camera>>,
    q_transforms: Query<(&Transform, &GlobalTransform)>,
    uid_registry: Res<UidRegistry>,
) {
    for mut position in q_positioned.iter_mut() {
        match *position {
            Position::ProxyViewport { target, ref mut target_world_position } => {
                let entity = uid_registry.entity(target);
                let Ok((_, global_transform)) = q_transforms.get(entity) else {
                    warn!("sys_pre_update_positions: Can't get target for ViewportOfWorld {target:?}.");
                    continue;
                };

                *target_world_position = global_transform.translation();
            }
            _ => {},
        }
    }
}

pub fn sys_update_positions(
    q_camera: Query<(&Camera, &BobbinViewport, &GlobalTransform), With<Camera>>,
    mut q_positioned: Query<(&Position, &mut Transform), Without<Camera>>,
) {
    let (camera, viewport, camera_global_transform) = q_camera.single();

    for (position, mut transform) in q_positioned.iter_mut() {
        match position {
            Position::Local(pos) => {
                transform.translation.x = pos.x;
                transform.translation.y = pos.y;
            }
            Position::ProxyViewport { target: _, target_world_position } => {
                if let Some(pos) = camera.world_to_ndc(camera_global_transform, *target_world_position) {
                    let pos = viewport.ndc_to_viewport(pos.xy());
                    transform.translation.x = pos.x;
                    transform.translation.y = pos.y;
                }
            }
        }
    }
}
