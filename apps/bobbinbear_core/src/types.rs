use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(
    Serialize, Deserialize, tsify::Tsify, Debug, Clone, Default, States, Eq, PartialEq, Hash, Copy,
)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum BBTool {
    #[default]
    Noop,
    Select,
    Grab,
    Box,
}

#[derive(Serialize, Deserialize, tsify::Tsify, Debug, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum BBCursor {
    Default,
    Pointer,
    Grab,
    Grabbing,
    Box,
}

#[macro_export]
macro_rules! debug_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => {
        #[cfg(not(target_arch = "wasm32"))]
        {
            println!($($t)*);
        }

        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::debug_1(&format!($($t)*).into())
        }
    }
}
