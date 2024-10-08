use std::{fmt::Display, sync::Arc};

use anyhow::anyhow;
use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    components::bbid::BBId,
    msgs::{
        effect::{EffectMsg, ObjectMovedEffect},
        Msg, MsgQue,
    },
    plugins::bounds_2d_plugin::GlobalBounds2D,
};

use super::{Cmd, CmdError, CmdMsg, CmdType, CmdUpdateTreatment};

#[derive(Debug)]
pub struct MoveObjectModel {
    target: BBId,
    initial_position: Option<Vec3>,
}

#[derive(Debug)]
pub struct MoveObjectsCmd {
    /// Stores entity BBIds, their original position + their new position
    pub to_move: Vec<MoveObjectModel>,
    pub offset: Vec2,
}
impl From<MoveObjectsCmd> for CmdType {
    fn from(value: MoveObjectsCmd) -> Self {
        Self::MoveObjects(value)
    }
}
impl From<MoveObjectsCmd> for CmdMsg {
    fn from(value: MoveObjectsCmd) -> Self {
        let cmd_type: CmdType = value.into();
        CmdMsg::Execute(Arc::new(cmd_type))
    }
}
impl From<MoveObjectsCmd> for Msg {
    fn from(value: MoveObjectsCmd) -> Self {
        Msg::from(CmdMsg::from(value))
    }
}

impl Display for MoveObjectsCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Moving {} objects.", self.to_move.len(),)
    }
}

impl MoveObjectsCmd {
    pub fn from_single(bbid: BBId, offset: Vec2) -> Self {
        Self {
            to_move: vec![MoveObjectModel {
                target: bbid,
                initial_position: None,
            }],
            offset,
        }
    }
    pub fn from_multiple(bbids: Vec<BBId>, offset: Vec2) -> Self {
        let to_move: Vec<MoveObjectModel> = bbids
            .into_iter()
            .map(|bbid| MoveObjectModel {
                target: bbid,
                initial_position: None,
            })
            .collect();

        Self { to_move, offset }
    }

    fn get_to_move_entities(&self, world: &mut World) -> Vec<(Entity, BBId)> {
        let mut q_transforms = world.query::<(Entity, &BBId)>();

        q_transforms
            .iter_mut(world)
            .filter_map(|(entity, bbid)| {
                match self.to_move.iter().any(|model| model.target.eq(bbid)) {
                    true => Some((entity, *bbid)),
                    false => None,
                }
            })
            .collect()
    }
}

impl Cmd for MoveObjectsCmd {
    fn execute(&mut self, world: &mut World, responder: &mut MsgQue) -> Result<(), CmdError> {
        let to_move_entities = self.get_to_move_entities(world);
        let mut ss_state = SystemState::<(
            Query<(&mut Transform, Option<&Parent>, Option<&mut GlobalBounds2D>)>,
            Query<&GlobalTransform>,
        )>::new(world);
        let (mut q_movable, mut q_parent) = ss_state.get_mut(world);

        let mut to_callback = vec![];

        // to_move_transforms.iite
        for (entity, bbid) in to_move_entities {
            let model = self
                .to_move
                .iter_mut()
                .find(|model| bbid.eq(&model.target))
                .ok_or(anyhow!("Could not find to_move with bbid {bbid:?}"))?;

            let (mut transform, maybe_parent, maybe_bounds_2d) =
                q_movable.get_mut(entity).map_err(|err| {
                    anyhow!("Could not get transform on entity {entity:?}.\n - Reason {err:?}")
                })?;

            if model.initial_position.is_none() {
                #[cfg(feature = "debug_select")]
                debug!(
                    "Setting {bbid:?} initial position to {:?}",
                    transform.translation
                );
                model.initial_position = Some(transform.translation);
            }

            let initial_position = model.initial_position.unwrap();

            transform.translation.x = initial_position.x + self.offset.x;
            transform.translation.y = initial_position.y + self.offset.y;

            // TODO: Low priority. Make this faster by translating the bounds
            if let Some(mut bounds_2d) = maybe_bounds_2d {
                *bounds_2d = GlobalBounds2D::NeedsCalculate;
            }

            let mut world_position = transform.translation;
            if let Some(parent_e) = maybe_parent {
                if let Ok(global_transform) = q_parent.get(**parent_e) {
                    world_position = global_transform.transform_point(world_position);
                }
            }
            to_callback.push(ObjectMovedEffect {
                target: bbid,
                world_position,
            })
        }

        for effect in to_callback {
            responder.push_internal(EffectMsg::ObjectMoved(effect))
        }

        Ok(())
    }
    fn undo(&mut self, world: &mut World, responder: &mut MsgQue) -> Result<(), CmdError> {
        let to_move_entities = self.get_to_move_entities(world);
        let mut ss_state = SystemState::<(
            Query<(&mut Transform, Option<&Parent>, Option<&mut GlobalBounds2D>)>,
            Query<&GlobalTransform>,
        )>::new(world);
        let (mut q_movable, mut q_parent) = ss_state.get_mut(world);

        let mut to_callback = vec![];

        // to_move_transforms.iite
        for (entity, bbid) in to_move_entities {
            let model = self
                .to_move
                .iter_mut()
                .find(|model| bbid.eq(&model.target))
                .ok_or(anyhow!("Could not find to_move with bbid {bbid:?}"))?;

            let (mut transform, maybe_parent, maybe_bounds_2d) =
                q_movable.get_mut(entity).map_err(|err| {
                    anyhow!("Could not get transform on entity {entity:?}.\n - Reason {err:?}")
                })?;

            let initial_position = model
                .initial_position
                .expect("Model does not have initial position.  This should never happen.");

            transform.translation.x = initial_position.x;
            transform.translation.y = initial_position.y;

            // TODO: Low priority. Make this faster by translating the bounds
            if let Some(mut bounds_2d) = maybe_bounds_2d {
                *bounds_2d = GlobalBounds2D::NeedsCalculate;
            }

            let mut world_position = transform.translation;
            if let Some(parent_e) = maybe_parent {
                if let Ok(global_transform) = q_parent.get(**parent_e) {
                    world_position = global_transform.transform_point(world_position);
                }
            }
            to_callback.push(ObjectMovedEffect {
                target: bbid,
                world_position,
            })
        }

        for effect in to_callback {
            responder.push_internal(EffectMsg::ObjectMoved(effect))
        }

        Ok(())
    }

    fn try_update_from_prev(&mut self, other: &CmdType) -> super::CmdUpdateTreatment {
        match other {
            CmdType::MoveObjects(cmd) => {
                let same_bbids = self.to_move.iter_mut().zip(cmd.to_move.iter()).all(|(my, other)| {
                    let same_bbid = my.target.eq(&other.target);

                    match (same_bbid, other.initial_position) {
                        (true, Some(initial_position)) => {
                            my.initial_position = Some(initial_position);
                        }
                        (true, None) => warn!("MoveObjectsCmd.try_update_from_prev(): Prev has no initial_position to take {:?}.", other.target),
                        (false, _) => warn!("MoveObjectsCmd.try_update_from_prev(): Command bbids are different. {:?} {:?}", my.target, other.target),
                    } 
                    same_bbid
                });

                if same_bbids {
                    CmdUpdateTreatment::AsRepeat
                } else {
                    CmdUpdateTreatment::AsSeperate
                }
            }
            _ => CmdUpdateTreatment::AsSeperate,
        }
    }
}
