
use bevy::prelude::Event;

use crate::types::{BBCursor, BBTool};

#[derive(Event, Clone, Debug)]
pub enum FrontendMsg {
    SetCursor(BBCursor),
    SetCurrentTool(BBTool),
}
