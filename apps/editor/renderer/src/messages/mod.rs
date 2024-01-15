pub mod input;
pub mod tools;

use std::collections::VecDeque;

use bevy::prelude::*;

use crate::utils::messages::RecursiveMessageQue;

use self::frontend::FrontendMsg;

pub mod frontend;

#[derive(Event)]
pub enum Msg {
    Frontend(FrontendMsg),
}

pub fn sys_msg_handler(world: &mut World) {
    let mut messages = {
        if let Some(mut events) = world.get_resource_mut::<Events<Msg>>() {
            events.drain().collect::<VecDeque<_>>()
        } else {
            warn!("WARN: Could not get messages to handle.  This should never happen but shouldn't cause issues");
            VecDeque::new()
        }
    };

    let mut msg_que: RecursiveMessageQue<Msg> = messages.into();
    msg_que.handle(|msg| {
        match msg {
            Msg::Frontend(frontend_msg) => {

            }
        }
        None
    })
}
