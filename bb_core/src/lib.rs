//! Displays a single [`Sprite`], created from an image.
mod api;
mod changeset;
mod serialise;
mod index;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use api::IpcPlugin;
use changeset::ChangesetPlugin;
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
            resolution: (10., 10.).into(),
            canvas: Some(canvas_id),
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        ..Default::default()
    });

    app.add_plugins(default_plugins)
        // App plugins
        .add_plugins(IpcPlugin)
        .add_plugins(ChangesetPlugin)

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
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        });
}
