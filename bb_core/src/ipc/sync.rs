use core::panic;
use std::{sync::{Arc, Mutex}, borrow::{Borrow, BorrowMut}, cell::RefCell};

use wasm_bindgen::prelude::*;
use bevy::prelude::World;
use js_sys::{Promise};
use wasm_bindgen_futures::{JsFuture, future_to_promise};
use futures::channel::oneshot;

pub(crate) fn execute_world_tasks_begin(world: &mut World) {
    let receiver = CHANNEL_FRAME_START.with(|rx| {
        let rx = &rx.borrow().1;
        while let Ok(task) = rx.borrow().try_recv() {
            (task.task)(world);
        }
    });
}

pub(crate) fn execute_world_tasks_end(world: &mut World) {
    let receiver = CHANNEL_FRAME_END.with(|rx| {
        let rx = &rx.borrow().1;
        while let Ok(task) = rx.borrow().try_recv() {
            (task.task)(world);
        }
    });
}

struct WorldTask {
    task: Box<dyn FnOnce(&mut World) + 'static>,
}
// {
//     let (tx, rx) = futures::channel::oneshot::channel();
//
//     let output = Arc::new(Mutex::new(None));
//     let output_cloned = output.clone();
//     let boxed_task = Box::new(move |world: &mut World| {
//         let mut output = output_cloned.lock().unwrap();
//         *output = Some(task(world));
//         tx.send(());
//         output
//     });
//
//     let world_task = unsafe { WorldTask { task: boxed_task } };
//     {
//         let channel = match channel {
//             ExecutionChannel::FrameStart => &CHANNEL_FRAME_START.0,
//             ExecutionChannel::FrameEnd => &CHANNEL_FRAME_END.0,
//             ExecutionChannel::RenderApp => &CHANNEL_RENDER_APP.0,
//         };
//
//         let sender = channel.lock().unwrap();
//         sender.send(world_task).unwrap();
//     }
//
//     spawn_local(async {
//
//     });
//
//     let v = JsFuture::from(promise).await;
//
//     let mut output = output.lock().unwrap();
//     output.take().unwrap()
// }

// Convert a oneshot::Receiver into a JavaScript Promise
fn rx_to_promise(rx: oneshot::Receiver<()>) -> Promise {
    future_to_promise(async move {
        match rx.await {
            Ok(result) => Ok(JsValue::NULL),
            Err(e) => panic!("rx_to_promise: Panic with {e:?}"),
        }
    })
}

pub(crate) async fn execute_in_world<
    T: 'static,
    F: FnOnce(&mut World) -> T + 'static,
>(
    channel: ExecutionChannel,
    task: F,
) -> T {

    let (tx, rx) = oneshot::channel();
    let output = Arc::new(Mutex::new(None));

    let output_cloned = output.clone();
    let boxed_task = Box::new(move |world: &mut World| {
        let mut output = output_cloned.lock().unwrap();
        *output = Some(task(world));
        tx.send(()).expect("Failed to send task complete.");
    });

    let world_task = WorldTask { task: boxed_task };
    {
        let channel = match channel {
            ExecutionChannel::FrameStart => &CHANNEL_FRAME_START,
            ExecutionChannel::FrameEnd => &CHANNEL_FRAME_END,
        };

        channel.with(|channel| {
            channel.borrow().0.borrow_mut().send(world_task).unwrap();
        });
    }

    JsFuture::from(rx_to_promise(rx)).await.unwrap();

    let mut output = output.lock().unwrap();
    output.take().unwrap()
}

use std::sync::mpsc::{ Sender, Receiver };

thread_local! {

    pub static CHANNEL_FRAME_START: (RefCell<Sender<WorldTask>>, RefCell<Receiver<WorldTask>>) = {
        let (tx, rx) = std::sync::mpsc::channel();
        (RefCell::new(tx), RefCell::new(rx))
    };
    pub static CHANNEL_FRAME_END: (RefCell<Sender<WorldTask>>, RefCell<Receiver<WorldTask>>) = {
        let (tx, rx) = std::sync::mpsc::channel();
        (RefCell::new(tx), RefCell::new(rx))
    };
}

// lazy_static::lazy_static! {
//   static ref CHANNEL_FRAME_START: (Mutex<std::sync::mpsc::Sender<WorldTask>>, Mutex<std::sync::mpsc::Receiver<WorldTask>>) = {
//     let (rx, tx) = std::sync::mpsc::channel();
//     (Mutex::new(rx), Mutex::new(tx))
//   };
//   static ref CHANNEL_FRAME_END: (Mutex<std::sync::mpsc::Sender<WorldTask>>, Mutex<std::sync::mpsc::Receiver<WorldTask>>) = {
//     let (rx, tx) = std::sync::mpsc::channel();
//     (Mutex::new(rx), Mutex::new(tx))
//   };
//   static ref CHANNEL_RENDER_APP: (Mutex<std::sync::mpsc::Sender<WorldTask>>, Mutex<std::sync::mpsc::Receiver<WorldTask>>) = {
//     let (rx, tx) = std::sync::mpsc::channel();
//     (Mutex::new(rx), Mutex::new(tx))
//   };
// }

pub(crate) enum ExecutionChannel {
    FrameStart,
    FrameEnd,
}
