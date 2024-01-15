use bevy::prelude::*;
use bevy_mod_raycast::{RaycastMesh, RaycastMethod, RaycastSource, RaycastSystem};
use bevy_prototype_lyon::prelude::*;

use crate::systems::camera::{sys_setup_camera, CameraTag};

#[derive(Debug, Clone, Reflect)]
pub struct RaycastHitplane;

pub struct RaycastPlugin;
impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                (
                    sys_setup_raycast.after(sys_setup_camera),
                    sys_setup_hitplane,
                ),
            )
            .add_systems(PreUpdate, sys_raycast_to_cursor.before(RaycastSystem::BuildRays::<RaycastHitplane>))
        ;
    }
}
/// Sets up the camera to have a raycaster on it.
///
/// * `commands`:
/// * `camera`:
pub fn sys_setup_raycast(mut commands: Commands, mut camera: Query<Entity, With<CameraTag>>) {
    let entity = camera.single_mut();
    if let Some(mut commands) = commands.get_entity(entity) {
        commands.insert(RaycastSource::<RaycastHitplane>::default());
        println!("sys_setup_raycast: Got camera.")
    } else {
        panic!("sys_setup_raycast: Cannot get camera.")
    }
}

#[derive(Component)]
pub struct HitPlaneTag;

/// Sets up the hitplane, which we use to get general world coordinates
/// cursor coordinates in the input systems.
///
/// * `commands`: 
fn sys_setup_hitplane(mut commands: Commands) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(10000., 10000.),
        ..Default::default()
    };
    commands.spawn((
        HitPlaneTag {},
        Name::from("BgHitPlane"),
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform {
                translation: Vec3::new(0., 0., 100.),
                ..Default::default()
            },
            ..Default::default()
        },
        RaycastMesh::<RaycastHitplane>::default(),
        Fill::color(Color::rgb(0.2, 0.2, 0.2)),
    ));
}

/// Tells the raycaster to raycast at cursor position
///
/// * `cursor`: 
/// * `raycast_hit_plane`: 
fn sys_raycast_to_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut raycast_hit_plane: Query<&mut RaycastSource<RaycastHitplane>>,
) {
    // Grab the most recent cursor event if it exists:
    let Some(cursor_moved) = cursor.iter().last() else {
        return;
    };
    for mut pick_source in &mut raycast_hit_plane {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
    }
}

