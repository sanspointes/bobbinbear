mod components;
mod editor;
mod msgs;
mod plugins;
mod types;
mod wasm;
mod systems;
mod utils;
mod constants;

use crossbeam_channel::unbounded;

use bevy::prelude::*;
use editor::start_bobbin_bear;
use msgs::{Message, frontend::FrontendMsg};
use wasm::{FrontendReceiver, FrontendSender};
use wasm_bindgen::prelude::wasm_bindgen;

// web app entry_point
#[wasm_bindgen]
pub fn main_web(container_id: String, _set_api: js_sys::Function) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let (_from_frontend_sender, from_frontend_receiver) = unbounded::<Message>();
    let (to_frontend_sender, _to_frontend_receiver) = unbounded::<FrontendMsg>();

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bobbin Bear :: Embroidery Editor".to_string(),
            resolution: (800., 600.).into(),
            canvas: Some(container_id),
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        ..Default::default()
    });

    let mut app = start_bobbin_bear(default_plugins);

    app.insert_resource(FrontendReceiver(from_frontend_receiver))
        .insert_resource(FrontendSender(to_frontend_sender));

    app.run()
}
