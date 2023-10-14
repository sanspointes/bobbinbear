use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};

#[derive(Component, Default)]
pub struct CameraTag {
    pub min_x: Option<f32>,
    pub max_x: Option<f32>,
    pub min_y: Option<f32>,
    pub max_y: Option<f32>,
}

pub fn sys_setup_camera(mut commands: Commands) {
    debug!("sys_setup_camera: Setting up camera.");
    commands.spawn((
        CameraTag::default(),
        Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(1., -1., 1.),
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(1., 0., 0.)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}
