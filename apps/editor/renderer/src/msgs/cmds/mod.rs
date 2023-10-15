pub mod add_remove_object_cmd;
pub mod multi_cmd;
pub mod update_path_cmd;

use std::{
    collections::VecDeque,
    error::Error,
    fmt::{Debug, Display},
    sync::Arc,
};

use bevy::prelude::*;

pub use multi_cmd::MultiCmd;
use thiserror::Error;

use self::add_remove_object_cmd::AddObjectCmd;
use self::update_path_cmd::UpdatePathCmd;

use super::Message;

/// Shared Logic

///
/// Commands are atomic actions that can be undone / redone
///
pub trait Cmd: Send + Sync + Debug + Display {
    fn name(&self) -> &str;
    fn execute(&mut self, world: &mut World) -> Result<(), CmdError>;
    fn undo(&mut self, world: &mut World) -> Result<(), CmdError>;
    fn is_repeated(&self, _other: &CmdType) -> bool {
        false
    }
}

// Command message / history logic. 

#[derive(Default)]
/// Sometimes we don't want to push a command to the undo/redo history.
enum CmdHistoryBehaviour {
    #[default]
    PushHistory,
    Ignore,
}

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("This command is already undone. This may lead to a broken app state.")]
    DoubleUndo,
    #[error("This command is already executed.  This may lead to a broken app state.")]
    DoubleExecute,
    #[error("Command specific error. {0:?}")]
    CustomError(Box<dyn Error>),
}

#[derive(Debug)]
pub enum CmdType {
    Multi(MultiCmd),
    AddObject(AddObjectCmd),
    UpdatePath(UpdatePathCmd),
}

macro_rules! unwrap_cmd_type {
    ($value:expr, $pattern:pat => $result:expr) => {
        match $value {
            CmdType::Multi($pattern) => $result,
            CmdType::AddObject($pattern) => $result,
            CmdType::UpdatePath($pattern) => $result,
        }
    };
}

#[derive(Event, Clone, Debug)]
pub enum CmdMsg {
    Execute(Arc<CmdType>),
    Undo,
    Redo,
}

#[derive(Resource, Default)]
pub struct CmdResource {
    redo_stack: Vec<CmdType>,
    undo_stack: Vec<CmdType>,
}
pub struct CmdMsgPlugin;

impl Plugin for CmdMsgPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CmdResource::default());
    }
}

pub fn msg_handler_cmds(
    world: &mut World,
    message: CmdMsg,
    _responses: &mut VecDeque<Message>,
) {
    match message {
        CmdMsg::Execute(cmd_wrapped) => {
            // TODO: Improve the unsound logic
            // Currently commands will not be acted upon if there are any strong references to them
            // throughout the app.
            //
            // We need to either:
            // - Refactor to avoid Arc<Mutex<CmdType>>
            let Some(cmd) = Arc::into_inner(cmd_wrapped) else {
                error!("Could not execute cmd. Could not take from Arc.");
                return;
            };

            unwrap_cmd_type!(cmd, mut cmd => {
                let execution_result = cmd.execute(world);
                if let Err(reason) = execution_result {
                    error!("Failed to execute command {cmd:?} with reason: \n - {reason:?}.");
                    return;
                }

                let mut cmd_resource = world.resource_mut::<CmdResource>();

                let cmd_history_behaviour = match cmd_resource.undo_stack.last() {
                    None => CmdHistoryBehaviour::PushHistory,
                    Some(prev) => match cmd.is_repeated(prev) {
                        true => CmdHistoryBehaviour::Ignore,
                        false => CmdHistoryBehaviour::PushHistory,
                    }
                };

                match cmd_history_behaviour {
                    CmdHistoryBehaviour::PushHistory => {
                        cmd_resource.undo_stack.push(cmd.into());
                    }
                    CmdHistoryBehaviour::Ignore => { },
                }

                cmd_resource.redo_stack.clear();

            });
        }
        CmdMsg::Undo => {
            let cmd = {
                let mut cmd_resource = world.resource_mut::<CmdResource>();
                cmd_resource.undo_stack.pop()
            };

            let Some(mut cmd) = cmd else {
                debug!("Nothing to undo.  TODO: Notify frontend.");
                return;
            };

            let execution_result = unwrap_cmd_type!(cmd, ref mut cmd => {
                cmd.undo(world)
            });

            if let Err(reason) = execution_result {
                error!("Failed to undo command {cmd:?} with reason: \n - {reason:?}.");
            }

            let mut cmd_resource = world.resource_mut::<CmdResource>();
            cmd_resource.redo_stack.push(cmd);
        }
        CmdMsg::Redo => {
            let cmd = {
                let mut cmd_resource = world.resource_mut::<CmdResource>();
                cmd_resource.redo_stack.pop()
            };

            let Some(mut cmd) = cmd else {
                debug!("Nothing to undo.  TODO: Notify frontend.");
                return;
            };

            let execution_result = unwrap_cmd_type!(cmd, ref mut cmd => {
                cmd.execute(world)
            });

            if let Err(reason) = execution_result {
                error!("Failed to undo command {cmd:?} with reason: \n - {reason:?}.");
            }

            let mut cmd_resource = world.resource_mut::<CmdResource>();
            cmd_resource.undo_stack.push(cmd);
        }
    }
}
