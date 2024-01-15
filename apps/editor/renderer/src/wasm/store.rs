use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

use crate::messages::frontend::FrontendMsg;
use crate::messages::Msg;

#[derive(Clone)]
pub struct EditorApiStore {
    pub from_frontend: Sender<Msg>,
    pub to_frontend: Receiver<FrontendMsg>,
}

pub struct Store {
    next_id: AtomicUsize,
    items: Mutex<HashMap<usize, EditorApiStore>>,
}

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
    pub static ref GLOBAL_STORE: Store = Store::new();
}
