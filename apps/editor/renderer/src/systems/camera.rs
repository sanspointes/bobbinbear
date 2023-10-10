use bevy::prelude::*;

#[derive(Component, Default)]
pub struct CameraTag {
    pub min_x: Option<f32>,
    pub max_x: Option<f32>,
    pub min_y: Option<f32>,
    pub max_y: Option<f32>,
}

pub fn sys_setup_camera(mut commands: Commands) {
    println!("sys_setup_camera: Setting up camera.");
    commands.spawn((
        CameraTag::default(),
        Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(1., -1., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}
