use std::collections::VecDeque;

use js_sys::{Array as JsArray, Object as JsObject, Error as JsError};
use serde::{Serialize, Deserialize};
use tsify::Tsify;
use wasm_bindgen::{JsValue, convert::FromWasmAbi};

use crate::types::{BBCursor, BBTool};

use super::{ApiEffectMsg, ApiResponseMsg};

#[derive(Serialize, Deserialize, Tsify, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "tag", content = "value" )]
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
#[serde(tag = "tag", content = "value")]
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
#[serde(tag = "tag", content = "value" )]
pub enum JsApiMsg {
    Effect(JsApiEffectMsg),
    Response(VecDeque<JsApiResponseMsg>, usize),
}
impl TryFrom<JsApiMsg> for JsValue {
    type Error = JsValue;
    fn try_from(value: JsApiMsg) -> Result<Self, Self::Error> {
        let v = serde_wasm_bindgen::to_value(&value).map_err(|reason| {
            JsValue::from_str(&reason.to_string())
        });
        v
    }
}


impl From<ApiEffectMsg> for JsApiMsg {
    fn from(value: ApiEffectMsg) -> Self {
        Self::Effect(value.into())
    }
}
