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
}

pub enum BBCursor {
    Default,
    Pointer,
}
