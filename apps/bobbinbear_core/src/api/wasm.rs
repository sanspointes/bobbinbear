use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Mutex,
};

use crate::{msgs::{api::JsApiMsg, MsgRespondable, ToolMessage}, types};
use bevy::{utils::HashMap, prelude::debug};
use crossbeam_channel::{Receiver, Sender};
use js_sys::{Promise, Array};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

use super::msg_polling::{ApiResponseQue, start_cancleable_raf, WasmPoll};

/**
* WASM Bindgen safe API Store
*/
#[derive(Clone)]
pub struct EditorApiStore {
    pub api_to_editor_sender: Sender<MsgRespondable>,
    pub editor_to_api_receiver: ApiResponseQue,
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
            editor_to_api_receiver: ApiResponseQue::new(editor_to_api_receiver),
        };
        let id = GLOBAL_STORE.insert(store_item);
        Self { id }
    }
}

// impl EditorApiMethods for EditorApi {
#[wasm_bindgen]
impl EditorApi {
    #[wasm_bindgen]
    pub async fn set_tool(&mut self, tool: types::BBTool) -> Promise {
        debug!("EditorApi.set_tool({:?})", tool);
        let v = GLOBAL_STORE
            .get(self.id)
            .expect("Could not get api sender/receiver");

        debug!("EditorApi.set_tool(..) -> Sending msg.");
        let msg = MsgRespondable::new(ToolMessage::SwitchTool(tool));
        let response_id = msg.1;
        v.api_to_editor_sender
            .send(msg)
            .expect("Could not send set_tool message.");
        debug!("EditorApi.set_tool(..) -> Awaiting responses.");

        let promise = js_sys::Promise::new(&mut |res, rej| {
            let sid = self.id.clone();
            let rsid = response_id.clone();
            let cb = Box::new(move || {
                let mut v = GLOBAL_STORE
                    .get(sid)
                    .expect("Could not get api sender/receiver");

                let responses = v.editor_to_api_receiver.extract_responses_with_id(rsid);

                match responses {
                    None => WasmPoll::Pending,
                    Some(responses) => {
                        let data = serde_json::to_string(&responses);
                        // let data: Result<Vec<_>, _> = responses.into_iter().map(|msg| serde_json::to_string(&msg)).collect();

                        match data {
                            Ok(serialised) => {
                                res.call1(&JsValue::undefined(), &JsValue::from(serialised));
                            }
                            Err(reason) => {
                                rej.call1(&JsValue::undefined(), &JsValue::from(reason.to_string()));
                            }
                        };
                        WasmPoll::Ready
                    },
                }
            });

            start_cancleable_raf(cb)
        });

        promise
    }
}
