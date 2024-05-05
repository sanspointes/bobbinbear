use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Default, Clone, Copy, PartialEq)]
#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum BobbinTool {
    Noop,
    #[default]
    Select,
}
