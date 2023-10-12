pub mod tools;
pub mod frontend;
pub mod keybinds;

use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{wasm::FrontendSender, plugins::input_plugin::InputMessage};

use self::{frontend::FrontendMsg, keybinds::msg_handler_keybinds};
pub use self::tools::{msg_handler_tool, ToolControllerPlugin, ToolMessage};

#[derive(Event, Clone, Debug)]
pub enum Message {
    // RawInput(RawInputMessage),
    Input(InputMessage),
    Frontend(FrontendMsg),
    Tool(ToolMessage),
}

impl From<FrontendMsg> for Message {
    fn from(value: FrontendMsg) -> Self {
        Self::Frontend(value)
    }
}
impl From<ToolMessage> for Message {
    fn from(value: ToolMessage) -> Self {
        Self::Tool(value)
    }
}
impl From<InputMessage> for Message {
    fn from(value: InputMessage) -> Self {
        Self::Input(value)
    }
}
// impl From<InputMessage> for Message {
//     fn from(value: InputMessage) -> Self {
//         Self::Tool(ToolMessage::Input(value))
//     }
// }

/// Entry point for a lot of the non-trivial interactivity of the system.
/// A lot of it requires exclusive world access and thus runs in a single thread.
///
/// * `world`:
pub fn sys_msg_handler(world: &mut World) {
    let mut messages = {
        if let Some(mut events) = world.get_resource_mut::<Events<Message>>() {
            events.drain().collect::<VecDeque<_>>()
        } else {
            warn!("WARN: Could not get messages to handle.  This should never happen but shouldn't cause issues");
            VecDeque::new()
        }
    };

    let mut iterations = 0;
    while let Some(msg) = messages.pop_front() {
        if iterations > 1000 {
            panic!("Too many messages in a single update.")
        }
        iterations += 1;
        match msg {
            Message::Input(input_message) => msg_handler_keybinds(world, &input_message, &mut messages),
            Message::Tool(tool_message) => msg_handler_tool(world, &tool_message, &mut messages),
            Message::Frontend(frontend_message) => {
                if let Some(frontend_sender) = world.get_resource_mut::<FrontendSender>() {
                    match frontend_sender.0.send(frontend_message) {
                        Err(reason) => panic!(
                            "Error sending message back to frontend. {:?} {:?}",
                            reason, reason.0
                        ),
                        _ => {}
                    }
                }
            }
        }
    }
}
