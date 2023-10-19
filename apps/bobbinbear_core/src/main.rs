// Contains the WASM api for interacting with the app,
// Generally these are just exposed functions that send messages.
mod api;

mod components;

// Contains message definitions + handlers, these implement
// the higher level behaviours of our app and modify the entities in the scene.
mod msgs;

// Contains entities + component definitions.
// These should largely remain high level based on functionality, such as an enum describing
// which hatching style to use.
// mod entities;

// Plugin definitions: basically tiny app specific libraries to make certian behaviours easier.
// These take the high level components from the `entities` module and attempt to perform the
// behaviour "magically".
// i.e. Managed delete functionality
// i.e. Drawing the path of a vector object.
// i.e. Converting and drawing the hatching styles of an entity.
// mod plugins;


// mod systems;

mod plugins;

mod editor;
// mod utils;


mod types;
mod systems;
mod utils;
mod constants;

// #[cfg(debug_assertions)]
use crossbeam_channel::unbounded;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use editor::start_bobbin_bear;
use msgs::MsgRespondable;
use api::EditorToApiSender;
use api::ApiToEditorReceiver;
use msgs::api::JsApiMsg;

fn main() {
    // TODO add webview gui
    let (_api_to_editor_sender, api_to_editor_receiver) = unbounded::<MsgRespondable>();
    let (editor_to_api_sender, _editor_to_api_receiver) = unbounded::<JsApiMsg>();

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bobbin Bear :: Embroidery Editor".to_string(),
            resizable: true,
            ..Default::default()
        }),
        ..Default::default()
    });

    let mut app = start_bobbin_bear(default_plugins);

    app.insert_resource(ApiToEditorReceiver(api_to_editor_receiver))
        .insert_resource(EditorToApiSender(editor_to_api_sender));

    app.run()
}
