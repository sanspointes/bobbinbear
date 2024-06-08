use bevy::ecs::schedule::States;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(States, Hash, Eq, Default, Clone, Copy, PartialEq, Debug)]
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum BobbinTool {
    Noop,
    #[default]
    Select,
    Pen,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum BobbinCursor {
    #[default]
    Default,
    DefaultTap,
    Pointer,
    PointerTap,
    PointerMove,
    PenSplitEdge,
    PenSameEndpoint,
    Pen,
}
