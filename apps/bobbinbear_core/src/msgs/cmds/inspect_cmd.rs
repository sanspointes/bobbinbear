use std::{fmt::Display, sync::Arc};

use anyhow::anyhow;
use bevy::prelude::*;

use crate::{
    components::bbid::{BBId, BBIdUtils},
    plugins::inspect_plugin::{InspectState, update_inspect_state},
};

use super::{Cmd, CmdError, CmdMsg, CmdType};

#[derive(Component)]
pub struct InspectingTag;

#[derive(Debug)]
pub struct InspectCmd {
    pub target: Option<BBId>,
}
impl InspectCmd {
    pub fn inspect(target: BBId) -> Self {
        Self {
            target: Some(target),
        }
    }

    fn get_selected_bbid(world: &mut World) -> Option<(Entity, BBId)> {
        let mut q_selected = world.query_filtered::<(Entity, &BBId), With<InspectingTag>>();
        q_selected.get_single(world).ok().map(|(e, b)| (e, *b))
    }

    /// Takes the current target, inspects it.  If there was a previous target, stores it for
    /// undoing.
    ///
    /// * `world`:
    fn perform(&mut self, world: &mut World) -> Result<(), CmdError> {
        let target = self.target.take();

        // Unselect the previous selection, if any
        let prev_inspected = {
            let selected = InspectCmd::get_selected_bbid(world);
            match (selected, target) {
                (Some((entity, bbid)), Some(target)) => {
                    if target != bbid {
                        update_inspect_state(world, InspectState::None);
                        world.entity_mut(entity).remove::<InspectingTag>();
                        Some(bbid)
                    } else {
                        None
                    }
                }
                (Some((entity, bbid)), None) => {
                    update_inspect_state(world, InspectState::None);
                    world.entity_mut(entity).remove::<InspectingTag>();
                    Some(bbid)
                }
                (_, _) => None,
            }
        };

        info!("Deinspected {prev_inspected:?}");

        if let Some(target) = target {
            let entity = world.get_entity_id_by_bbid(target).ok_or(anyhow!(
                "InspectCmd: Cannot find target {target:?} in scene."
            ))?;
            world.entity_mut(entity).insert(InspectingTag);
            update_inspect_state(world, InspectState::InspectVector);
            info!("InspectCmd: Inspected {target:?}");
        }

        if let Some(prev) = prev_inspected {
            self.target = Some(prev);
        }
        Ok(())
    }
}

impl From<InspectCmd> for CmdType {
    fn from(value: InspectCmd) -> Self {
        Self::Inspect(value)
    }
}
impl From<InspectCmd> for CmdMsg {
    fn from(value: InspectCmd) -> Self {
        let cmd_type: CmdType = value.into();
        CmdMsg::Execute(Arc::new(cmd_type))
    }
}

impl Display for InspectCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InspectCmd: Target {:?}", self.target)
    }
}
impl Cmd for InspectCmd {
    fn execute(&mut self, world: &mut World) -> Result<(), CmdError> {
        self.perform(world)
    }
    fn undo(&mut self, world: &mut bevy::prelude::World) -> Result<(), CmdError> {
        self.perform(world)
    }

    fn try_update_from_prev(&mut self, _other: &CmdType) -> super::CmdUpdateTreatment {
        // TODO: This will need to be adjusted once selection box selection becomes a thing.
        super::CmdUpdateTreatment::AsSeperate
    }
}
