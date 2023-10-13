use bevy::prelude::*;

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
pub enum BBTool {
    #[default]
    Select,
    Grab,
    Box,
}

#[derive(Debug, Clone)]
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
