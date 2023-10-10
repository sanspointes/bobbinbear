pub mod editor2;
// mod sketch;
pub mod types;

use crossbeam_channel::{unbounded, Receiver, Sender};
use js_sys::JsString;
use std::str::FromStr;

use bevy::prelude::*;
use editor2::{
    msgs::{DocMessage, ToolMessage, Tool},
    frontend::{FrontendMessage, FrontendReceiver, FrontendSender},
    EditorPlugin, Message,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct EditorApi {
    dispatcher: Sender<Message>,
    receiver: Receiver<FrontendMessage>,
}

#[wasm_bindgen]
impl EditorApi {
    /// Adds a new document
    /// * `name`: Name for the document
    /// * `width`: Width of the new document
    /// * `height`: Height of the new document
    pub async fn add_document(&mut self, name: JsString, width: f32, height: f32) {
        self.dispatcher
            .send(Message::Document(DocMessage::Create {
                    name: name.into(),
                    size: Vec2::new(width, height),
            }))
            .expect("Error adding document.");
    }

    /// Focuses a specific document
    ///
    /// * `id`: Id of the document you'd like to focus
    pub async fn focus_document(&mut self, id: usize) {
        self.dispatcher
            .send(Message::Document(DocMessage::SetActive(id)))
            .expect("Error focusing document.");
    }

    pub async fn delete_document(&mut self, id: usize) {
        self.dispatcher
            .send(Message::Document(DocMessage::Delete(id)))
            .expect("Error deleting document.");
    }

    pub async fn resize_document(&mut self, id: usize, new_width: f32, new_height: f32) {
        self.dispatcher
            .send(Message::Document(DocMessage::Resize{
                id,
                new_size: Vec2::new(new_width, new_height),
            }))
            .expect("Error resizing document.");
    }

    /// Set the current tool
    ///
    /// * `tool`: The tool to select
    pub async fn set_tool(&mut self, tool: Tool) {
        self.dispatcher
            .send(Message::Tool(ToolMessage::SwitchTool(tool)))
            .expect("Error setting the current tool.");
    }

    pub async fn receive_messages(&mut self, handler: js_sys::Function) {
        while let Ok(msg) = self.receiver.try_recv() {
            let serialized =
                serde_json::to_string(&msg).expect("Failed to serialize frontend message");
            let js_value = js_sys::JsString::from_str(&serialized)
                .expect("Failed to convert serialized to JsValue");
            use web_sys::console;
            console::log_1(&js_value);
            handler.call1(&JsValue::undefined(), &js_value).unwrap();
        }
    }
}

// web app entry_point
#[wasm_bindgen]
pub fn main_web(container_id: String, set_api: js_sys::Function) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let (from_frontend_sender, from_frontend_receiver) = unbounded::<Message>();
    let (to_frontend_sender, to_frontend_receiver) = unbounded::<FrontendMessage>();

    let api = EditorApi {
        dispatcher: from_frontend_sender,
        receiver: to_frontend_receiver,
    };
    set_api
        .call1(&JsValue::undefined(), &JsValue::from(api))
        .expect("Error sending api.");

    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_event::<Message>()
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
        .add_plugin(EditorPlugin {})
        // .add_system(set_window_icon.on_startup())
        .run();
}
