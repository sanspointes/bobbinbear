use std::fmt::Display;

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
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
}

#[derive(Debug, Clone)]
pub enum BBWindingRule {
    Default,
    NonZero,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BBRegion {
    winding_rule: BBWindingRule,
    pub cycles: Vec<BBCycle>
}

impl BBRegion {
    pub fn new(cycles: Vec<BBCycle>) -> Self {
        Self {
            winding_rule: BBWindingRule::Default,
            cycles,
        }
    }
}
