use std::{fmt::Display, sync::Arc};

use bevy::prelude::*;

use crate::{
    components::bbid::{BBId, BBIdUtils},
    msgs::{effect::EffectMsg, MsgQue},
    plugins::selection_plugin::Selected,
};

use super::{Cmd, CmdError, CmdMsg, CmdType};

#[derive(Debug)]
pub struct SelectObjectsCmd {
    pub to_select: Vec<BBId>,
    pub to_deselect: Vec<BBId>,
}

impl From<SelectObjectsCmd> for CmdType {
    fn from(value: SelectObjectsCmd) -> Self {
        Self::SelectObjects(value)
    }
}
impl From<SelectObjectsCmd> for CmdMsg {
    fn from(value: SelectObjectsCmd) -> Self {
        let cmd_type: CmdType = value.into();
        CmdMsg::Execute(Arc::new(cmd_type))
    }
}

impl Display for SelectObjectsCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SelectObjectsCmd: Selecting {:?}, Deselecting {:?}",
            self.to_select, self.to_deselect
        )
    }
}

impl SelectObjectsCmd {
    pub fn select(to_select: Vec<BBId>) -> Self {
        Self {
            to_select,
            to_deselect: vec![],
        }
    }
    pub fn deselect(to_deselect: Vec<BBId>) -> Self {
        Self {
            to_select: vec![],
            to_deselect,
        }
    }
    pub fn select_deselect(to_select: Vec<BBId>, to_deselect: Vec<BBId>) -> Self {
        Self {
            to_select,
            to_deselect,
        }
    }

    pub fn get_to_select_to_deselect_entities(
        &self,
        world: &mut World,
    ) -> Result<(Vec<Entity>, Vec<Entity>), CmdError> {
        let to_select_entities = world.try_entities_by_bbid_vec(&self.to_select)?;
        let to_deselect_entities = world.try_entities_by_bbid_vec(&self.to_deselect)?;

        Ok((to_select_entities, to_deselect_entities))
    }

    pub fn apply_select_deselect(
        &self,
        world: &mut World,
        to_select: &Vec<BBId>,
        to_deselect: &Vec<BBId>,
        responder: &mut MsgQue,
    ) -> Result<(), CmdError> {
        let mut q_selectable = world.query::<&mut Selected>();

        let to_select_e: Vec<_> = to_select.iter().map(|bbid| world.bbid(*bbid)).collect();
        for entity in to_select_e {
            let mut selected = q_selectable.get_mut(world, entity)?;
            *selected = Selected::Yes;
        }
        let to_deselect_e: Vec<_> = to_deselect.iter().map(|bbid| world.bbid(*bbid)).collect();
        for entity in to_deselect_e {
            let mut selected = q_selectable.get_mut(world, entity)?;
            *selected = Selected::No;
        }

        let mut q_selectable = world.query::<(&BBId, &Selected)>();
        let currently_selected: Vec<_> = q_selectable
            .iter(world)
            .filter_map(|(bbid, selected)| match selected {
                Selected::Yes => Some(bbid),
                Selected::No => None,
            })
            .cloned()
            .collect();

        responder.push_internal(EffectMsg::ObjectSelectionChanged {
            selected: to_select.clone(),
            deselected: to_deselect.clone(),
            currently_selected,
        });

        Ok(())
    }
}

impl Cmd for SelectObjectsCmd {
    fn execute(&mut self, world: &mut World, responder: &mut MsgQue) -> Result<(), CmdError> {
        self.apply_select_deselect(world, &self.to_select, &self.to_deselect, responder)?;
        Ok(())
    }
    fn undo(
        &mut self,
        world: &mut bevy::prelude::World,
        responder: &mut MsgQue,
    ) -> Result<(), CmdError> {
        // Same as above but selecting / deselecting is switched
        self.apply_select_deselect(world, &self.to_deselect, &self.to_select, responder)?;
        Ok(())
    }

    fn try_update_from_prev(&mut self, _other: &CmdType) -> super::CmdUpdateTreatment {
        // TODO: This will need to be adjusted once selection box selection becomes a thing.
        super::CmdUpdateTreatment::AsSeperate
    }
}
