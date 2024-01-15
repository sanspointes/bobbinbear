#![allow(dead_code)]

use std::{fmt::Display, ops::Add};

use glam::Vec2;

#[allow(unused_imports)]
#[cfg(feature = "debug_draw")]
use comfy::{draw_text, Vec2Extensions, ORANGE, ORANGE_RED, PURPLE};

use super::{
    bb_graph::BBGraph,
    bb_node::{BBNode, BBNodeIndex},
    errors::BBResult,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[cfg_attr( feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents an index position of a BBVNRegion, which are joins between two nodes.
pub struct BBEdgeIndex(pub usize);
impl From<usize> for BBEdgeIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<BBEdgeIndex> for usize {
    fn from(value: BBEdgeIndex) -> Self {
        value.0
    }
}

impl From<&mut BBEdgeIndex> for usize {
    fn from(value: &mut BBEdgeIndex) -> Self {
        value.0
    }
}
impl Display for BBEdgeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "e#{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr( feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BBEdge {
    Line {
        start: BBNodeIndex,
        end: BBNodeIndex,
    },
    Quadratic {
        start: BBNodeIndex,
        ctrl1: Vec2,
        end: BBNodeIndex,
    },
    Cubic {
        start: BBNodeIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBNodeIndex,
    },
}
impl Display for BBEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Line { start, end } => {
                write!(f, "line: {} -> {}", start, end)
            }
            Self::Quadratic { start, end, .. } => {
                write!(f, "quad: {} -> {}", start, end)
            }
            Self::Cubic { start, end, .. } => {
                write!(f, "cubi: {} -> {}", start, end)
            }
        }
    }
}

impl BBEdge {
    /// Gets the index of the `start` node
    pub fn start_idx(&self) -> BBNodeIndex {
        match self {
            BBEdge::Line { start, .. }
            | BBEdge::Quadratic { start, .. }
            | BBEdge::Cubic { start, .. } => *start,
        }
    }
    pub fn start(&self, bbvn: &BBGraph) -> BBNode {
        bbvn.node(self.start_idx()).unwrap().clone()
    }
    pub fn start_pos(&self, bbvn: &BBGraph) -> Vec2 {
        self.start(bbvn).position()
    }

    /// Gets the index of the `end` node
    pub fn end_idx(&self) -> BBNodeIndex {
        match self {
            BBEdge::Line { end, .. }
            | BBEdge::Quadratic { end, .. }
            | BBEdge::Cubic { end, .. } => *end,
        }
    }
    pub fn end(&self, bbvn: &BBGraph) -> BBNode {
        bbvn.node(self.end_idx()).unwrap().clone()
    }
    pub fn end_pos(&self, bbvn: &BBGraph) -> Vec2 {
        self.end(bbvn).position()
    }

    pub fn set_start_idx(&mut self, start_idx: BBNodeIndex) {
        match self {
            BBEdge::Line { start, .. }
            | BBEdge::Quadratic { start, .. }
            | BBEdge::Cubic { start, .. } => *start = start_idx,
        }
    }
    pub fn set_end_idx(&mut self, end_idx: BBNodeIndex) {
        match self {
            BBEdge::Line { end, .. }
            | BBEdge::Quadratic { end, .. }
            | BBEdge::Cubic { end, .. } => *end = end_idx,
        }
    }

    /// Returns the index on the other side, i.e if you provide the start index it will return the
    /// end index and vice versa.
    pub fn other_node_idx(&self, idx: BBNodeIndex) -> BBNodeIndex {
        match self {
            BBEdge::Line { end, start }
            | BBEdge::Quadratic { end, start, .. }
            | BBEdge::Cubic { end, start, .. } => {
                if idx == *end {
                    *start
                } else {
                    *end
                }
            }
        }
    }

    pub fn t_point(&self, bbvn: &BBGraph, t: f32) -> Vec2 {
        match self {
            BBEdge::Line { .. } => Vec2::lerp(self.start_pos(bbvn), self.end_pos(bbvn), t),
            BBEdge::Quadratic { ctrl1, .. } => {
                let p1 = Vec2::lerp(self.start_pos(bbvn), *ctrl1, t);
                let p2 = Vec2::lerp(*ctrl1, self.end_pos(bbvn), t);
                Vec2::lerp(p1, p2, t)
            }
            BBEdge::Cubic { ctrl1, ctrl2, .. } => {
                let p1 = Vec2::lerp(self.start_pos(bbvn), *ctrl1, t);
                let p2 = Vec2::lerp(*ctrl1, *ctrl2, t);
                let p3 = Vec2::lerp(*ctrl2, self.end_pos(bbvn), t);
                Vec2::lerp(Vec2::lerp(p1, p2, t), Vec2::lerp(p2, p3, t), t)
            }
        }
    }

    /// Calculates the tangent of the bezier/line at `t=0`
    ///
    /// * `bbvn`: The BBGraph to source the point data from
    pub fn calc_start_tangent(&self, bbvn: &BBGraph) -> BBResult<Vec2> {
        match self {
            BBEdge::Line { start, end } => {
                let v_start = bbvn.node(*start)?.position();
                let v_end = bbvn.node(*end)?.position();

                Ok(v_end - v_start)
            }
            BBEdge::Quadratic { start, ctrl1, .. } => {
                let v_start = bbvn.node(*start)?.position();

                Ok(*ctrl1 - v_start)
            }
            BBEdge::Cubic { start, ctrl1, .. } => {
                let v_start = bbvn.node(*start)?.position();

                Ok(*ctrl1 - v_start)
            }
        }
    }

    /// Calculates the tangent of the bezier/line at `t=1`
    ///
    /// * `bbvn`: The BBGraph to source the point data from
    pub fn calc_end_tangent(&self, bbvn: &BBGraph) -> BBResult<Vec2> {
        match self {
            BBEdge::Line { start, end } => {
                let v_start = bbvn.node(*start)?.position();
                let v_end = bbvn.node(*end)?.position();

                Ok(v_end - v_start)
            }
            BBEdge::Quadratic { end, ctrl1, .. } => {
                let v_end = bbvn.node(*end)?.position();

                Ok(v_end - *ctrl1)
            }
            BBEdge::Cubic { end, ctrl2, .. } => {
                let v_end = bbvn.node(*end)?.position();

                Ok(v_end - *ctrl2)
            }
        }
    }
    /// Returns true if this BBEdge references the given index.
    ///
    /// * `index`:
    pub fn contains_node_idx(&self, index: BBNodeIndex) -> bool {
        self.start_idx() == index || self.end_idx() == index
    }

    /// Checks if another edge shares a node with self.
    ///
    /// * `other`: Other edge
    pub fn shares_node_idx(&self, other: &BBEdge) -> bool {
        self.contains_node_idx(other.start_idx()) || self.contains_node_idx(other.end_idx())
    }

    /// Returns the node that two edges share (are linked by) otherwise None.
    ///
    /// * `other`: The other edge that you want to compare.
    pub fn shared_node(&self, other: &BBEdge) -> Option<BBNodeIndex> {
        if self.contains_node_idx(other.start_idx()) {
            Some(other.start_idx())
        } else if self.contains_node_idx(other.end_idx()) {
            Some(other.end_idx())
        } else {
            None
        }
    }

    /// Returns a clone of the BBEdge but in the reversed direction
    pub fn reversed(&self) -> Self {
        match self {
            BBEdge::Line { start, end } => BBEdge::Line {
                start: *end,
                end: *start,
            },
            BBEdge::Quadratic { start, ctrl1, end } => BBEdge::Quadratic {
                start: *end,
                ctrl1: *ctrl1,
                end: *start,
            },
            BBEdge::Cubic {
                start,
                ctrl1,
                ctrl2,
                end,
            } => BBEdge::Cubic {
                start: *end,
                ctrl1: *ctrl2,
                ctrl2: *ctrl1,
                end: *start,
            },
        }
    }

    /// Returns the edge, redirected to start from the provided node and point to the other node.
    pub fn directed_from(self, from_node_idx: BBNodeIndex) -> BBEdge {
        if self.start_idx() == from_node_idx {
            self
        } else {
            self.reversed()
        }
    }

    pub fn translate(&mut self, translation: Vec2) {
        match self {
            Self::Quadratic { ctrl1, .. } => {
                *ctrl1 = ctrl1.add(translation);
            }
            Self::Cubic { ctrl1, ctrl2, .. } => {
                *ctrl1 = ctrl1.add(translation);
                *ctrl2 = ctrl2.add(translation);
            }
            _ => (),
        }
    }

    pub fn adjacents(&self, graph: &BBGraph) -> BBResult<Vec<BBEdgeIndex>> {
        let start_adjs = graph.node(self.start_idx())?.adjacents();
        let end_adjs = graph.node(self.end_idx())?.adjacents();
        let mut adjs = vec![];
        adjs.extend_from_slice(start_adjs);
        adjs.extend_from_slice(end_adjs);
        // adjs.push();
        // adjs.push(graph.node(self.end_idx())?.adjacents());
        Ok(adjs)
    }
}
