#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
/// Represents an index position of a BBVNRegion, which are joins between two anchors.
pub struct BBLinkIndex(pub usize);
impl From<usize> for BBLinkIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<BBLinkIndex> for usize {
    fn from(value: BBLinkIndex) -> Self {
        value.0
    }
}

impl From<&mut BBLinkIndex> for usize {
    fn from(value: &mut BBLinkIndex) -> Self {
        value.0
    }
}

// OPS
impl std::ops::AddAssign<usize> for BBLinkIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}
impl std::ops::SubAssign<usize> for BBLinkIndex {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
/// Represents an index position of a BBVNLink, which are joins between two anchors.
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

// OPS
impl std::ops::AddAssign<usize> for BBRegionIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}
impl std::ops::SubAssign<usize> for BBRegionIndex {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}
