use bevy::prelude::*;
use bevy_wasm_api::convert::JsArrayBuilder;
use std::sync::{
    mpsc::{channel, Receiver, Sender, TryRecvError},
    Arc, Mutex,
};
use wasm_bindgen::{prelude::*, JsValue};

use super::Effect;

#[wasm_bindgen]
extern "C" {
    fn receiveRustEvents(events: JsValue);
}

#[derive(Resource, Debug)]
pub struct EffectQue {
    sender: Sender<Effect>,
    receiver: Arc<Mutex<Receiver<Effect>>>,
}

impl EffectQue {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
        }
    }

    /// Adds the incoming event via the sender.
    /// Sending over a channel is inherently thread-safe.
    pub fn push_effect(&self, event: Effect) {
        self.sender.send(event).unwrap();
    }

    pub fn forward_effects_to_js(&mut self) {
        // Drain the receiver and push all events to the events vector
        let mut had_effect = false;
        let mut events_array = JsArrayBuilder::new();
        loop {
            match self.receiver.lock().unwrap().try_recv() {
                Ok(event) => {
                    had_effect = true;
                    events_array = events_array.with_js_value(&serde_wasm_bindgen::to_value(&event).unwrap());
                },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
        if had_effect {
            receiveRustEvents(events_array.build_as_js_value());
        }
    }
}
