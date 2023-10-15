use bevy::prelude::*;
use bevy_mod_raycast::{RaycastSource, RaycastMethod};

use crate::systems::camera::CameraTag;

use super::Selectable;

pub fn sys_setup_selection_raycast(
    mut commands: Commands,
    q_camera: Query<Entity, With<CameraTag>>,
) {
    let e_camera = q_camera.single();
    commands
        .get_entity(e_camera)
        .expect("sys_setup_selection_raycast: Cannot get camera")
        .insert(RaycastSource::<Selectable>::default());
}

pub fn sys_selection_raycast_update_ray(
    mut q_raycast_source: Query<&mut RaycastSource<Selectable>>,
    mut ev_cursor_moved: EventReader<CursorMoved>,
) {
    let mut source = q_raycast_source.single_mut();
    for ev in &mut ev_cursor_moved {
        source.cast_method = RaycastMethod::Screenspace(ev.position);
    }
}
