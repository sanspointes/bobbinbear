use std::fmt::Display;

use glam::Vec2;

use super::bb_edge::BBEdgeIndex;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[cfg_attr( feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// Represents a reference to an anchor node in the BBVectorNetwork
pub struct BBNodeIndex(pub usize);
impl From<usize> for BBNodeIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<BBNodeIndex> for usize {
    fn from(value: BBNodeIndex) -> Self {
        value.0
    }
}

impl From<&mut BBNodeIndex> for usize {
    fn from(value: &mut BBNodeIndex) -> Self {
        value.0
    }
}
impl Display for BBNodeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "n#{}", self.0)
    }
}

// OPS
impl std::ops::AddAssign<usize> for BBNodeIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}
impl std::ops::SubAssign<usize> for BBNodeIndex {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}


#[derive(Clone, Debug)]
#[cfg_attr( feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BBNode {
    pub(crate) position: Vec2,
    pub(crate) adjacents: Vec<BBEdgeIndex>, // TODO Convert this to a smallvec
}

impl Display for BBNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [", self.position())?;
        for adj in self.adjacents.iter() {
            write!(f, "{adj},")?;
        }
        write!(f, "]")
    }
}

impl BBNode {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            adjacents: vec![],
        }
    }
    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn adjacents(&self) -> &[BBEdgeIndex] {
        &self.adjacents
    }
}
