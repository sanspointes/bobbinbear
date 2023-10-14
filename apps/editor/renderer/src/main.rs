// Contains the WASM api for interacting with the app,
// Generally these are just exposed functions that send messages.
mod wasm;

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
mod entities;
mod systems;
mod utils;

// #[cfg(debug_assertions)]
use crossbeam_channel::unbounded;

use bevy::prelude::*;
use bevy::DefaultPlugins;
use editor::start_bobbin_bear;
use msgs::Message;
use msgs::frontend::FrontendMsg;
use wasm::FrontendReceiver;
use wasm::FrontendSender;

fn main() {
    // TODO add webview gui
    let (_from_frontend_sender, from_frontend_receiver) = unbounded::<Message>();
    let (to_frontend_sender, _to_frontend_receiver) = unbounded::<FrontendMsg>();

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bobbin Bear :: Embroidery Editor".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    });

    let mut app = start_bobbin_bear(default_plugins);

    app.insert_resource(FrontendReceiver(from_frontend_receiver))
        .insert_resource(FrontendSender(to_frontend_sender));

    app.run()
}
