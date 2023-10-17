// #[cfg(target_arch = "wasm32")]
mod msg_polling;
mod wasm;

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};

use crate::{
    msgs::{api::JsApiMsg, MsgRespondable},
    types::BBTool,
};

// Wasm API for editor
pub use wasm::EditorApi;

// This file defines the public API for interfacing with bobbinbear editor.
// It defines a trait that is implemented in `wasm` submodule (and later desktop if there's a way
// to integrate with tauri).

// These resources wrap sender/receiver to be stored in the bevy engine
// The inverse sender/receiver will be stored within the EditorApi struct
#[derive(Resource, Debug)]
pub struct ApiToEditorReceiver(pub Receiver<MsgRespondable>);
#[derive(Resource)]
pub struct EditorToApiSender(pub Sender<JsApiMsg>);

/// Trait contains shared methods between WASM editor api and future editor apis.
trait EditorApiMethods {
    fn set_tool(&mut self, tool: BBTool);
}
