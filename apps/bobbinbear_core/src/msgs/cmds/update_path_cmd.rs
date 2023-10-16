use std::{fmt::{Debug, Display}, mem, sync::Arc};

use anyhow::anyhow;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{tess::path::Path as TessPath, Path};

use crate::components::bbid::{BBId, BBIdUtils};

use super::{Cmd, CmdError, CmdType, CmdMsg, CmdUpdateTreatment};

pub struct UpdatePathCmd {
    name: String,
    pub target_bbid: BBId,
    path: TessPath,
}
impl From<UpdatePathCmd> for CmdType {
    fn from(value: UpdatePathCmd) -> Self {
        Self::UpdatePath(value)
    }
}
impl From<UpdatePathCmd> for CmdMsg {
    fn from(value: UpdatePathCmd) -> Self {
        let cmd_type: CmdType = value.into();
        CmdMsg::Execute(Arc::new(cmd_type))
    }
}

impl Display for UpdatePathCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UpdatePathComponentCmd on {}",
            self.target_bbid,
        )
    }
}

impl Debug for UpdatePathCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdatePathCmd")
            .field("target_bbid", &self.target_bbid)
            .field("path", &self.path)
            .finish()
    }
}

impl UpdatePathCmd {
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
    ) -> Result<(), CmdError> {
        let target_entity = world
            .get_entity_id_by_bbid(target_bbid)
            .ok_or(anyhow!("Can't find entity entity with {target_bbid:?}."))?;

        let mut path = world
            .query::<&mut Path>()
            .get_mut(world, target_entity)
            .map_err(|err| anyhow!("Error getting path of {target_entity:?}.\n - Reason: {err:?}."))?;

        mem::swap(&mut path.0, &mut self.path);
        path.set_changed();

        Ok(())
    }
}

impl Cmd for UpdatePathCmd {
    fn execute(&mut self, world: &mut bevy::prelude::World) -> Result<(), CmdError> {
        self.swap_path(world, self.target_bbid)
    }
    fn undo(&mut self, world: &mut bevy::prelude::World) -> Result<(), CmdError> {
        self.swap_path(world, self.target_bbid)
    }

    fn try_update_from_prev(&mut self, other: &CmdType) -> super::CmdUpdateTreatment {
        match other {
            CmdType::UpdatePath(cmd) => {
                CmdUpdateTreatment::AsRepeat
            }
            _ => CmdUpdateTreatment::AsSeperate,
        }
        
    }
}
