use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use super::{ execute_in_world, anyhow_result_to_js_result, ExecutionChannel };
use crate::changeset::{undo_change, redo_change};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct UndoRedoApi;

#[allow(dead_code)]
#[wasm_bindgen]
impl UndoRedoApi {

    #[wasm_bindgen]
    pub async fn undo(&self) -> js_sys::Promise {
        future_to_promise(execute_in_world(ExecutionChannel::FrameEnd, |w| {
            anyhow_result_to_js_result(undo_change(w))
        }))
    }

    #[wasm_bindgen]
    pub async fn redo(&self) -> js_sys::Promise {
        future_to_promise(execute_in_world(ExecutionChannel::FrameEnd, |w| {
            anyhow_result_to_js_result(redo_change(w))
        }))
    }

}
