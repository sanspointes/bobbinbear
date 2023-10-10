use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

use crate::editor2::msgs::Tool;

#[derive(Clone, Debug, Serialize, Deserialize /*, specta::Type */)]
pub enum Cursors {
    Default,
    Grab,
    Grabbing,
    Box,
    Pen,
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
