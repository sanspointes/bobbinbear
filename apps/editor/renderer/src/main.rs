// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod editor2;
// mod sketch;
pub mod types;

// #[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_prototype_lyon::render::ShapeMaterial;
use crossbeam_channel::unbounded;
use editor2::entities::{
    vector::{Ordered, PathSegment},
    Bounded, HoveredState, SelectedState,
};

use crate::editor2::{
    frontend::{FrontendMessage, FrontendReceiver, FrontendSender},
    EditorPlugin, Message,
};

use bevy::prelude::*;
use bevy::DefaultPlugins;

fn main() {
    // TODO add webview gui
    let (_from_frontend_sender, from_frontend_receiver) = unbounded::<Message>();
    let (to_frontend_sender, _to_frontend_receiver) = unbounded::<FrontendMessage>();
    let mut app = App::new();
    app.insert_resource(TaskPoolOptions::with_num_threads(1))
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_event::<Message>()
        .insert_resource(FrontendReceiver(from_frontend_receiver))
        .insert_resource(FrontendSender(to_frontend_sender))
        .add_plugins(DefaultPlugins.build().set(WindowPlugin {
            primary_window: Some(Window {
                // title: "Bevy game".to_string(), // ToDo
                // resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EditorPlugin {});

    // #[cfg(debug_assertions)]
    {
        app.add_plugins(WorldInspectorPlugin::new());
        app.register_type::<ShapeMaterial>();
        app.register_type::<PathSegment>();
        app.register_type::<HoveredState>();
        app.register_type::<SelectedState>();
        app.register_type::<Bounded>();
        app.register_type::<Ordered>();
    }

    app.run();
}
