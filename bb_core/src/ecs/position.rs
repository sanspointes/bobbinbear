//! # Position
//! 
//! Module contains logic for a smart `Position` component that represents the position of an
//! object via a number of different methods.

use bevy::{ecs::reflect::ReflectComponent, prelude::*};

use crate::plugins::viewport::Viewport;

#[derive(Component, Clone, Copy, Reflect)]
#[reflect(Component)]
pub enum CalcPosition {
    /// ViewportOfWorld automatically positions this element in the camera viewport (in the same position visually) 
    /// as the target entity.
    ViewportOfWorld {
        target: Entity,
        target_world_position: Vec3,
    },
}

#[allow(clippy::single_match)]
pub fn sys_pre_update_positions(
    mut q_positioned: Query<&mut CalcPosition, Without<Camera>>,
    q_transforms: Query<(&Transform, &GlobalTransform)>,
) {
    for mut position in q_positioned.iter_mut() {
        match *position {
            CalcPosition::ViewportOfWorld { target, ref mut target_world_position } => {
                let Ok((_, global_transform)) = q_transforms.get(target) else {
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
    q_camera: Query<(&Camera, &Viewport, &GlobalTransform), With<Camera>>,
    mut q_positioned: Query<(&CalcPosition, &mut Transform, &GlobalTransform), Without<Camera>>,
) {
    let (camera, viewport, camera_global_transform) = q_camera.single();

    for (position, mut transform, global_transform) in q_positioned.iter_mut() {
        match position {
            CalcPosition::ViewportOfWorld { target, target_world_position } => {
                if let Some(pos) = camera.world_to_ndc(camera_global_transform, *target_world_position) {
                    let pos = viewport.ndc_to_viewport(pos.xy());
                    transform.translation.x = pos.x;
                    transform.translation.y = pos.y;
                }
            }
        }
    }
}
