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
