mod select_tool;
mod grab_tool;
mod box_tool;

use std::collections::VecDeque;

use bevy::{prelude::*, ecs::system::SystemState};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{editor2::{frontend::FrontendMessage, input::InputMessage, msgs::Message}, debug_log};

use self::{select_tool::{handle_select_tool_message, SelectToolRes}, grab_tool::handle_grab_tool_message, box_tool::handle_box_tool_message};

#[derive(Clone, Debug)]
pub enum ToolMessage {
    OnActivate(Tool),
    OnDeactivate(Tool),

    Input(InputMessage),

    SwitchTool(Tool),
    PushTool(Tool),
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
            ToolMessage::Input(input_message) => Ok(ToolHandlerMessage::Input(input_message.clone())),
            ToolMessage::OnActivate(_) => Ok(ToolHandlerMessage::OnActivate),
            ToolMessage::OnDeactivate(_) => Ok(ToolHandlerMessage::OnDeactivate),
            _ => Err(format!("ToolHandlerMessage does not have an equivalent enum variant for {:?}.", value)),
        }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    States,
    Hash, /*, specta::Type */
)]

#[wasm_bindgen(js_name = "Tool")]
pub enum Tool {
    #[default]
    Select,
    Grab,
    Box,
}
#[derive(Resource)]
pub struct ToolResource {
    old_current_tool: Tool,
    current_tool_stack: Vec<Tool>,
}
impl Default for ToolResource {
    fn default() -> Self {
        Self {
            old_current_tool: Tool::Select,
            current_tool_stack: vec![Tool::Select],
        }
    }
}
impl ToolResource {
    fn get_current_tool(&self) -> Tool {
        *self.current_tool_stack.last().unwrap_or(&Tool::Select)
    }

    fn generate_frontend_message(
        &mut self,
        responses: &mut VecDeque<Message>,
    ) {
        let new_current_tool = self.get_current_tool();

        // If the current tool has changed, pass lifecycle events to the tool sub_handlers
        // so they can load / unload their required state.
        if new_current_tool != self.old_current_tool {
            debug_log!("\tSwitching tool from {:?} to {:?}", self.old_current_tool, new_current_tool);
            responses.push_back(ToolMessage::OnDeactivate(self.old_current_tool.clone()).into());
            responses.push_back(ToolMessage::OnActivate(new_current_tool.clone()).into());
        }

        responses.push_back(FrontendMessage::SetCurrentTool(new_current_tool).into());

        self.old_current_tool = new_current_tool;
    }
}

pub struct ToolControllerPlugin;

impl Plugin for ToolControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<Tool>()
            .insert_resource(ToolResource::default())
            .insert_resource(SelectToolRes::default())
        ;
    }
}

pub fn handle_tool_message(world: &mut World, message: &ToolMessage, responses: &mut VecDeque<Message>) {

    let mut tool_sys_state = SystemState::<(
        // Cur tool
        ResMut<NextState<Tool>>,
        // Tool Resource
        ResMut<ToolResource>,

    )>::new(world);

    let (mut next_tool, mut res) = tool_sys_state.get_mut(world);
    match message {
        ToolMessage::PushTool(tool) => {
            trace!("ToolMessage::PushTool -> {:?}", tool);
            if *tool != res.get_current_tool() {
                res.current_tool_stack.push(*tool);
                next_tool.set(res.get_current_tool());
                res.generate_frontend_message(responses);
            }
        }
        ToolMessage::SwitchTool(tool) => {
            trace!("ToolMessage::SwitchTool -> {:?}", tool);
            if let Some(first) = res.current_tool_stack.first_mut() {
                *first = *tool;
            } else {
                res.current_tool_stack.push(*tool);
            }
            next_tool.set(res.get_current_tool());
            res.generate_frontend_message(responses);
        }
        ToolMessage::ResetToRootTool => {
            trace!("ToolMessage::ResetToRootTool (current_tool_stack: {:?})", res.current_tool_stack);
            if let Some(first) = res.current_tool_stack.first() {
                res.current_tool_stack = vec![*first];
                next_tool.set(res.get_current_tool());
                res.generate_frontend_message(responses);
            }
        }
        tool_message => {
            if let Ok(tool_handler_message) = &tool_message.try_into() {
                match res.get_current_tool() {
                    Tool::Select => handle_select_tool_message(world, tool_handler_message, responses),
                    Tool::Grab => handle_grab_tool_message(world, tool_handler_message, responses),
                    Tool::Box => handle_box_tool_message(world, tool_handler_message, responses),
                }
            } else {
                warn!("Warning: Unhandled ToolMessage ({:?}).  Cannot convert to ToolHandlerMessage to pass to active tool.", tool_message);
            }
        }
    }
}
