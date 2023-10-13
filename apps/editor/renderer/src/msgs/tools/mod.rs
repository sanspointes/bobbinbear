mod grab_tool;
mod select_tool;
mod box_tool;

use std::collections::VecDeque;

use bevy::{ecs::system::SystemState, prelude::*};

use crate::{types::BBTool, plugins::input_plugin::InputMessage};

use self::{
    // box_tool::msg_handler_box_tool,
    grab_tool::msg_handler_grab_tool,
    select_tool::{msg_handler_select_tool, SelectToolRes}, box_tool::{msg_handler_box_tool, BoxToolResource},
};

use super::{frontend::FrontendMsg, Message};

#[derive(Clone, Debug)]
pub enum ToolMessage {
    OnActivate(BBTool),
    OnDeactivate(BBTool),

    Input(InputMessage),

    SwitchTool(BBTool),
    PushTool(BBTool),
    ResetToRootTool,
}

#[derive(Debug)]
pub enum ToolHandlerMessage {
    OnActivate,
    OnDeactivate,

    // For keyboard, mouse input
    Input(InputMessage),
}

impl TryFrom<&ToolMessage> for ToolHandlerMessage {
    type Error = String;
    fn try_from(value: &ToolMessage) -> Result<Self, Self::Error> {
        match value {
            ToolMessage::Input(input_message) => {
                Ok(ToolHandlerMessage::Input(input_message.clone()))
            }
            ToolMessage::OnActivate(_) => Ok(ToolHandlerMessage::OnActivate),
            ToolMessage::OnDeactivate(_) => Ok(ToolHandlerMessage::OnDeactivate),
            _ => Err(format!(
                "ToolHandlerMessage does not have an equivalent enum variant for {:?}.",
                value
            )),
        }
    }
}

#[derive(Default, Resource)]
pub struct ToolResource {
    base_tool: BBTool,
    tool_stack: Vec<BBTool>,
}
impl ToolResource {
    fn get_current_tool(&self) -> BBTool {
        *self.tool_stack.last().unwrap_or(&BBTool::Select)
    }

    fn generate_frontend_message(&mut self, responses: &mut VecDeque<Message>) {
        let new_current_tool = self.get_current_tool();

        // If the current tool has changed, pass lifecycle events to the tool sub_handlers
        // so they can load / unload their required state.
        if new_current_tool != self.base_tool {
            responses.push_back(ToolMessage::OnDeactivate(self.base_tool.clone()).into());
            responses.push_back(ToolMessage::OnActivate(new_current_tool.clone()).into());
        }

        responses.push_back(FrontendMsg::SetCurrentTool(new_current_tool).into());

        self.base_tool = new_current_tool;
    }
}

pub struct ToolControllerPlugin;

impl Plugin for ToolControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<BBTool>()
            .insert_resource(ToolResource::default())
            .insert_resource(SelectToolRes::default())
            .insert_resource(BoxToolResource::default())
        ;
    }
}

pub fn msg_handler_tool(
    world: &mut World,
    message: &ToolMessage,
    responses: &mut VecDeque<Message>,
) {
    let mut tool_sys_state = SystemState::<(
        // Cur tool
        ResMut<NextState<BBTool>>,
        // Tool Resource
        ResMut<ToolResource>,
    )>::new(world);

    let (mut next_tool, mut res) = tool_sys_state.get_mut(world);
    match message {
        ToolMessage::PushTool(tool) => {
            trace!("ToolMessage::PushTool -> {:?}", tool);
            if *tool != res.get_current_tool() {
                res.tool_stack.push(*tool);
                next_tool.set(res.get_current_tool());
                res.generate_frontend_message(responses);
            }
        }
        ToolMessage::SwitchTool(tool) => {
            trace!("ToolMessage::SwitchTool -> {:?}", tool);
            if let Some(first) = res.tool_stack.first_mut() {
                *first = *tool;
            } else {
                res.tool_stack.push(*tool);
            }
            next_tool.set(res.get_current_tool());
            res.generate_frontend_message(responses);
        }
        ToolMessage::ResetToRootTool => {
            trace!(
                "ToolMessage::ResetToRootTool (current_tool_stack: {:?})",
                res.tool_stack
            );
            if let Some(first) = res.tool_stack.first() {
                res.tool_stack = vec![*first];
                next_tool.set(res.get_current_tool());
                res.generate_frontend_message(responses);
            }
        }
        tool_message => {
            if let Ok(tool_handler_message) = &tool_message.try_into() {
                match res.get_current_tool() {
                    BBTool::Select => msg_handler_select_tool(world, tool_handler_message, responses),
                    BBTool::Grab => msg_handler_grab_tool(world, tool_handler_message, responses),
                    BBTool::Box => msg_handler_box_tool(world, tool_handler_message, responses),
                }
            } else {
                warn!("Warning: Unhandled ToolMessage ({:?}).  Cannot convert to ToolHandlerMessage to pass to active tool.", tool_message);
            }
        }
    }
}
