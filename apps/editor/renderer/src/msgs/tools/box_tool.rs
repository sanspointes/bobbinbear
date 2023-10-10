use std::{collections::VecDeque, matches, sync::Arc};

use bevy::prelude::*;

use crate::{
    debug_log,
    types::BBCursor, msgs::{Message, frontend::FrontendMsg},
};

use super::ToolHandlerMessage;

pub fn msg_handler_box_tool(
    world: &mut World,
    message: &ToolHandlerMessage,
    responses: &mut VecDeque<Message>,
) {
    match message {
        ToolHandlerMessage::OnActivate => {
            debug_log!("BoxTool::OnActivate");
            responses.push_back(FrontendMsg::SetCursor(BBCursor::Box).into());
        }
        ToolHandlerMessage::OnDeactivate => {
            debug_log!("BoxTool::OnDeactivate");
        }
        ToolHandlerMessage::Input(input_message) => match input_message {
            _ => {}
        },
    }
}
