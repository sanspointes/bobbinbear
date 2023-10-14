pub mod add_remove_object_cmd;
pub mod multi_cmd;
pub mod update_bbvector_shape_cmd;

use std::{
    collections::VecDeque,
    error::Error,
    fmt::{Debug, Display},
    sync::{Arc, Mutex},
};

use bevy::prelude::*;

pub use multi_cmd::MultiCommand;
use thiserror::Error;

use super::Message;

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("This command is already undone. This may lead to a broken app state.")]
    DoubleUndo,
    #[error("This command is already executed.  This may lead to a broken app state.")]
    DoubleExecute,
    #[error("Command specific error. {0:?}")]
    CustomError(Box<dyn Error>),
}

/// Commands
///
/// Commands are atomic actions that can be undone / redone
///

pub trait Cmd: Send + Sync + Debug + Display {
    fn name(&self) -> &str;
    fn execute(&mut self, world: &mut World) -> Result<(), CmdError>;
    fn undo(&mut self, world: &mut World) -> Result<(), CmdError>;
}

#[derive(Event, Clone, Debug)]
pub enum CmdMsg {
    ExecuteCmd(Arc<Mutex<dyn Cmd>>),
    UndoCmd,
    RedoCmd,
}
impl CmdMsg {
    pub fn execute_from_cmd<C: Cmd + 'static>(cmd: C) -> Self {
        CmdMsg::ExecuteCmd(Arc::new(Mutex::new(cmd)))
    }
}

#[derive(Resource, Default)]
pub struct CmdResource {
    redo_stack: Vec<Arc<Mutex<dyn Cmd>>>,
    undo_stack: Vec<Arc<Mutex<dyn Cmd>>>,
}
pub struct CmdPlugin;

impl Plugin for CmdPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CmdResource::default());
    }
}

pub fn msg_handler_cmds(
    mut world: &mut World,
    message: &CmdMsg,
    _responses: &mut VecDeque<Message>,
) {
    match message {
        CmdMsg::ExecuteCmd(cmd) => {
            let mut unlocked_cmd = cmd.lock().unwrap();
            debug!("CmdMsg::ExecuteCmd -> {}", unlocked_cmd);
            let result = unlocked_cmd.execute(world);

            if let Err(reason) = result {
                error!("Failed to execute command {unlocked_cmd} with reason: \n - {reason:?}.");
            }

            let mut cmd_resource = world.resource_mut::<CmdResource>();
            cmd_resource.redo_stack.clear();
            cmd_resource.undo_stack.push(cmd.clone());
        }
        CmdMsg::UndoCmd => {
            let cmd = {
                let mut cmd_resource = world.resource_mut::<CmdResource>();
                cmd_resource.undo_stack.pop()
            };
            match cmd {
                Some(cmd) => {
                    let mut unlocked_cmd = cmd.lock().unwrap();
                    debug!("CmdMsg::UndoCmd -> {}", unlocked_cmd);
                    let result = unlocked_cmd.undo(world);

                    if let Err(reason) = result {
                        error!("Failed to undo command {unlocked_cmd} with reason: \n - {reason:?}.");
                    }

                    let mut cmd_resource = world.resource_mut::<CmdResource>();
                    cmd_resource.redo_stack.push(cmd.clone());
                }
                None => {
                    debug!("Nothing to undo.  TODO: Notify frontend.");
                }
            }
        }
        CmdMsg::RedoCmd => {
            let cmd = {
                let mut cmd_resource = world.resource_mut::<CmdResource>();
                cmd_resource.redo_stack.pop()
            };
            match cmd {
                Some(cmd) => {
                    let mut unlocked_cmd = cmd.lock().unwrap();
                    debug!("CmdMsg::RedoCmd -> {}", unlocked_cmd);
                    let result = unlocked_cmd.execute(world);

                    if let Err(reason) = result {
                        debug!("Failed to redo command {unlocked_cmd} with reason: \n - {reason:?}.");
                    }

                    let mut cmd_resource = world.resource_mut::<CmdResource>();
                    cmd_resource.undo_stack.push(cmd.clone());
                }
                None => {
                    debug!("Nothing to undo.  TODO: Notify frontend.");
                }
            }
        }
    }
}
