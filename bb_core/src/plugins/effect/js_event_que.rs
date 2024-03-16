use bevy::prelude::*;
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
        loop {
            match self.receiver.lock().unwrap().try_recv() {
                Ok(event) => {
                    let js_value = serde_wasm_bindgen::to_value(&event).unwrap();
                    receiveRustEvents(js_value);
                },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
    }
}
