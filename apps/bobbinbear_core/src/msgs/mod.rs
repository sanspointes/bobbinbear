pub mod api;
pub mod cmds;
pub mod keybinds;
pub mod tools;

use std::{
    collections::VecDeque,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy::prelude::*;
use lazy_static::lazy_static;

use crate::{
    api::{ApiToEditorReceiver, EditorToApiSender},
    plugins::input_plugin::InputMessage,
};

pub use self::tools::{msg_handler_tool, ToolMessage, ToolMsgPlugin};
use self::{
    api::{JsApiMsg, JsApiResponseMsg, ApiResponseMsg, ApiEffectMsg},
    cmds::{msg_handler_cmds, CmdMsg},
    keybinds::msg_handler_keybinds,
};

#[derive(Event, Clone, Debug)]
pub enum Msg {
    // RawInput(RawInputMessage),
    Input(InputMessage),
    Tool(ToolMessage),
    Cmd(CmdMsg),
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

lazy_static! {
    static ref MSG_ID_PROVIDER: AtomicUsize = AtomicUsize::new(0);
}
// All messages are wrapped with an identifier to allow responses.
// * 0 : The Msg Object
// * 1 : Optional response ID (if the message is respondable)
pub struct MsgRespondable(pub Msg, pub usize);
impl MsgRespondable {
    pub fn new(msg: impl Into<Msg>) -> Self {
        Self(msg.into(), MSG_ID_PROVIDER.fetch_add(1, Ordering::Relaxed))
    }
}

/// This struct is used to que up messages to p
///
/// * `que`: Que of messages to perform, this can be pushed to by message handlers
/// * `response_id`: Optional response ID if it needs to respond to the EditorApi
pub struct MsgQue {
    que: VecDeque<Msg>,
    response_id: Option<usize>,
    responses: VecDeque<ApiResponseMsg>,
    effects: VecDeque<ApiEffectMsg>
}
impl From<Msg> for MsgQue {
    fn from(value: Msg) -> Self {
        let mut que = VecDeque::<Msg>::with_capacity(4);
        que.push_back(value);
        Self {
            que,
            response_id: None,
            responses: VecDeque::new(),
            effects: VecDeque::new(),
        }
    }
}
impl From<MsgRespondable> for MsgQue {
    fn from(value: MsgRespondable) -> Self {
        let mut que = VecDeque::<Msg>::with_capacity(4);
        que.push_back(value.0);
        Self {
            que,
            response_id: Some(value.1),
            responses: VecDeque::new(),
            effects: VecDeque::new(),
        }
    }
}
impl MsgQue {
    fn peek_top(&self) -> Option<&Msg> {
        self.que.get(0)
    }
    fn push_internal(&mut self, msg: impl Into<Msg>) {
        self.que.push_back(msg.into());
    }
    fn respond(&mut self, response: impl Into<ApiResponseMsg>) {
        self.responses.push_back(response.into());
    }
    fn notify_effect(&mut self, effect: impl Into<ApiEffectMsg>) {
        self.effects.push_back(effect.into());
    }
    fn next(&mut self) -> Option<Msg> {
        self.que.pop_front()
    }
    fn as_js_api_msgs(mut self) -> VecDeque<JsApiMsg> {
        let MsgQue { responses, effects, mut response_id, .. } = self;

        let mut js_api_msgs: VecDeque<JsApiMsg> = effects.into_iter().map(|v| v.into()).collect();
        if let Some(response_id) = response_id.take() {
            let mut responses: VecDeque<JsApiResponseMsg> = responses.into_iter().map(|v| v.into()).collect();
            if responses.is_empty() {
                responses.push_back(JsApiResponseMsg::Success);
            }
            let response_msg = JsApiMsg::Response(responses, response_id);
            js_api_msgs.push_back(response_msg);
        }

        js_api_msgs
    }
}

/// Entry point for a lot of the non-trivial interactivity of the system.
/// A lot of it requires exclusive world access and thus runs in a single thread.
///
/// * `world`:
pub fn sys_msg_handler(world: &mut World) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_msg_handler").entered();

    let mut all_messages: VecDeque<MsgQue> = {
        #[cfg(feature = "debug_trace")]
        let _span = info_span!("sys_msg_handler: Receiving messages").entered();
        // Messages sent from API have a unique ID that we can use to respond to the API call with.
        let receiver = world.resource_mut::<ApiToEditorReceiver>();
        let mut responders: VecDeque<MsgQue> = receiver.0.try_iter().map(MsgQue::from).collect();

        // These messages are not respondable.
        if let Some(mut events) = world.get_resource_mut::<Events<Msg>>() {
            for msg in events.drain() {
                responders.push_back(MsgQue::from(msg))
            }
        } else {
            warn!("WARN: Could not get messages to handle.  This should never happen but shouldn't cause issues");
        }

        responders
    };

    let mut iterations = 0;
    while let Some(mut msg_responder) = all_messages.pop_front() {
        let source_msg_description = msg_responder
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
            }
        }

        #[cfg(feature = "debug_trace")]
        let _span = info_span!("sys_msg_handler: Handing responses/effects").entered();
        // Respond to API + send side effects back to UI layer
        let sender = world.resource_mut::<EditorToApiSender>();
        let to_send = msg_responder.as_js_api_msgs();
        for msg in to_send {
            info!("sys_msg_handler: Sending msg {msg:?}.");
            sender.0.send(msg).expect("sys_msg_handler: Error sending effect for message call.");
        }
    }
}
