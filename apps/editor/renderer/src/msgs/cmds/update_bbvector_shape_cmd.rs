use std::{fmt::{Debug, Display}, mem};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{tess::path::Path as TessPath, Path};
use thiserror::Error;

use crate::components::bbid::{BBId, BBIdUtils};

use super::{Cmd, CmdError};

#[derive(Error, Debug)]
pub enum UpdateBBVectorShapeError {
    #[error("Cannot find entity {0:?}.")]
    CantFindEntity(Entity),
    #[error("Cannot find entity via bbid {0:?}.")]
    CantFindTarget(BBId),
}

impl From<UpdateBBVectorShapeError> for CmdError {
    fn from(value: UpdateBBVectorShapeError) -> Self {
        CmdError::CustomError(Box::new(value))
    }
}

pub struct UpdatePathComponentCmd {
    name: String,
    target_bbid: BBId,
    path: TessPath,
}

impl Display for UpdatePathComponentCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UpdatePathComponentCmd on {}",
            self.target_bbid,
        )
    }
}

impl Debug for UpdatePathComponentCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddObjectCmd")
            .field("target_bbid", &self.target_bbid)
            .field("path", &self.path)
            .finish()
    }
}

impl UpdatePathComponentCmd {
    pub fn new(target_bbid: BBId, path: TessPath) -> Self {
        Self {
            name: format!("Update path on \"{}\"", target_bbid),
            target_bbid,
            path,
        }
    }

    fn swap_path(
        &mut self,
        world: &mut World,
        target_bbid: BBId,
    ) -> Result<(), UpdateBBVectorShapeError> {
        let target_entity = world
            .get_entity_id_by_bbid(target_bbid)
            .ok_or(UpdateBBVectorShapeError::CantFindTarget(self.target_bbid))?;

        let mut path = world
            .query::<&mut Path>()
            .get_mut(world, target_entity)
            .map_err(|_| UpdateBBVectorShapeError::CantFindEntity(target_entity))?;

        mem::swap(&mut path.0, &mut self.path);
        path.set_changed();

        return Ok(())
    }
}

impl Cmd for UpdatePathComponentCmd {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute(&mut self, world: &mut bevy::prelude::World) -> Result<(), CmdError> {
        return self
            .swap_path(world, self.target_bbid)
            .map_err(|e| CmdError::from(e));
    }
    fn undo(&mut self, world: &mut bevy::prelude::World) -> Result<(), CmdError> {
        return self
            .swap_path(world, self.target_bbid)
            .map_err(|e| CmdError::from(e));
    }
}
