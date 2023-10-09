use bevy::prelude::*;

#[derive(Component)]
pub struct CameraTag;

pub fn sys_setup_camera(mut commands: Commands) {
    println!("sys_setup_camera: Setting up camera.");
    commands.spawn((
        CameraTag,
        Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(1., -1., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}
