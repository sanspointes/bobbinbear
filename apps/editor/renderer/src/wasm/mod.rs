
pub mod store;

use bevy::prelude::*;
use crossbeam_channel::{Sender, Receiver};
use wasm_bindgen::prelude::*;

use store::{GLOBAL_STORE, EditorApiStore};
use crate::msgs::{Message, frontend::FrontendMsg};

#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[derive(Resource, Debug)]
// Can receive messages from the javascript layer
pub struct FrontendReceiver(pub Receiver<Message>);

#[derive(Resource)]
/// Can send messages to the javascript layer
pub struct FrontendSender(pub Sender<FrontendMsg>);

#[wasm_bindgen]
#[allow(dead_code)]
pub struct EditorApi {
    id: usize,
}

impl EditorApi {
    pub fn new (to_frontend: Receiver<FrontendMsg>, from_frontend: Sender<Message>) -> Self {
        let store_item = EditorApiStore { to_frontend, from_frontend };
        let id = GLOBAL_STORE.insert(store_item);
        Self {
            id,
        }
    }
    // pub async fn add_document(&mut self, name: JsString, width: f32, height: f32) {
    //     self.dispatcher
    //         .send(Message::Document(DocMessage::Create {
    //                 name: name.into(),
    //                 size: Vec2::new(width, height),
    //         }))
    //         .expect("Error adding document.");
    // }
    //
    // /// Focuses a specific document
    // ///
    // /// * `id`: Id of the document you'd like to focus
    // pub async fn focus_document(&mut self, id: usize) {
    //     self.dispatcher
    //         .send(Message::Document(DocMessage::SetActive(id)))
    //         .expect("Error focusing document.");
    // }
    //
    // pub async fn delete_document(&mut self, id: usize) {
    //     self.dispatcher
    //         .send(Message::Document(DocMessage::Delete(id)))
    //         .expect("Error deleting document.");
    // }
    //
    // pub async fn resize_document(&mut self, id: usize, new_width: f32, new_height: f32) {
    //     self.dispatcher
    //         .send(Message::Document(DocMessage::Resize{
    //             id,
    //             new_size: Vec2::new(new_width, new_height),
    //         }))
    //         .expect("Error resizing document.");
    // }
    //
    // /// Set the current tool
    // ///
    // /// * `tool`: The tool to select
    // pub async fn set_tool(&mut self, tool: Tool) {
    //     self.dispatcher
    //         .send(Message::Tool(ToolMessage::SwitchTool(tool)))
    //         .expect("Error setting the current tool.");
    // }
    //
    // pub async fn receive_messages(&mut self, handler: js_sys::Function) {
    //     while let Ok(msg) = self.receiver.try_recv() {
    //         let serialized =
    //             serde_json::to_string(&msg).expect("Failed to serialize frontend message");
    //         let js_value = js_sys::JsString::from_str(&serialized)
    //             .expect("Failed to convert serialized to JsValue");
    //         use web_sys::console;
    //         console::log_1(&js_value);
    //         handler.call1(&JsValue::undefined(), &js_value).unwrap();
    //     }
    // }
}
