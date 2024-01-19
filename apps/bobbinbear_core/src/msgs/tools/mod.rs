mod box_tool;
mod grab_tool;
mod noop_tool;
mod pen_tool;
mod select_tool;

use anyhow::anyhow;
use bevy::{ecs::system::SystemState, prelude::*};
use thiserror::Error;

use crate::{
    plugins::{input_plugin::InputMessage, screen_space_root_plugin::sys_setup_ss_root},
    types::BBTool, shared::sys_setup_cached_meshes,
};

use self::{
    box_tool::{BoxTool, BoxToolRes},
    grab_tool::{GrabTool, GrabToolState},
    pen_tool::{sys_setup_pen_resource, PenResource, PenTool},
    select_tool::{SelectFsm, SelectTool},
};

use super::{api::ApiEffectMsg, effect::EffectMsg, MsgQue};

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
    pub fn get_current_tool(&self) -> BBTool {
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
            .insert_resource(GrabToolState::default())
            .insert_resource(PenResource::default());

        app.add_systems(PostStartup, sys_setup_pen_resource.after(sys_setup_ss_root).after(sys_setup_cached_meshes));
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
        BBTool::Select => SelectTool::handle_msg(world, &msg, responder),
        BBTool::Grab => GrabTool::handle_msg(world, &msg, responder),
        BBTool::Box => BoxTool::handle_msg(world, &msg, responder),
        BBTool::Pen => PenTool::handle_msg(world, &msg, responder),
        BBTool::Noop => (),
    }
    let msg = ToolHandlerMessage::OnActivate;
    match curr_tool {
        BBTool::Select => SelectTool::handle_msg(world, &msg, responder),
        BBTool::Grab => GrabTool::handle_msg(world, &msg, responder),
        BBTool::Box => BoxTool::handle_msg(world, &msg, responder),
        BBTool::Pen => PenTool::handle_msg(world, &msg, responder),
        BBTool::Noop => (),
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
            if let Ok(msg) = &tool_message.try_into() {
                match res.get_current_tool() {
                    BBTool::Select => SelectTool::handle_msg(world, msg, responder),
                    BBTool::Grab => GrabTool::handle_msg(world, msg, responder),
                    BBTool::Box => BoxTool::handle_msg(world, msg, responder),
                    BBTool::Pen => PenTool::handle_msg(world, msg, responder),
                    BBTool::Noop => (),
                }
            } else {
                warn!("Warning: Unhandled ToolMessage ({:?}).  Cannot convert to ToolHandlerMessage to pass to active tool.", tool_message);
            }
        }
    }
}

pub trait ToolHandler {
    /// Executes when setting up the tool handler.
    ///
    /// * `world`:
    fn setup(world: &mut World);

    /// Handles messages such as commands or actions
    ///
    /// * `world`:
    /// * `msg`:
    /// * `responder`:
    fn handle_msg(world: &mut World, msg: &ToolHandlerMessage, responder: &mut MsgQue);

    /// Handles events (side effects of executed actions)
    ///
    /// * `world`:
    /// * `msg`:
    /// * `responder`:
    fn handle_effects(world: &mut World, event: &EffectMsg);
}

pub fn msg_handler_effect_for_tools(world: &mut World, msg: &EffectMsg, _responder: &mut MsgQue) {
    let tool_resource = world.resource::<ToolResource>();
    let tool = tool_resource.get_current_tool();

    match tool {
        BBTool::Noop => (),
        BBTool::Grab => GrabTool::handle_effects(world, msg),
        BBTool::Select => SelectTool::handle_effects(world, msg),
        BBTool::Box => BoxTool::handle_effects(world, msg),
        BBTool::Pen => PenTool::handle_effects(world, msg),
    }
}
