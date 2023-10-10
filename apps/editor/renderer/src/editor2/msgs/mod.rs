mod tools;

use std::collections::VecDeque;

use bevy::prelude::*;

pub use self::tools::{handle_tool_message, Tool, ToolControllerPlugin, ToolMessage};

use super::{
    frontend::{FrontendMessage, FrontendSender},
    input::InputMessage,
};

#[derive(Event, Clone, Debug)]
pub enum Message {
    // RawInput(RawInputMessage),
    // Input(InputMessage),
    Frontend(FrontendMessage),
    Tool(ToolMessage),
}

impl From<FrontendMessage> for Message {
    fn from(value: FrontendMessage) -> Self {
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
        Self::Tool(ToolMessage::Input(value))
    }
}

/// Entry point for a lot of the non-trivial interactivity of the system.
/// A lot of it requires exclusive world access and thus runs in a single thread.
///
/// * `world`:
pub fn editor_msg_system(world: &mut World) {
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
            Message::Tool(tool_message) => handle_tool_message(world, &tool_message, &mut messages),
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
