use std::fmt::Display;

use bevy::prelude::World;

use super::{Cmd, CmdError};

#[derive(Debug)]
pub struct MultiCommand {
    commands: Vec<Box<dyn Cmd>>,
}

impl Display for MultiCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MultiCommand on {} commands: \n", self.commands.len())?;
        for cmd in self.commands.iter() {
            write!(f, "  - {} \n", cmd)?;
        }
        write!(f, "\n")
    }
}

impl MultiCommand {
    pub fn new(commands: Vec<Box<dyn Cmd>>) -> Self {
        Self {
            commands,
        }
    }
}

impl Cmd for MultiCommand {
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
