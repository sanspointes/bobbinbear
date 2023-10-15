use std::ops::Div;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::view::RenderLayers,
    window::PrimaryWindow,
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes::{self, RectangleOrigin},
};

use crate::{constants::BB_LAYER_UI, editor::EditorSet};

#[derive(Component, Default)]
pub struct ScreenSpaceCameraTag;
#[derive(Component, Default)]
pub struct ScreenSpaceRootTag;

/// This plugin creates a new entity with component `ScreenSpaceRootTag` where
/// you can position content in screenspace coordinates and it will display on camera in
/// screenspace.
pub struct ScreenSpaceRootPlugin;
impl Plugin for ScreenSpaceRootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, sys_setup)
            .add_systems(Update, sys_update_transform);
        // In debug mode show the test bounds elements
        #[cfg(debug_assertions)]
        {
            app.add_systems(PostStartup, sys_setup_screenspace_test)
                .add_systems(Update, sys_update_screenspace_test.in_set(EditorSet::PostMsgs));
        }
    }
}

/// Creates the screenspace root.
fn sys_setup(mut commands: Commands) {
    commands.spawn((
        Name::from("ScreenSpaceRootTag"),
        ScreenSpaceRootTag,
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 500.),
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    commands.spawn((
        Name::from("ScreenSpaceCamera"),
        ScreenSpaceCameraTag::default(),
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            ..Default::default()
        },
        ComputedVisibility::default(),
        RenderLayers::layer(BB_LAYER_UI),
    ));
    println!("Adding screenspace root to camera.");
}
/// Updates the screenspace root in accordance with the camera projection
fn sys_update_transform(
    mut q_ss_root: Query<&mut Transform, With<ScreenSpaceRootTag>>, 
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_window.single();
    let half_size = Vec2::new(window.width(), window.height()).div(2.);
    let mut ss_root_transform = q_ss_root.single_mut();
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
    q_ss_root: Query<Entity, With<ScreenSpaceRootTag>>,
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
                ..default()
            };
            builder.spawn((
                Name::from(name),
                orientation,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&tl_shape),
                    ..default()
                },
                Fill::color(Color::rgba(0., 0., 0.5, 0.1)),
                RenderLayers::layer(BB_LAYER_UI),
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
            },
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
