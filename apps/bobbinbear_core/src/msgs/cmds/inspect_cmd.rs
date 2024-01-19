use std::{fmt::Display, sync::Arc};

use anyhow::anyhow;
use bevy::{prelude::*, ecs::event::event_update_condition};

use crate::{
    components::{bbid::{BBId, BBIdUtils}, scene::BBObject},
    plugins::inspect_plugin::{InspectState, update_inspect_state}, msgs::{MsgQue, effect::EffectMsg},
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

    fn get_inspected_entity(world: &mut World) -> Option<(Entity, BBId)> {
        let mut q_selected = world.query_filtered::<(Entity, &BBId), With<InspectingTag>>();
        q_selected.get_single(world).ok().map(|(e, b)| (e, *b))
    }

    /// Takes the current target, inspects it.  If there was a previous target, stores it for
    /// undoing.
    ///
    /// * `world`:
    fn perform(&mut self, world: &mut World, responder: &mut MsgQue) -> Result<(), CmdError> {
        let target = self.target.take();

        if let Some((entity, bbid)) = InspectCmd::get_inspected_entity(world) {
            println!("Prev inspected: {entity:?} {bbid}");
            if Some(bbid) != target {
                update_inspect_state(world, InspectState::None);
                world.entity_mut(entity).remove::<InspectingTag>();

                let object_type = *world.get::<BBObject>(entity).unwrap();
                responder.push_internal(EffectMsg::ObjectUninspected { object_type, target: bbid });
                self.target = Some(bbid);
            }
        }

        if let Some(target) = target {
            let entity = world.try_bbid(target).ok_or(anyhow!(
                "InspectCmd: Cannot find target {target:?} in scene."
            ))?;

            if let Some(object_type) = world.get::<BBObject>(entity).cloned() {
                world.entity_mut(entity).insert(InspectingTag);
                update_inspect_state(world, InspectState::InspectVector);
                info!("InspectCmd: Inspected {target:?}");
                responder.push_internal(EffectMsg::ObjectInspected { object_type, target });
            } else {
                return Err(CmdError::ExecutionError(anyhow!("Can't inspect entity {target} as it doesn't have a BBObject component.")))
            }
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
    fn execute(&mut self, world: &mut World, responder: &mut MsgQue) -> Result<(), CmdError> {
        self.perform(world, responder)
    }
    fn undo(&mut self, world: &mut World, responder: &mut MsgQue) -> Result<(), CmdError> {
        self.perform(world, responder)
    }

    fn try_update_from_prev(&mut self, _other: &CmdType) -> super::CmdUpdateTreatment {
        // TODO: This will need to be adjusted once selection box selection becomes a thing.
        super::CmdUpdateTreatment::AsSeperate
    }
}
