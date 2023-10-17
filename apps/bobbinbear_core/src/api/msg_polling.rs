use crossbeam_channel::{Receiver, TryRecvError};
use js_sys::Function;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};
use thiserror::Error;
use wasm_bindgen::prelude::Closure;

use crate::msgs::api::{JsApiMsg, ApiResponseMsg, JsApiResponseMsg};

#[derive(Error, Debug)]
pub enum ReceiverFutureError {
    #[error("Receiver disconnected.")]
    ReceiverDisconnected,
}

// Stores a filter predicate to check if it can handle the message (remove from que)
// and the actual message handler itself
type ResponsePredicate = Arc<dyn Fn(&JsApiMsg) -> bool>;

#[derive(Clone)]
pub struct ApiResponseQue {
    receiver: Receiver<JsApiMsg>,
    to_handle_que: VecDeque<JsApiMsg>,
}

impl ApiResponseQue {
    pub fn new(receiver: Receiver<JsApiMsg>) -> Self {
        let poller = Self {
            receiver,
            to_handle_que: VecDeque::new(),
        };
        poller
    }

    /// Receives messages from the JsApiMsg receiver
    pub fn receive(&mut self) {
        let mut new_messages: VecDeque<_> = self.receiver.try_recv().into_iter().collect();
        self.to_handle_que.append(&mut new_messages);
    }

    /// Extracts msgs matching predicate
    ///
    /// * `predicate`:
    pub fn extract_msgs_with_predicate(&mut self, predicate: ResponsePredicate) -> Vec<JsApiMsg> {
        let (matches, non_matches): (Vec<_>, Vec<_>) = self
            .to_handle_que
            .iter()
            .cloned()
            .partition(|msg| predicate(msg));

        self.to_handle_que = non_matches.into();
        matches
    }

    pub fn extract_responses_with_id(&mut self, response_id: usize) -> Option<Vec<JsApiResponseMsg>> {
        let rsid = response_id.clone();
        let same_id_predicate = Arc::new(move |msg: &JsApiMsg| match msg {
            JsApiMsg::Response(_, id) => rsid == *id,
            _ => false,
        });
        let extracted = self.extract_msgs_with_predicate(same_id_predicate);

        if extracted.len() > 1 {
            panic!("ApiResponseQue.extract_responses_with_id() has more than one response group.");
        }

        if let Some(JsApiMsg::Response(msgs, _)) = extracted.first() {
            return Some(msgs.clone());
        }
        None
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub enum WasmPoll {
    Ready,
    Pending,
}

pub fn start_cancleable_raf(mut callback: Box<dyn FnMut() -> WasmPoll>) {
    let in_closure = Rc::new(RefCell::new(None));
    let out_closure = in_closure.clone();

    *out_closure.borrow_mut() = Some(Closure::new(move || {
        let v = (callback)();
        
        match v {
            WasmPoll::Pending => {
                request_animation_frame(in_closure.borrow().as_ref().unwrap());
            }
            WasmPoll::Ready => {

            }
        };
    }));

    request_animation_frame(out_closure.borrow().as_ref().unwrap());
}
