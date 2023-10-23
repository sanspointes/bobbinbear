mod box_tool;
mod grab_tool;
mod select_tool;

use anyhow::anyhow;
use bevy::{ecs::system::SystemState, prelude::*};
use thiserror::Error;

use crate::{plugins::input_plugin::InputMessage, types::BBTool};

use self::{
    box_tool::{msg_handler_box_tool, BoxToolRes},
    // box_tool::msg_handler_box_tool,
    grab_tool::{msg_handler_grab_tool, GrabToolState},
    select_tool::{msg_handler_select_tool, SelectFsm},
};

use super::{api::ApiEffectMsg, MsgQue};

#[derive(Error, Debug)]
pub enum ToolFsmError {
    #[error("No transition for event")]
    NoTransition,
    #[error("Unknown error during transtion: {0:?}")]
    TransitionError(anyhow::Error),
}

pub type ToolFsmResult<T> = Result<(T, T), ToolFsmError>;

#[derive(Clone, Debug)]
pub enum ToolMessage {
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
    type Error = anyhow::Error;
    fn try_from(value: &ToolMessage) -> anyhow::Result<Self> {
        match value {
            ToolMessage::Input(input_message) => Ok(ToolHandlerMessage::Input(*input_message)),
            _ => Err(anyhow!(
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
}

pub struct ToolMsgPlugin;

impl Plugin for ToolMsgPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<BBTool>()
            .insert_resource(ToolResource::default())
            .insert_resource(SelectFsm::default())
            .insert_resource(BoxToolRes::default())
            .insert_resource(GrabToolState::default());
    }
}

fn handle_active_tool_change(
    world: &mut World,
    responder: &mut MsgQue,
    prev_tool: BBTool,
    curr_tool: BBTool,
) {
    let msg = ToolHandlerMessage::OnDeactivate;
    match prev_tool {
        BBTool::Select => msg_handler_select_tool(world, &msg, responder),
        BBTool::Grab => msg_handler_grab_tool(world, &msg, responder),
        BBTool::Box => msg_handler_box_tool(world, &msg, responder),
    }
    let msg = ToolHandlerMessage::OnActivate;
    match curr_tool {
        BBTool::Select => msg_handler_select_tool(world, &msg, responder),
        BBTool::Grab => msg_handler_grab_tool(world, &msg, responder),
        BBTool::Box => msg_handler_box_tool(world, &msg, responder),
    }
    responder.notify_effect(ApiEffectMsg::SetCurrentTool(curr_tool));
}

pub fn msg_handler_tool(world: &mut World, message: &ToolMessage, responder: &mut MsgQue) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_handler_tool").entered();

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
            let prev = res.get_current_tool();
            if *tool != prev {
                res.tool_stack.push(*tool);
                next_tool.set(res.get_current_tool());
                handle_active_tool_change(world, responder, prev, *tool);
            }
        }
        ToolMessage::SwitchTool(tool) => {
            trace!("ToolMessage::SwitchTool -> {:?}", tool);
            let prev = res.get_current_tool();
            if let Some(first) = res.tool_stack.first_mut() {
                *first = *tool;
            } else {
                res.tool_stack.push(*tool);
            }
            next_tool.set(res.get_current_tool());
            if prev != *tool {
                handle_active_tool_change(world, responder, prev, *tool);
            }
        }
        ToolMessage::ResetToRootTool => {
            trace!(
                "ToolMessage::ResetToRootTool (current_tool_stack: {:?})",
                res.tool_stack
            );
            let prev = res.get_current_tool();
            if let Some(first) = res.tool_stack.first().cloned() {
                res.tool_stack = vec![first];
                next_tool.set(res.get_current_tool());
                handle_active_tool_change(world, responder, prev, first);
            }
        }
        tool_message => {
            if let Ok(tool_handler_message) = &tool_message.try_into() {
                match res.get_current_tool() {
                    BBTool::Select => {
                        msg_handler_select_tool(world, tool_handler_message, responder)
                    }
                    BBTool::Grab => msg_handler_grab_tool(world, tool_handler_message, responder),
                    BBTool::Box => msg_handler_box_tool(world, tool_handler_message, responder),
                }
            } else {
                warn!("Warning: Unhandled ToolMessage ({:?}).  Cannot convert to ToolHandlerMessage to pass to active tool.", tool_message);
            }
        }
    }
}
