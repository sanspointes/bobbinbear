mod api;

mod components;
mod constants;
mod editor;
mod msgs;
mod plugins;
mod systems;
mod types;
mod utils;
mod prelude;
mod serialisation;
mod events;

use api::{ApiToEditorReceiver, EditorApi, EditorToApiSender};
use crossbeam_channel::unbounded;

use bevy::prelude::*;
use editor::start_bobbin_bear;
use msgs::{api::JsApiMsg, MsgRespondable};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// web app entry_point
#[wasm_bindgen]
pub fn main_web(canvas_id: String, set_api: js_sys::Function) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let (api_to_editor_sender, api_to_editor_receiver) = unbounded::<MsgRespondable>();
    let (editor_to_api_sender, editor_to_api_receiver) = unbounded::<JsApiMsg>();

    let api = EditorApi::new(api_to_editor_sender, editor_to_api_receiver);
    set_api.call1(&JsValue::undefined(), &JsValue::from(api)).expect("BobbinBear: Error passing API back to JS land.");

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

    let mut app = start_bobbin_bear(default_plugins);

    app.insert_resource(ApiToEditorReceiver(api_to_editor_receiver))
        .insert_resource(EditorToApiSender(editor_to_api_sender));

    app.run()
}
