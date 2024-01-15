use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
};

use crate::{
    msgs::{api::JsApiMsg, Msg, MsgRespondable, ToolMessage},
    types::{self},
};
use bevy::utils::HashMap;
use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

/**
* WASM Bindgen safe API Store
*/
#[derive(Clone)]
pub struct EditorApiStore {
    pub api_to_editor_sender: Sender<MsgRespondable>,
    pub editor_to_api_receiver: Receiver<JsApiMsg>,
}
struct Store {
    next_id: AtomicUsize,
    items: Mutex<HashMap<usize, EditorApiStore>>,
}

#[allow(dead_code)]
impl Store {
    pub fn new() -> Self {
        Self {
            next_id: AtomicUsize::new(0),
            items: Mutex::new(HashMap::new()),
        }
    }

    pub fn insert(&self, item: EditorApiStore) -> usize {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.items.lock().unwrap().insert(id, item);
        id
    }

    pub fn get(&self, id: usize) -> Option<EditorApiStore> {
        self.items.lock().unwrap().get(&id).cloned()
    }

    // Additional methods like remove, etc. can be added if needed
}

lazy_static! {
    static ref GLOBAL_STORE: Store = Store::new();
}

/**
* Editor API implementation
*/
#[wasm_bindgen]
pub struct EditorApi {
    id: usize,
}
impl EditorApi {
    pub fn new(
        api_to_editor_sender: Sender<MsgRespondable>,
        editor_to_api_receiver: Receiver<JsApiMsg>,
    ) -> Self {
        let store_item = EditorApiStore {
            api_to_editor_sender,
            editor_to_api_receiver,
        };
        let id = GLOBAL_STORE.insert(store_item);
        Self { id }
    }

    fn send_to_editor(&mut self, msg: impl Into<Msg>) -> usize {
        let v = GLOBAL_STORE
            .get(self.id)
            .expect("Could not get api sender/receiver");

        let msg = MsgRespondable::new(msg);
        let id = msg.1;

        v.api_to_editor_sender
            .send(msg)
            .expect("Could not send set_tool message.");

        return id;
    }
}

// impl EditorApiMethods for EditorApi {
#[wasm_bindgen]
impl EditorApi {
    #[wasm_bindgen]
    pub fn receive_msg(&mut self) -> Option<JsApiMsg> {
        let v = GLOBAL_STORE
            .get(self.id)
            .expect("Could not get api sender/receiver");
        v.editor_to_api_receiver.try_recv().ok()
    }

    #[wasm_bindgen]
    pub fn set_tool(&mut self, tool: types::BBTool) -> usize {
        self.send_to_editor(ToolMessage::SwitchTool(tool))
    }
}
