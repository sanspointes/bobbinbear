use std::{fmt::Display, sync::Arc};

use anyhow::anyhow;
use bb_vector_network::prelude::{BBEdge, BBEdgeIndex, BBGraph, BBNodeIndex};
use bevy::prelude::*;

use crate::{
    components::{
        bbid::{BBId, BBIdUtils},
        scene::VectorGraphDirty,
    },
    msgs::{effect::EffectMsg, MsgQue},
    plugins::vector_graph_plugin::VectorGraph,
};

use super::{Cmd, CmdError, CmdMsg, CmdType};

#[derive(Debug, Clone, Copy)]
/// Stores a reference to a pre-existing or not yet created BBNode.
/// For the `New` variant, if used with a target bbid then coordinates are in screen space
/// else they are in world space.
pub enum AddRemoveEdgeNode {
    Existing(BBNodeIndex),
    New(Vec2),
}

impl From<Vec2> for AddRemoveEdgeNode {
    fn from(value: Vec2) -> Self {
        Self::New(value)
    }
}

impl AddRemoveEdgeNode {
    pub fn from_idx_or_local_pos(idx: Option<BBNodeIndex>, local_pos: Vec2) -> Self {
        match idx {
            Some(idx) => AddRemoveEdgeNode::Existing(idx),
            None => AddRemoveEdgeNode::New(local_pos),
        }
    }

    pub fn local_position_from_graph(&self, graph: &BBGraph) -> Vec2 {
        match self {
            Self::Existing(idx) => graph.node(*idx).unwrap().position(),
            Self::New(pos) => *pos,
        }
    }
    /// Gets the position of this node reference.  Either from the stored new pos or by querying
    /// the existing node from the reference graph.
    ///
    /// * `world`:
    /// * `bbid`: BBId of the entity with the VectorGraph
    pub fn local_position_from_bbid(&self, world: &mut World, bbid: BBId) -> Vec2 {
        match self {
            AddRemoveEdgeNode::New(pos) => *pos,
            AddRemoveEdgeNode::Existing(node_idx) => {
                let graph = world.bbid_get::<VectorGraph>(bbid);
                self.local_position_from_graph(&graph.0)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AddRemoveEdgeType {
    Line {
        start: AddRemoveEdgeNode,
        end: AddRemoveEdgeNode,
    },
    Quadratic {
        start: AddRemoveEdgeNode,
        ctrl1: Vec2,
        end: AddRemoveEdgeNode,
    },
    Cubic {
        start: AddRemoveEdgeNode,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: AddRemoveEdgeNode,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum AddRemoveEdgeAction {
    Add(AddRemoveEdgeType),
    Remove(BBEdgeIndex),
}

impl Display for AddRemoveEdgeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(edge_type) => {
                write!(f, "Adding {edge_type:?}")
            }
            Self::Remove(edge_idx) => {
                write!(f, "Removing {edge_idx}")
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AddRemoveEdgeCmd {
    action: AddRemoveEdgeAction,
    target: BBId,
}

impl From<AddRemoveEdgeCmd> for CmdType {
    fn from(value: AddRemoveEdgeCmd) -> Self {
        CmdType::AddRemoveEdge(value)
    }
}

impl From<AddRemoveEdgeCmd> for CmdMsg {
    fn from(value: AddRemoveEdgeCmd) -> Self {
        let cmd_type: CmdType = value.into();
        CmdMsg::Execute(Arc::new(cmd_type))
    }
}

impl Display for AddRemoveEdgeCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on object with BBID \"{}\"", self.action, self.target)
    }
}

impl AddRemoveEdgeCmd {
    pub fn new_add_line(target: BBId, start: AddRemoveEdgeNode, end: AddRemoveEdgeNode) -> Self {
        Self {
            action: AddRemoveEdgeAction::Add(AddRemoveEdgeType::Line { start, end }),
            target,
        }
    }

    pub fn new_add_quadratic(
        target: BBId,
        start: AddRemoveEdgeNode,
        ctrl1: Vec2,
        end: AddRemoveEdgeNode,
    ) -> Self {
        Self {
            action: AddRemoveEdgeAction::Add(AddRemoveEdgeType::Quadratic { start, ctrl1, end }),
            target,
        }
    }

    pub fn new_add_cubic(
        target: BBId,
        start: AddRemoveEdgeNode,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: AddRemoveEdgeNode,
    ) -> Self {
        Self {
            action: AddRemoveEdgeAction::Add(AddRemoveEdgeType::Cubic {
                start,
                ctrl1,
                ctrl2,
                end,
            }),
            target,
        }
    }
}

impl AddRemoveEdgeCmd {
    fn add_edge(&mut self, graph: &mut BBGraph, edge_type: &AddRemoveEdgeType) -> BBEdgeIndex {
        debug!("AddRemoveEdgeCmd::add_edge(edge_type: {edge_type:?})");
        use AddRemoveEdgeNode::*;

        let (idx, _) = match edge_type {
            AddRemoveEdgeType::Line { start, end } => match (start, end) {
                (New(s_pos), New(e_pos)) => graph.line(*s_pos, *e_pos),
                (Existing(s_idx), New(e_pos)) => graph.line_from(*s_idx, *e_pos),
                (New(s_pos), Existing(e_idx)) => graph.line_to(*s_pos, *e_idx),
                (Existing(s_idx), Existing(e_idx)) => graph.line_from_to(*s_idx, *e_idx),
            },
            AddRemoveEdgeType::Quadratic { start, ctrl1, end } => match (start, end) {
                (New(s_pos), New(e_pos)) => graph.quadratic(*s_pos, *ctrl1, *e_pos),
                (Existing(s_idx), New(e_pos)) => graph.quadratic_from(*s_idx, *ctrl1, *e_pos),
                (New(s_pos), Existing(e_idx)) => graph.quadratic_to(*s_pos, *ctrl1, *e_idx),
                (Existing(s_idx), Existing(e_idx)) => {
                    graph.quadratic_from_to(*s_idx, *ctrl1, *e_idx)
                }
            },
            AddRemoveEdgeType::Cubic {
                start,
                ctrl1,
                ctrl2,
                end,
            } => match (start, end) {
                (New(s_pos), New(e_pos)) => graph.cubic(*s_pos, *ctrl1, *ctrl2, *e_pos),
                (Existing(s_idx), New(e_pos)) => graph.cubic_from(*s_idx, *ctrl1, *ctrl2, *e_pos),
                (New(s_pos), Existing(e_idx)) => graph.cubic_to(*s_pos, *ctrl1, *ctrl2, *e_idx),
                (Existing(s_idx), Existing(e_idx)) => {
                    graph.cubic_from_to(*s_idx, *ctrl1, *ctrl2, *e_idx)
                }
            },
        };

        idx
    }
    fn remove_edge(&mut self, graph: &mut BBGraph, edge_idx: &BBEdgeIndex) -> AddRemoveEdgeAction {
        debug!("AddRemoveEdgeCmd::remove_edge(edge_idx: {edge_idx:?})");
        let edge = *graph.edge(*edge_idx).unwrap();
        let original_start = graph.node(edge.start_idx()).unwrap().clone();
        let original_end = graph.node(edge.end_idx()).unwrap().clone();

        graph.delete_edge(*edge_idx).unwrap();

        let start = match graph.node(edge.start_idx()) {
            Ok(_) => AddRemoveEdgeNode::Existing(edge.start_idx()),
            Err(_) => AddRemoveEdgeNode::New(original_start.position()),
        };
        let end = match graph.node(edge.end_idx()) {
            Ok(_) => AddRemoveEdgeNode::Existing(edge.end_idx()),
            Err(_) => AddRemoveEdgeNode::New(original_end.position()),
        };

        AddRemoveEdgeAction::Add(match edge {
            BBEdge::Line { .. } => AddRemoveEdgeType::Line { start, end },
            BBEdge::Quadratic { ctrl1, .. } => AddRemoveEdgeType::Quadratic { start, ctrl1, end },
            BBEdge::Cubic { ctrl1, ctrl2, .. } => AddRemoveEdgeType::Cubic {
                start,
                ctrl1,
                ctrl2,
                end,
            },
        })
    }

    fn perform(
        &mut self,
        world: &mut World,
        responder: &mut MsgQue,
    ) -> Result<(), super::CmdError> {
        let target_entity = world.bbid(self.target);
        let mut graph =
            world
                .get_mut::<VectorGraph>(target_entity)
                .ok_or(CmdError::ExecutionError(anyhow!(
                    "Could not get target bbid"
                )))?;

        let (next_action, effect) = match self.action {
            AddRemoveEdgeAction::Add(edge_type) => {
                let idx = self.add_edge(&mut graph.0, &edge_type);
                (
                    AddRemoveEdgeAction::Remove(idx),
                    EffectMsg::EdgeAdded {
                        target: self.target,
                        idx,
                    },
                )
            }
            AddRemoveEdgeAction::Remove(edge_idx) => (
                self.remove_edge(&mut graph.0, &edge_idx),
                EffectMsg::EdgeRemoved {
                    target: self.target,
                    idx: edge_idx,
                },
            ),
        };

        let e = world.bbid(self.target);
        println!("Components: {:#?}", world.entity_components(e));

        let mut dirty_state = world.bbid_get_mut::<VectorGraphDirty>(self.target);
        *dirty_state = VectorGraphDirty::Dirty;

        responder.push_internal(effect);

        self.action = next_action;

        Ok(())
    }
}

impl Cmd for AddRemoveEdgeCmd {
    fn execute(
        &mut self,
        world: &mut World,
        responder: &mut MsgQue,
    ) -> Result<(), super::CmdError> {
        self.perform(world, responder)
    }

    fn undo(&mut self, world: &mut World, responder: &mut MsgQue) -> Result<(), super::CmdError> {
        self.perform(world, responder)
    }

    fn try_update_from_prev(&mut self, _other: &super::CmdType) -> super::CmdUpdateTreatment {
        super::CmdUpdateTreatment::AsSeperate
    }
}
