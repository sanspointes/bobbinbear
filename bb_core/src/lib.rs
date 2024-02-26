//! Displays a single [`Sprite`], created from an image.
mod api;
mod undoredo;
mod ecs;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::math::prelude::Circle;
use bevy_wasm_api::BevyWasmApiPlugin;
use ecs::node::{sys_derived_mesh_for_node, sys_derived_material_for_node};
use undoredo::UndoRedoPlugin;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn setup_bb_core(canvas_id: String) {
    let mut app = App::new();

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bobbin Bear :: Embroidery Editor".to_string(),
            canvas: Some(canvas_id),
            ..Default::default()
        }),
        ..Default::default()
    });

    app.add_plugins(default_plugins)
        // App plugins
        .add_plugins(BevyWasmApiPlugin)
        .add_plugins(UndoRedoPlugin)

        .add_systems(PostUpdate, (sys_derived_mesh_for_node, sys_derived_material_for_node))

        .add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(50.)).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        });
}
