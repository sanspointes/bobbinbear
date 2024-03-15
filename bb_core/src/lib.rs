//! Displays a single [`Sprite`], created from an image.
pub mod api;
mod ecs;
mod plugins;
mod selected;
mod undoredo;

use bevy::prelude::*;
use bevy_spts_vectorgraphic::VectorGraphicPlugin;
use bevy_wasm_api::BevyWasmApiPlugin;
use plugins::bounds2d::Bounds2DPlugin;
use plugins::viewport::ViewportPlugin;
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
    app.add_plugins(default_plugins);

    setup(&mut app);

    app.run()
}

pub fn setup(app: &mut App) {
    app
        // App plugins
        .add_plugins(BevyWasmApiPlugin)
        .add_plugins(VectorGraphicPlugin)
        .add_plugins((UndoRedoPlugin, Bounds2DPlugin, ViewportPlugin));
}
