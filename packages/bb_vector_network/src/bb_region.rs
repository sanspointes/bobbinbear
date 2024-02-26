use std::fmt::Display;

use flo_curves::{bezier::Curve, Coord2};

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents an index position of a BBVNRegion, which are joins between two nodes.
pub struct BBRegionIndex(pub usize);
impl From<usize> for BBRegionIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<BBRegionIndex> for usize {
    fn from(value: BBRegionIndex) -> Self {
        value.0
    }
}

impl From<&mut BBRegionIndex> for usize {
    fn from(value: &mut BBRegionIndex) -> Self {
        value.0
    }
}
impl Display for BBRegionIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "e#{}", self.0)
    }
}


#[derive(Debug, Clone, Default)]
#[cfg_attr( feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(dead_code)]
pub struct BBCycle {
    pub filled: bool,
    pub edges: Vec<BBEdgeIndex>,
    pub children: Vec<BBCycle>,
}

impl BBCycle {
    pub fn new(edges: Vec<BBEdgeIndex>) -> Self {
        Self {
            filled: true,
            edges,
            children: vec![],
        }
    }

    fn edges_deep_inner(&self, edges_out: &mut Vec<Vec<BBEdgeIndex>>) {
        edges_out.push(self.edges.clone());

        for c in &self.children {
            c.edges_deep_inner(edges_out);
        }
    }

    pub fn edges_deep(&self) -> Vec<Vec<BBEdgeIndex>> {
        let mut edges = vec![];
        self.edges_deep_inner(&mut edges);
        edges
    }
}

#[derive(Debug, Clone)]
#[cfg_attr( feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BBWindingRule {
    Default,
    NonZero,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(dead_code)]
pub struct BBRegion {
    winding_rule: BBWindingRule,
    pub root_cycle: BBCycle,
}

impl BBRegion {
    pub fn new(root_cycle: BBCycle) -> Self {
        Self {
            winding_rule: BBWindingRule::Default,
            root_cycle,
        }
    }
}
