mod editor;
mod msgs;
mod plugins;
mod types;
mod wasm;
mod entities;
mod systems;

use crossbeam_channel::unbounded;

use bevy::prelude::*;
use editor::EditorPlugin;
use msgs::{Message, frontend::FrontendMsg};
use wasm_bindgen::prelude::wasm_bindgen;

// web app entry_point
#[wasm_bindgen]
pub fn main_web(container_id: String, set_api: js_sys::Function) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let (_from_frontend_sender, from_frontend_receiver) = unbounded::<Message>();
    let (to_frontend_sender, _to_frontend_receiver) = unbounded::<FrontendMsg>();

    // let api = EditorApi {
    //     dispatcher: from_frontend_sender,
    //     receiver: to_frontend_receiver,
    // };
    // set_api
    //     .call1(&JsValue::undefined(), &JsValue::from(api))
    //     .expect("Error sending api.");

    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_event::<Message>()
        // .insert_resource(FrontendReceiver(from_frontend_receiver))
        // .insert_resource(FrontendSender(to_frontend_sender))
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
        .add_plugins(EditorPlugin)
        // .add_system(set_window_icon.on_startup())
        .run();
}
