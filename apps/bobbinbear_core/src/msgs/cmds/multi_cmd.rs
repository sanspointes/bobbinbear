use std::{fmt::Display, sync::Arc};

use bevy::prelude::World;

use crate::msgs::MsgQue;

use super::{Cmd, CmdError, CmdMsg, CmdType};

#[derive(Debug)]
pub struct MultiCmd {
    commands: Vec<Box<dyn Cmd>>,
}

impl From<MultiCmd> for CmdType {
    fn from(value: MultiCmd) -> Self {
        Self::Multi(value)
    }
}
impl From<MultiCmd> for CmdMsg {
    fn from(value: MultiCmd) -> Self {
        let cmd_type: CmdType = value.into();
        CmdMsg::Execute(Arc::new(cmd_type))
    }
}

impl Display for MultiCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "MultiCommand on {} commands:", self.commands.len())?;
        for cmd in self.commands.iter() {
            writeln!(f, "  - {}", cmd)?;
        }
        writeln!(f, "\n")
    }
}

impl MultiCmd {
    pub fn new(commands: Vec<Box<dyn Cmd>>) -> Self {
        Self { commands }
    }
}

impl Cmd for MultiCmd {
    fn execute(&mut self, world: &mut World, responder: &mut MsgQue) -> Result<(), CmdError> {
        for cmd in self.commands.iter_mut() {
            cmd.execute(world, responder)?;
        }
        Ok(())
    }

    fn undo(&mut self, world: &mut World, responder: &mut MsgQue) -> Result<(), CmdError> {
        for cmd in self.commands.iter_mut().rev() {
            cmd.undo(world, responder)?;
        }
        Ok(())
    }
}
