mod types;
// Contains the WASM api for interacting with the app,
// Generally these are just exposed functions that send messages.
mod wasm;

// Contains message definitions + handlers, these implement
// the higher level behaviours of our app and modify the entities in the scene.
mod messages;

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


mod systems;

mod plugins;

mod editor;
mod utils;

//

use bevy::prelude::*;
use crossbeam_channel::unbounded;
use messages::{Msg, frontend::FrontendMsg};
use wasm::{EditorApi, FrontendReceiver, FrontendSender};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, renderer!");
}

// web app entry_point
#[wasm_bindgen]
pub fn main_web(container_id: String, set_api: js_sys::Function) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let (from_frontend_sender, from_frontend_receiver) = unbounded::<Msg>();
    let (to_frontend_sender, to_frontend_receiver) = unbounded::<FrontendMsg>();

    let api = EditorApi::new(to_frontend_receiver, from_frontend_sender);
    set_api
        .call1(&JsValue::undefined(), &JsValue::from(api))
        .expect("Error sending api.");

    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_event::<Msg>()
        .insert_resource(FrontendReceiver(from_frontend_receiver))
        .insert_resource(FrontendSender(to_frontend_sender))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".to_string(), // ToDo
                resolution: (800., 600.).into(),
                canvas: Some(container_id),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        
        // .add_system(set_window_icon.on_startup())
        .run();
}
