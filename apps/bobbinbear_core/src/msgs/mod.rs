pub mod cmds;
pub mod frontend;
pub mod keybinds;
pub mod tools;

use std::{
    collections::VecDeque,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy::prelude::*;
use lazy_static::lazy_static;

use crate::{api::EditorToApiSender, plugins::input_plugin::InputMessage};

pub use self::tools::{msg_handler_tool, ToolMessage, ToolMsgPlugin};
use self::{
    cmds::{msg_handler_cmds, CmdMsg},
    frontend::FrontendMsg,
    keybinds::msg_handler_keybinds,
};

#[derive(Event, Clone, Debug)]
pub enum Msg {
    // RawInput(RawInputMessage),
    Input(InputMessage),
    Frontend(FrontendMsg),
    Tool(ToolMessage),
    Cmd(CmdMsg),
}
// All messages are wrapped with an identifier to allow responses.
pub struct MsgWrapper(pub Msg, pub usize);

lazy_static! {
    static ref MSG_ID_PROVIDER: AtomicUsize = AtomicUsize::new(0);
}

impl From<FrontendMsg> for Msg {
    fn from(value: FrontendMsg) -> Self {
        Self::Frontend(value)
    }
}
impl From<ToolMessage> for Msg {
    fn from(value: ToolMessage) -> Self {
        Self::Tool(value)
    }
}
impl From<InputMessage> for Msg {
    fn from(value: InputMessage) -> Self {
        Self::Input(value)
    }
}
impl From<CmdMsg> for Msg {
    fn from(value: CmdMsg) -> Self {
        Self::Cmd(value)
    }
}

pub struct MsgResponder {
    que: VecDeque<Msg>,
    response_id: usize,
}
impl MsgResponder {
    fn from_msg(msg: Msg) -> Self {
        let mut que = VecDeque::<Msg>::with_capacity(4);
        que.push_back(msg);
        Self {
            que,
            response_id: MSG_ID_PROVIDER.fetch_add(1, Ordering::Relaxed),
        }
    }
    fn peek_top(&self) -> Option<&Msg> {
        self.que.get(0)
    }
    fn respond(&mut self, msg: impl Into<Msg>) {
        self.que.push_back(msg.into());
    }
    fn next(&mut self) -> Option<Msg> {
        self.que.pop_front()
    }
}

/// Entry point for a lot of the non-trivial interactivity of the system.
/// A lot of it requires exclusive world access and thus runs in a single thread.
///
/// * `world`:
pub fn sys_msg_handler(world: &mut World) {
    let _span = info_span!("sys_msg_handler").entered();

    let mut messages = {
        if let Some(mut events) = world.get_resource_mut::<Events<Msg>>() {
            events
                .drain()
                .map(MsgResponder::from_msg)
                .collect::<VecDeque<_>>()
        } else {
            warn!("WARN: Could not get messages to handle.  This should never happen but shouldn't cause issues");
            VecDeque::new()
        }
    };

    let mut iterations = 0;
    while let Some(mut msg_responder) = messages.pop_front() {
        let description = msg_responder
            .peek_top()
            .map(|msg| format!("Handling message: {:?}", msg));

        while let Some(msg) = msg_responder.next() {
            if iterations > 1000 {
                panic!("Too many messages in a single update.")
            }
            iterations += 1;

            match msg {
                Msg::Input(input_msg) => {
                    msg_handler_keybinds(world, &input_msg, &mut msg_responder)
                }
                Msg::Tool(tool_msg) => msg_handler_tool(world, &tool_msg, &mut msg_responder),
                Msg::Cmd(cmd_msg) => msg_handler_cmds(world, cmd_msg, &mut msg_responder),
                Msg::Frontend(frontend_msg) => {
                    let _span = info_span!("handle_frontend_msg").entered();

                    if let Some(editor_to_api_sender) =
                        world.get_resource_mut::<EditorToApiSender>()
                    {
                        if let Err(reason) = editor_to_api_sender.0.send(frontend_msg) {
                            panic!(
                                "Error sending message back to frontend. {:?} {:?}",
                                reason, reason.0
                            )
                        }
                    }
                }
            }
        }
    }
}
