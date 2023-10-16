pub mod add_remove_object_cmd;
pub mod multi_cmd;
pub mod update_path_cmd;
pub mod move_objects_cmd;

use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
    sync::Arc,
};

use bevy::prelude::*;

pub use multi_cmd::MultiCmd;
use thiserror::Error;

use self::{add_remove_object_cmd::AddObjectCmd, move_objects_cmd::MoveObjectsCmd};
use self::update_path_cmd::UpdatePathCmd;

use super::Message;

/// Shared Logic

// Whenever we run a new command we compare it with the old command via the `update_from_other`
// method in the command trait.  The command will either update itself from the other's data
// (if they are same type/repeatable) or flag the other command as a seperate command.
pub enum CmdUpdateTreatment {
    // Prev command is a repeat of next command, flags that command has updated self and can run.  
    // Overwrite prev in update que.
    AsRepeat,
    // Prev command is seperate from next command, push next command to update que.
    AsSeperate,
}

///
/// Commands are atomic actions that can be undone / redone
///
pub trait Cmd: Send + Sync + Debug + Display {
    fn execute(&mut self, world: &mut World) -> Result<(), CmdError>;
    fn undo(&mut self, world: &mut World) -> Result<(), CmdError>;
    /// Tries to update self using data from prev, if successful (CmdUpdateTreatment::AsRepeat)
    /// it will update itself with data from the previous command.
    fn try_update_from_prev(&mut self, _other: &CmdType) -> CmdUpdateTreatment {
        CmdUpdateTreatment::AsSeperate
    }
}

// Command message / history logic. 

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("This command is already undone. This may lead to a broken app state.")]
    DoubleUndo,
    #[error("This command is already executed.  This may lead to a broken app state.")]
    DoubleExecute,
    #[error("Command specific error. {0:?}")]
    ExecutionError(anyhow::Error),
}
impl From<anyhow::Error> for CmdError {
    fn from(value: anyhow::Error) -> Self {
        Self::ExecutionError(value)
    }
}

#[derive(Debug)]
pub enum CmdType {
    Multi(MultiCmd),
    AddObject(AddObjectCmd),
    UpdatePath(UpdatePathCmd),
    MoveObjects(MoveObjectsCmd),
}

macro_rules! unwrap_cmd_type {
    ($value:expr, $pattern:pat => $result:expr) => {
        match $value {
            CmdType::Multi($pattern) => $result,
            CmdType::AddObject($pattern) => $result,
            CmdType::UpdatePath($pattern) => $result,
            CmdType::MoveObjects($pattern) => $result,
        }
    };
}

#[derive(Event, Clone, Debug)]
pub enum CmdMsg {
    // Executes a command adding it to the undo stack.
    Execute(Arc<CmdType>),
    // Undoes a command
    Undo,
    // Redoes a command
    Redo,
    // Flags that the previous command cannot be repeated. (i.e. last command has finished
    // repeating)
    DisallowRepeated,
}

#[derive(Default, Copy, Clone)]
enum RepeatOverride {
    #[default]
    Default,
    Prevent,
}

#[derive(Resource, Default)]
pub struct CmdResource {
    repeat_behaviour: RepeatOverride,
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
    let _span = info_span!("sys_handler_cmds").entered();

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
                let mut cmd_resource = world.resource_mut::<CmdResource>();
                let treatment = match (cmd_resource.repeat_behaviour, cmd_resource.undo_stack.last()) {
                    (RepeatOverride::Default, Some(prev)) => cmd.try_update_from_prev(prev),
                    (RepeatOverride::Prevent, Some(_)) => CmdUpdateTreatment::AsSeperate,
                    (_, None) => CmdUpdateTreatment::AsSeperate,
                };
                cmd_resource.repeat_behaviour = RepeatOverride::Default;

                #[cfg(feature = "debug_cmd")]
                if matches!(treatment, CmdUpdateTreatment::AsRepeat) {
                    debug!("Command updated from previous. {cmd:?}");
                }

                let execution_result = cmd.execute(world);
                if let Err(reason) = execution_result {
                    error!("Failed to execute command {cmd:?} with reason: \n - {reason:?}.");
                    return;
                }

                let mut cmd_resource = world.resource_mut::<CmdResource>();
                // Repeated commands remove the previous command from the undo stack.
                if matches!(treatment, CmdUpdateTreatment::AsRepeat) {
                    cmd_resource.undo_stack.pop();
                }
                cmd_resource.undo_stack.push(cmd.into());
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
        CmdMsg::DisallowRepeated => {
            let mut cmd_resource = world.resource_mut::<CmdResource>();
            cmd_resource.repeat_behaviour = RepeatOverride::Prevent;
        }
    }
}
