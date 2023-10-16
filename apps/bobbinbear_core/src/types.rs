use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    // Serialize,
    // Deserialize,
    PartialEq,
    Eq,
    States,
    Hash, /*, specta::Type */
)]
#[wasm_bindgen()]
pub enum BBTool {
    #[default]
    Select,
    Grab,
    Box,
}

#[derive(Debug, Clone)]
#[wasm_bindgen()]
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
