use bevy::{
    math::prelude::Rectangle, prelude::*, render::camera::CameraProjection,
    sprite::MaterialMesh2dBundle, window::WindowResized,
};

use crate::plugins::viewport::Viewport;

pub fn sys_setup_viewport(
    mut commands: Commands,
) {
    println!("Setup viewport");
    commands
        .spawn(SpatialBundle::default())
        .insert(Viewport::default())
        .insert(Name::from("Viewport"))
        .with_children(|commands| {
            commands.spawn(Camera2dBundle::default());
        });
}

pub fn sys_update_viewport_on_window_resize(
    mut ev_resize: EventReader<WindowResized>,
    mut q_viewport: Query<&mut Viewport>,
) {
    let mut viewport = q_viewport.single_mut();

    if let Some(ev) = ev_resize.read().last() {
        viewport.window_size = Vec2::new(ev.width, ev.height);
        if viewport.target_size.length() == 0. {
            viewport.target_size = viewport.window_size;
        }
    }
}

pub fn sys_update_camera_from_viewport(
    mut q_viewport: Query<(&Viewport, &mut Transform), Without<Camera>>,
    mut q_primary_camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
) {
    let (viewport, mut transform) = q_viewport.single_mut();
    let (ref mut projection, mut cam_transform) = q_primary_camera.single_mut();

    // Update zoom level
    projection.update(viewport.target_size.x, viewport.target_size.y);
    // Update position
    transform.translation = (viewport.target_position - viewport.target_size / 2.).extend(100.);
    cam_transform.translation = (viewport.target_size / 2.).extend(100.);
}
