use std::collections::VecDeque;

use serde::{Serialize, Deserialize};
use tsify::Tsify;

use crate::types::{BBCursor, BBTool};

use super::{ApiEffectMsg, ApiResponseMsg};

#[derive(Serialize, Deserialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum JsApiEffectMsg {
    SetCursor(BBCursor),
    SetCurrentTool(BBTool),
}
impl From<ApiEffectMsg> for JsApiEffectMsg {
    fn from(value: ApiEffectMsg) -> Self {
        match value {
            ApiEffectMsg::SetCursor(cursor) => Self::SetCursor(cursor),
            ApiEffectMsg::SetCurrentTool(tool) => Self::SetCurrentTool(tool),
        }
    }
}

#[derive(Serialize, Deserialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum JsApiResponseMsg {
    Success,
    Error(String),
}

impl From<ApiResponseMsg> for JsApiResponseMsg {
    fn from(value: ApiResponseMsg) -> Self {
        match value {
            ApiResponseMsg::Success => Self::Success,
            ApiResponseMsg::Err(reason) => Self::Error(reason.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum JsApiMsg {
    Effect(JsApiEffectMsg),
    Response(VecDeque<JsApiResponseMsg>, usize),
}

impl From<ApiEffectMsg> for JsApiMsg {
    fn from(value: ApiEffectMsg) -> Self {
        Self::Effect(value.into())
    }
}
