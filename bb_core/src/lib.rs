//! Displays a single [`Sprite`], created from an image.
pub mod api;
mod ecs;
mod materials;
mod meshes;
mod plugins;
mod tools;
mod utils;
mod views;

use bevy::asset::AssetMetaCheck;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::WindowMode;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_spts_changeset::events::ChangesetEvent;
use bevy_spts_uid::{Uid, UidRegistry};
use bevy_spts_vectorgraphic::VectorGraphicPlugin;
use bevy_wasm_api::BevyWasmApiPlugin;
use ecs::position::Position;
use ecs::{
    sys_cleanup_edge_positions_to_bounding_box, sys_sort_sync_position_proxy_and_transform,
    sys_sync_position_proxy_and_transform, sys_update_endpoint_positions_on_edge_move,
    InternalObject, ObjectType,
};
use materials::BobbinMaterialsPlugin;
use meshes::BobbinMeshesPlugin;
use plugins::inspecting::BecauseInspected;
use plugins::selected::SelectedPlugin;
use tools::{BobbinToolsPlugin, ToolSet};
use views::BobbinViewsPlugin;
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

    // Disable asset metadata checking.
    app.insert_resource(AssetMetaCheck::Never);

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

#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PosSet {
    // Positions the source objects and then updates transforms and global transforms
    PositionObjects,
    // Positions the proxy objects and then updates transforms and global transforms
    Propagate,
}

pub fn setup(app: &mut App) {
    app.add_event::<ChangesetEvent>();

    app.configure_sets(Update, PosSet::PositionObjects.after(ToolSet));
    app.configure_sets(Update, PosSet::Propagate.after(PosSet::PositionObjects));
    app.add_systems(
        Update,
        (
            sys_update_endpoint_positions_on_edge_move,
            sys_cleanup_edge_positions_to_bounding_box,
        )
            .chain()
            .in_set(PosSet::PositionObjects),
    );
    app.add_systems(
        Update,
        (sys_sort_sync_position_proxy_and_transform.pipe(sys_sync_position_proxy_and_transform))
            .chain()
            .in_set(PosSet::Propagate),
    );

    app.insert_resource(UidRegistry::default());
    app.register_type::<UidRegistry>();
    app.register_type::<HashMap<Uid, Entity>>();
    app.register_type::<Uid>();
    app.register_type::<Position>();
    app.register_type::<BecauseInspected>();
    app.register_type::<InternalObject>();
    app.register_type::<ObjectType>();

    app.add_plugins((
        DefaultInspectorConfigPlugin,
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
    ))
    // Sanspointes plugin libs
    .add_plugins((BevyWasmApiPlugin::default(), VectorGraphicPlugin))
    // App specific
    .add_plugins((
        BobbinMeshesPlugin,
        BobbinMaterialsPlugin,
        BobbinToolsPlugin,
        BobbinViewsPlugin,
        UndoRedoPlugin,
        Bounds2DPlugin,
        ViewportPlugin,
        EffectPlugin,
        SelectedPlugin,
    ));
}
