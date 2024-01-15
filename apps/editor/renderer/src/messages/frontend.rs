use bevy::prelude::Event;

use crate::types::{BBCursor, BBTool};

#[derive(Event)]
pub enum FrontendMsg {
    SetCursor(BBCursor),
    SetCurrentTool(BBTool),
}
