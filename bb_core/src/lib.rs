//! Displays a single [`Sprite`], created from an image.
pub mod api;
mod ecs;
mod plugins;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy::utils::HashMap;
use bevy::window::WindowMode;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_spts_changeset::events::ChangesetEvent;
use bevy_spts_uid::{Uid, UidRegistry};
use bevy_spts_vectorgraphic::VectorGraphicPlugin;
use bevy_wasm_api::BevyWasmApiPlugin;
use ecs::position::{sys_pre_update_positions, sys_update_positions, CalcPosition};
use plugins::inspecting::BecauseInspected;
use wasm_bindgen::prelude::*;

use plugins::bounds2d::Bounds2DPlugin;
use plugins::effect::EffectPlugin;
use plugins::undoredo::UndoRedoPlugin;
use plugins::viewport::ViewportPlugin;

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
            resizable: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        }),
        ..Default::default()
    });
    app.add_plugins(default_plugins);

    setup(&mut app);

    app.run()
}

pub fn setup(app: &mut App) {
    app.add_event::<ChangesetEvent>();

    app.add_systems(
        PostUpdate,
        (sys_pre_update_positions.pipe(sys_update_positions))
            .before(TransformSystem::TransformPropagate),
    );

    app.insert_resource(UidRegistry::default());
    app.register_type::<UidRegistry>();
    app.register_type::<HashMap<Uid, Entity>>();
    app.register_type::<Uid>();
    app.register_type::<CalcPosition>();
    app.register_type::<BecauseInspected>();

    app.add_plugins((
        DefaultInspectorConfigPlugin,
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
    ))
    // App plugins
    .add_plugins(BevyWasmApiPlugin)
    .add_plugins(VectorGraphicPlugin)
    .add_plugins((UndoRedoPlugin, Bounds2DPlugin, ViewportPlugin, EffectPlugin));
}
