use crate::types::{BBCursor, BBTool};

pub enum FrontendMsg {
    SetCursor(BBCursor),
    SetCurrentTool(BBTool),
}
