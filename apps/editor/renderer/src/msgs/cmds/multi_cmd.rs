use std::{fmt::Display, sync::Arc};

use bevy::prelude::World;

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
        CmdMsg::ExecuteCmd(Arc::new(cmd_type))
    }
}

impl Display for MultiCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MultiCommand on {} commands: \n", self.commands.len())?;
        for cmd in self.commands.iter() {
            write!(f, "  - {} \n", cmd)?;
        }
        write!(f, "\n")
    }
}

impl MultiCmd {
    pub fn new(commands: Vec<Box<dyn Cmd>>) -> Self {
        Self { commands }
    }
}

impl Cmd for MultiCmd {
    fn name(&self) -> &str {
        "Multi Commands"
    }
    fn execute(&mut self, world: &mut World) -> Result<(), CmdError> {
        for cmd in self.commands.iter_mut() {
            cmd.execute(world)?;
        }
        Ok(())
    }

    fn undo(&mut self, world: &mut World) -> Result<(), CmdError> {
        for cmd in self.commands.iter_mut() {
            cmd.undo(world)?;
        }
        Ok(())
    }
}
