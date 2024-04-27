use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use super::Viewport;

#[derive(Component)]
pub enum ViewportDebug {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

pub fn sys_setup_viewport_debug(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_viewport: Query<Entity, With<Viewport>>,
) {
    let mesh = meshes.add(Rectangle::new(20., 20.));
    let parent = q_viewport.single();

    commands.spawn((
        MaterialMesh2dBundle {
            material: materials.add(Color::hsl(0., 0.5, 0.5)),
            mesh: mesh.clone().into(),
            ..Default::default()
        },
        ViewportDebug::TopLeft,
    )).set_parent(parent);
    commands.spawn((
        MaterialMesh2dBundle {
            material: materials.add(Color::hsl(0.25, 0.5, 0.5)),
            mesh: mesh.clone().into(),
            ..Default::default()
        },
        ViewportDebug::BottomLeft,
    )).set_parent(parent);
    commands.spawn((
        MaterialMesh2dBundle {
            material: materials.add(Color::hsl(0.5, 0.5, 0.5)),
            mesh: mesh.clone().into(),
            ..Default::default()
        },
        ViewportDebug::TopRight,
    )).set_parent(parent);
    commands.spawn((
        MaterialMesh2dBundle {
            material: materials.add(Color::hsl(0.75, 0.5, 0.5)),
            mesh: mesh.clone().into(),
            ..Default::default()
        },
        ViewportDebug::BottomRight,
    )).set_parent(parent);
}

pub fn sys_update_viewport_debug_positions(
    q_viewport: Query<&Viewport>,
    mut q_viewport_debug: Query<(&ViewportDebug, &mut Transform)>,
) {
    let viewport = q_viewport.single();

    for (viewport_debug, mut transform) in q_viewport_debug.iter_mut() {
        match viewport_debug {
            ViewportDebug::TopLeft => {
                let pos = viewport.ndc_to_viewport(Vec2::new(-1., -1.));
                transform.translation.x = pos.x;
                transform.translation.y = pos.y;
            }
            ViewportDebug::BottomLeft => {
                let pos = viewport.ndc_to_viewport(Vec2::new(-1., 1.));
                transform.translation.x = pos.x;
                transform.translation.y = pos.y;
            }
            ViewportDebug::BottomRight => {
                let pos = viewport.ndc_to_viewport(Vec2::new(1., 1.));
                transform.translation.x = pos.x;
                transform.translation.y = pos.y;
            }
            ViewportDebug::TopRight => {
                let pos = viewport.ndc_to_viewport(Vec2::new(1., -1.));
                transform.translation.x = pos.x;
                transform.translation.y = pos.y;
            }
        }
    }
}
