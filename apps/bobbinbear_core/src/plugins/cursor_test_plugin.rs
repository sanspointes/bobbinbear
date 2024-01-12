use bevy::prelude::*;

use crate::utils::coordinates;

use super::screen_space_root_plugin::ScreenSpaceRootTag;

pub struct CursorTestPlugin;

impl Plugin for CursorTestPlugin {
    fn build(&self, app: &App) {}
}

#[derive(Component)]
pub struct CursorTestTag;

pub fn sys_setup_cursor_test(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    q_ss_root: Query<Entity, With<ScreenSpaceRootTag>>,
) {
    let ss_root = q_ss_root.single();
    //
    // let shape = shapes::Rectangle {
    //     extents: Vec2::new(20., 20.),
    //     ..Default::default()
    // };

    commands.entity(ss_root).with_children(|builder| {
        builder.spawn((
            CursorTestTag,
            Name::from("CursorTest"),
            MaterialMeshBundle {
                mesh: 
            },
        ));
    });
}

pub fn sys_update_cursor_test(mut q_cursor: Query<(&mut Transform), With<CursorTestTag>>) {
    coordinates::wor(screen_pos, window_size, proj_rect)
}
