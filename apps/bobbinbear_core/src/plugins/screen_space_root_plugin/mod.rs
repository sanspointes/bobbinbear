use std::ops::Div;

use bevy::{math::Vec3Swizzles, prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes::{self, RectangleOrigin},
};

use crate::{
    editor::EditorSet, msgs::sys_msg_handler, systems::camera::CameraTag, utils::coordinates,
};

#[derive(Component, Reflect, Default, Debug, Copy, Clone, PartialEq)]
#[reflect(Component)]
/// Component marking the entity that is the screenspace root.
/// Also has helper methods to convert from world coordinates to screen coordinates.
///
/// * `window_size`:
/// * `proj_rect`:
pub struct ScreenSpaceRoot {
    window_size: Vec2,
    world_bounds: Rect,
    projection_area: Rect,
}

#[allow(dead_code)]
impl ScreenSpaceRoot {
    pub fn world_to_screen(&self, world: impl Into<Vec2>) -> Vec2 {
        coordinates::world_to_screen(world.into(), self.window_size, self.world_bounds)
    }
    pub fn screen_to_world(&self, screen: impl Into<Vec2>) -> Vec2 {
        coordinates::screen_to_world(screen.into(), self.window_size, self.world_bounds)
    }
    pub fn window_size(&self) -> Vec2 {
        self.window_size
    }
    pub fn half_window_size(&self) -> Vec2 {
        self.window_size.div(2.)
    }
    pub fn world_bounds(&self) -> Rect {
        self.world_bounds
    }
    pub fn projection_area(&self) -> Rect {
        self.projection_area
    }
}

/// This plugin creates a new entity with component `ScreenSpaceRootTag` where
/// you can position content in screenspace coordinates and it will display on camera in
/// screenspace.
pub struct ScreenSpaceRootPlugin;
impl Plugin for ScreenSpaceRootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sys_setup)
            .add_systems(Update, sys_update_ss_root.in_set(EditorSet::PostMsgs));
        // In debug mode show the test bounds elements
        #[cfg(debug_assertions)]
        {
            app.add_systems(PostStartup, sys_setup_screenspace_test)
                .add_systems(Update, sys_update_screenspace_test.after(sys_msg_handler));
        }
    }
}

/// Creates the screenspace root.
fn sys_setup(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(Entity, &OrthographicProjection), With<CameraTag>>,
) {
    let (cam_entity, orth_proj) = q_camera.single();
    let window = q_window.single();
    let window_size = Vec2::new(window.width(), window.height());

    let e_ss_root = commands
        .spawn((
            Name::from("ScreenSpaceRootTag"),
            ScreenSpaceRoot {
                window_size,
                world_bounds: orth_proj.area,
                projection_area: orth_proj.area,
            },
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(0., 0., 500.),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .set_parent(cam_entity)
        .id();
}

/// Updates the screenspace root in accordance with the camera projection
pub fn sys_update_ss_root(
    mut q_ss_root: Query<(&mut Transform, &mut ScreenSpaceRoot)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&OrthographicProjection, &GlobalTransform), With<CameraTag>>,
) {
    let (mut ss_root_transform, mut ss_root) = q_ss_root.single_mut();

    let window = q_window.single();
    let window_size = Vec2::new(window.width(), window.height());
    let (ortho, global_trans) = q_camera.single();
    let mut proj_rect = ortho.area;

    let projection_area = Rect::from_corners(proj_rect.min, proj_rect.max);
    proj_rect.min += global_trans.translation().xy();
    proj_rect.max += global_trans.translation().xy();

    let world_bounds = proj_rect;

    let new_ss_root = ScreenSpaceRoot {
        window_size,
        world_bounds,
        projection_area,
    };
    ss_root.set_if_neq(new_ss_root);


    let half_size = window_size.div(2.);
    ss_root_transform.translation.x = -half_size.x;
    ss_root_transform.translation.y = -half_size.y;
}

#[derive(Component)]
enum TestTagOrientation {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}
fn sys_setup_screenspace_test(
    mut commands: Commands,
    q_ss_root: Query<Entity, With<ScreenSpaceRoot>>,
) {
    let root = q_ss_root.single();

    commands.entity(root).with_children(|builder| {
        use TestTagOrientation::*;
        let to_build = vec![TopLeft, TopRight, BottomLeft, BottomRight];
        for orientation in to_build {
            let origin = match orientation {
                BottomRight => RectangleOrigin::BottomRight,
                BottomLeft => RectangleOrigin::BottomLeft,
                TopRight => RectangleOrigin::TopRight,
                TopLeft => RectangleOrigin::TopLeft,
            };

            let name = match orientation {
                BottomRight => "BottomRightTest",
                BottomLeft => "BottomLeftTest",
                TopRight => "TopRightTest",
                TopLeft => "TopLeftTest",
            };

            let tl_shape = shapes::Rectangle {
                extents: Vec2::new(50., 50.),
                origin,
            };
            builder.spawn((
                Name::from(name),
                orientation,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&tl_shape),
                    ..default()
                },
                Fill::color(Color::rgba(0., 0., 0.5, 0.1)),
            ));
        }
    });
}

fn sys_update_screenspace_test(
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_test_elements: Query<(&mut Transform, &TestTagOrientation)>,
) {
    let primary_window = q_window.single();
    let window_size = Vec2::new(primary_window.width(), primary_window.height());

    for (mut transform, target_orientation) in q_test_elements.iter_mut() {
        use TestTagOrientation::*;
        match target_orientation {
            TopLeft => {
                transform.translation.x = 0.;
                transform.translation.y = window_size.y;
            }
            BottomLeft => {
                transform.translation.x = 0.;
                transform.translation.y = 0.;
            }
            TopRight => {
                transform.translation.x = window_size.x;
                transform.translation.y = window_size.y;
            }
            BottomRight => {
                transform.translation.x = window_size.x;
                transform.translation.y = 0.;
            }
        };
    }
}
