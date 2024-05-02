use bevy::{
    math::prelude::Rectangle,
    prelude::*,
    render::camera::CameraProjection,
    sprite::MaterialMesh2dBundle,
    window::{PrimaryWindow, WindowResized},
};

use crate::plugins::viewport::Viewport;

pub fn sys_setup_viewport(mut commands: Commands) {
    println!("Setup viewport");
    commands
        .spawn(Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(1., -1., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Viewport::default())
        .insert(Name::from("Viewport"));
}

pub fn sys_update_viewport_on_window_resize(
    mut ev_resize: EventReader<WindowResized>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_viewport: Query<&mut Viewport>,
) {
    let window = q_window.single();
    let mut viewport = q_viewport.single_mut();

    if let Some(ev) = ev_resize.read().last() {
        viewport.logical_size = Vec2::new(ev.width, ev.height);
        viewport.physical_size = Vec2::new(
            window.resolution.physical_width() as f32,
            window.resolution.physical_height() as f32,
        );
    }
}

pub fn sys_update_camera_from_viewport(
    mut q_viewport: Query<(&Viewport, &mut Transform), Without<Camera>>,
    mut q_primary_camera: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
) {
    let (viewport, mut transform) = q_viewport.single_mut();
    let (ref mut projection, mut cam_transform) = q_primary_camera.single_mut();

    // Update zoom level
    // projection.update(viewport.target_size.x, viewport.target_size.y);
    // Update position
    // transform.translation = (viewport.target_position - viewport.target_size / 2.).extend(100.);
    // cam_transform.translation = (viewport.target_size / 2.).extend(100.);
    // cam_transform.translation.y *= -1.;
}
