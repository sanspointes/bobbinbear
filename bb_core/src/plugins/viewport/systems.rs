use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::plugins::viewport::BobbinViewport;

use super::BobbinViewportResource;

pub fn sys_setup_viewport(
    mut viewport_resource: ResMut<BobbinViewportResource>,
    mut commands: Commands,
) {
    println!("Setup viewport");
    let entity = commands
        .spawn(Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(1., -1., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(VisibilityBundle::default())
        .insert(BobbinViewport::default())
        .insert(Name::from("Viewport"))
        .id();

    viewport_resource.viewport_entity = Some(entity);
}

pub fn sys_update_viewport_on_window_resize(
    mut ev_resize: EventReader<WindowResized>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_viewport: Query<&mut BobbinViewport>,
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
