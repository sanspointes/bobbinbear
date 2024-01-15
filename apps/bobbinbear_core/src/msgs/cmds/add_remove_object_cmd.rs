use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use anyhow::anyhow;
use bevy::prelude::*;

use crate::{
    components::bbid::{BBId, BBIdUtils},
    serialisation::SerialisableEntity,
};

use super::{Cmd, CmdError, CmdMsg, CmdType};

pub struct AddObjectCmd {
    entity_bbid: BBId,
    parent: Option<BBId>,
    serialised: Option<SerialisableEntity>,
}
impl From<AddObjectCmd> for CmdType {
    fn from(value: AddObjectCmd) -> Self {
        Self::AddObject(value)
    }
}
impl From<AddObjectCmd> for CmdMsg {
    fn from(value: AddObjectCmd) -> Self {
        let cmd_type: CmdType = value.into();
        CmdMsg::Execute(Arc::new(cmd_type))
    }
}

impl Display for AddObjectCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parent_string = match self.parent {
            Some(parent) => parent.to_string(),
            None => String::from("Scene Root"),
        };
        write!(
            f,
            "Adding object with BBID {} to {}",
            self.entity_bbid, parent_string
        )
    }
}

impl Debug for AddObjectCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AddObjectCmd")
            .field("entity_bbid", &self.entity_bbid)
            .field("parent", &self.parent)
            .field(
                "scene",
                &self
                    .serialised
                    .as_ref()
                    .map(|_| String::from("Dynamic Scene")),
            )
            .finish()
    }
}

impl AddObjectCmd {
    pub fn from_builder<F: FnMut(&mut EntityWorldMut<'_>)>(
        world: &mut World,
        parent: Option<BBId>,
        mut builder: F,
    ) -> anyhow::Result<Self> {
        let mut entity_mut = world.spawn_empty();
        builder(&mut entity_mut);

        let id = entity_mut.id();
        AddObjectCmd::from_entity(world, id, parent)
    }

    pub fn from_entity(
        world: &mut World,
        entity: Entity,
        parent: Option<BBId>,
    ) -> anyhow::Result<Self> {
        let serialised = SerialisableEntity::from_entity_recursive(world, entity).ok_or(
            CmdError::from(anyhow!("Can't get SerialisableEntity for {entity:?}")),
        )?;

        let entity_bbid = world
            .get::<BBId>(entity)
            .cloned()
            .ok_or(CmdError::from(anyhow!("Cant get bbid from {entity:?}.")))?;

        world.entity_mut(entity).despawn_recursive();

        Ok(Self {
            entity_bbid,
            parent,
            serialised: Some(serialised),
        })
    }
}

impl Cmd for AddObjectCmd {
    fn execute(&mut self, world: &mut World) -> Result<(), CmdError> {
        let Some(serialised) = &self.serialised.take() else {
            return Err(CmdError::DoubleExecute);
        };

        let id = serialised.to_entity_recursive(world);

        let maybe_parent = self
            .parent
            .map(|parent_bbid| world.get_entity_id_by_bbid(parent_bbid))
            .flatten();
        // Parent the object if necessary
        if let Some(parent) = maybe_parent {
            world.entity_mut(id).set_parent(parent);
        }

        Ok(())
    }

    fn undo(&mut self, world: &mut World) -> Result<(), CmdError> {
        if self.serialised.is_some() {
            return Err(CmdError::DoubleUndo);
        }

        // Get entity of app id.
        let target_entity = world
            .get_entity_id_by_bbid(self.entity_bbid)
            .ok_or(CmdError::from(anyhow!(
                "Error finding target. Can't find {:?}.",
                self.entity_bbid
            )))?;

        if let Some(serialised) = SerialisableEntity::from_entity_recursive(world, target_entity) {
            self.serialised = Some(serialised);

            world.entity_mut(target_entity).despawn_recursive();

            Ok(())
        } else {
            Err(CmdError::from(anyhow!(
                "Cannot serialise entity {}",
                self.entity_bbid
            )))
        }
    }

    fn try_update_from_prev(&mut self, _other: &CmdType) -> super::CmdUpdateTreatment {
        super::CmdUpdateTreatment::AsSeperate
    }
}
