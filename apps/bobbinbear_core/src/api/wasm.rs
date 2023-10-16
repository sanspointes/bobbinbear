use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
};

use crate::{
    msgs::{self, frontend::FrontendMsg, Msg},
    types,
};
use bevy::utils::HashMap;
use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

use super::EditorApiMethods;
/**
* WASM Bindgen safe API Store
*/
#[derive(Clone)]
pub struct EditorApiStore {
    pub api_to_editor_sender: Sender<Msg>,
    pub editor_to_api_receiver: Receiver<FrontendMsg>,
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
        api_to_editor_sender: Sender<Msg>,
        editor_to_api_receiver: Receiver<FrontendMsg>,
    ) -> Self {
        let store_item = EditorApiStore {
            api_to_editor_sender,
            editor_to_api_receiver,
        };
        let id = GLOBAL_STORE.insert(store_item);
        Self { id }
    }
}

// impl EditorApiMethods for EditorApi {
#[wasm_bindgen]
impl EditorApi {
    #[wasm_bindgen]
    pub fn set_tool(&mut self, tool: types::BBTool) {
        let v = GLOBAL_STORE
            .get(self.id)
            .expect("Could not get api sender/receiver");
        v.api_to_editor_sender
            .send(msgs::ToolMessage::SwitchTool(tool).into())
            .expect("Could not send set_tool message.");
    }
}
