use std::{sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
}, time::Duration};

use crate::{msgs::{api::{ApiMsg, ApiResponseMsg}, MsgRespondable, ToolMessage}, types};
use bevy::utils::{HashMap, BoxedFuture};
use crossbeam_channel::{Receiver, Sender, select, RecvTimeoutError};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local};

/**
* WASM Bindgen safe API Store
*/
#[derive(Clone)]
pub struct EditorApiStore {
    pub api_to_editor_sender: Sender<MsgRespondable>,
    pub editor_to_api_receiver: Receiver<ApiMsg>,
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
        editor_to_api_receiver: Receiver<ApiMsg>,
    ) -> Self {
        let store_item = EditorApiStore {
            api_to_editor_sender,
            editor_to_api_receiver,
        };
        let id = GLOBAL_STORE.insert(store_item);
        Self { id }
    }
}

async fn get_responses_of_id(id: usize, receiver: Receiver<ApiMsg>) -> Result<Vec<ApiResponseMsg>, RecvTimeoutError> {
    let (result_sender, result_receiver): (Sender<Vec<ApiResponseMsg>>, Receiver<Vec<ApiResponseMsg>>) = crossbeam_channel::unbounded();
    // Clone the sender to send the result to the main thread.
    let result_sender_clone = result_sender.clone();

    spawn_local(async move {
        loop {
            select! {
                recv(receiver) -> result => {
                    let mut responses: Vec<ApiResponseMsg> = Vec::new();
                    if let Ok(ApiMsg::WrappedResponse(msg, response_id)) = result {
                        if response_id == id {
                            responses.push(msg);
                        }
                    }
                    if !responses.is_empty() {
                        let _ = result_sender_clone.send(responses);
                        break;
                    }
                }
            }
        }
    });

    result_receiver.recv_timeout(Duration::from_secs(3))
}

// impl EditorApiMethods for EditorApi {
#[wasm_bindgen]
impl EditorApi {
    #[wasm_bindgen]
    pub async fn set_tool(&mut self, tool: types::BBTool) -> Result<bool, JsError> {
        let v = GLOBAL_STORE
            .get(self.id)
            .expect("Could not get api sender/receiver");

        let msg = MsgRespondable::new(ToolMessage::SwitchTool(tool));
        let response_id = msg.1;
        v.api_to_editor_sender
            .send(msg)
            .expect("Could not send set_tool message.");

        match get_responses_of_id(response_id, v.editor_to_api_receiver).await {
            Ok(responses) => {
                Ok(responses.into_iter().any(|response| matches!(response, ApiResponseMsg::Success)))
            }
            Err(reason) => Err(JsError::from(reason)),
        }
    }
}
