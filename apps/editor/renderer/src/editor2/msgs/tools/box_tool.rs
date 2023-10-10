use std::{collections::VecDeque, matches, sync::Arc};

use bevy::prelude::*;

use crate::{
    debug_log,
    editor2::{frontend::FrontendMessage, Message},
    types::Cursors,
};

use super::ToolHandlerMessage;

pub fn handle_box_tool_message(
    world: &mut World,
    message: &ToolHandlerMessage,
    responses: &mut VecDeque<Message>,
) {
    match message {
        ToolHandlerMessage::OnActivate => {
            debug_log!("BoxTool::OnActivate");
            responses.push_back(FrontendMessage::SetCursor(Cursors::Box).into());
        }
        ToolHandlerMessage::OnDeactivate => {
            debug_log!("BoxTool::OnDeactivate");
        }
        ToolHandlerMessage::Input(input_message) => match input_message {
            _ => {}
        },
    }
}
