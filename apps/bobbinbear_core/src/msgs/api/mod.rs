mod serialised;

use std::sync::Arc;

use bevy::prelude::Event;

use crate::types::{BBCursor, BBTool};

pub use serialised::{JsApiMsg, JsApiResponseMsg, JsApiEffectMsg};

#[derive(Event, Clone, Debug)]
// Message type for sending responses to API calls.  It will respond to the API call with the
// response id from the `MsgQue` stored in `sys_msg_handler`.
pub enum ApiResponseMsg {
    Success,
    Err(Arc<anyhow::Error>),
}
impl From<anyhow::Error> for ApiResponseMsg {
    fn from(value: anyhow::Error) -> Self {
        Self::Err(Arc::new(value))
    }
}

#[derive(Event, Clone, Debug)]
/// Message type for sending side effects back to the JS/UI layer
pub enum ApiEffectMsg {
    SetCursor(BBCursor),
    SetCurrentTool(BBTool),
}

#[derive(Event, Clone, Debug)]
/// Internal type for sending data back to the JS/UI Layer and responding to API Calls.
/// This type will be converted to a JsApiResponseMsg.
pub enum ApiMsg {
    // Internals will send a response
    Response(ApiResponseMsg),
    // Api will receive an effect 
    Effect(ApiEffectMsg),
}

impl From<ApiEffectMsg> for ApiMsg {
    fn from(value: ApiEffectMsg) -> Self {
        Self::Effect(value)
    }
}

impl From<ApiResponseMsg> for ApiMsg {
    fn from(value: ApiResponseMsg) -> Self {
        Self::Response(value)
    }
}
