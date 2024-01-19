use std::{fmt::{Debug, Display}, mem, sync::Arc};

use anyhow::anyhow;
use bevy::prelude::*;

use crate::{components::bbid::{BBId, BBIdUtils}, plugins::{vector_graph_plugin::VectorGraph, bounds_2d_plugin::GlobalBounds2D}, msgs::{effect::EffectMsg, MsgQue}};

use super::{Cmd, CmdError, CmdType, CmdMsg, CmdUpdateTreatment};

pub struct UpdateVectorGraphCmd {
    name: String,
    pub target_bbid: BBId,
    vector_graph: VectorGraph,
}
impl From<UpdateVectorGraphCmd> for CmdType {
    fn from(value: UpdateVectorGraphCmd) -> Self {
        Self::UpdateVectorGraph(value)
    }
}
impl From<UpdateVectorGraphCmd> for CmdMsg {
    fn from(value: UpdateVectorGraphCmd) -> Self {
        let cmd_type: CmdType = value.into();
        CmdMsg::Execute(Arc::new(cmd_type))
    }
}

impl Display for UpdateVectorGraphCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UpdateVectorGraphCmd on {}",
            self.target_bbid,
        )
    }
}

impl Debug for UpdateVectorGraphCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateVectorGraphCmd")
            .field("target_bbid", &self.target_bbid)
            .field("vector_graph", &self.vector_graph)
            .finish()
    }
}

impl UpdateVectorGraphCmd {
    pub fn new(target_bbid: BBId, vector_graph: VectorGraph) -> Self {
        Self {
            name: format!("Update vector_graph on \"{}\"", target_bbid),
            target_bbid,
            vector_graph,
        }
    }

    fn swap_path(
        &mut self,
        world: &mut World,
        target_bbid: BBId,
        responder: &mut MsgQue,
    ) -> Result<(), CmdError> {
        let target_entity = world
            .try_bbid(target_bbid)
            .ok_or(anyhow!("Can't find entity entity with {target_bbid:?}."))?;

        let mut vector_graph = world
            .query::<&mut VectorGraph>()
            .get_mut(world, target_entity)
            .map_err(|err| anyhow!("Error getting path of {target_entity:?}.\n - Reason: {err:?}."))?;

        mem::swap(&mut vector_graph.0, &mut self.vector_graph.0);
        vector_graph.set_changed();

        if let Some(mut global_bounds) = world.get_mut::<GlobalBounds2D>(target_entity) {
            *global_bounds = GlobalBounds2D::NeedsCalculate;
        }

        responder.push_internal(EffectMsg::GraphStructureChanged { bbid: target_bbid });
        responder.push_internal(EffectMsg::GraphNeedsRemesh { bbid: target_bbid });

        Ok(())
    }
}

impl Cmd for UpdateVectorGraphCmd {
    fn execute(&mut self, world: &mut bevy::prelude::World, responder: &mut MsgQue) -> Result<(), CmdError> {
        self.swap_path(world, self.target_bbid, responder)
    }
    fn undo(&mut self, world: &mut bevy::prelude::World, responder: &mut MsgQue) -> Result<(), CmdError> {
        self.swap_path(world, self.target_bbid, responder)
    }

    fn try_update_from_prev(&mut self, other: &CmdType) -> super::CmdUpdateTreatment {
        match other {
            CmdType::UpdateVectorGraph(cmd) => {
                CmdUpdateTreatment::AsRepeat
            }
            _ => CmdUpdateTreatment::AsSeperate,
        }
        
    }
}
