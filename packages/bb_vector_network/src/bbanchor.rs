use glam::Vec2;

use crate::{BBLinkIndex, BBVectorNetwork};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
// Represents a reference to an anchor node in the BBVectorNetwork
pub struct BBAnchorIndex(pub usize);
impl From<usize> for BBAnchorIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<BBAnchorIndex> for usize {
    fn from(value: BBAnchorIndex) -> Self {
        value.0
    }
}

impl From<&mut BBAnchorIndex> for usize {
    fn from(value: &mut BBAnchorIndex) -> Self {
        value.0
    }
}

// OPS
impl std::ops::AddAssign<usize> for BBAnchorIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}
impl std::ops::SubAssign<usize> for BBAnchorIndex {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}


#[derive(Clone, Debug)]
pub struct BBAnchor {
    pub(crate) position: Vec2,
    pub(crate) adjacents: Vec<BBLinkIndex>, // TODO Convert this to a smallvec
}

impl BBAnchor {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            adjacents: vec![],
        }
    }
    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn adjacents(&self) -> &[BBLinkIndex] {
        &self.adjacents
    }
}
