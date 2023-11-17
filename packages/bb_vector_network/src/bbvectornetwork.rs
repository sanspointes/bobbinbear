#![allow(dead_code)]

/// BBVectorNetwork:
///
/// This struct represents a Figma style Vector Network.
///
/// TODO:
///  - Implement `link/quad/cube_from_to` methods for closing a region.

use glam::Vec2;

use crate::{bbvnlink::BBVNLink, bbvnregion::BBVNRegion, bbindex::BBAnchorIndex};

pub struct BBVectorNetwork {
    anchors: Vec<Vec2>,
    links: Vec<BBVNLink>,
    regions: Vec<BBVNRegion>,
}

impl BBVectorNetwork {
    pub fn new(begin_point: Vec2) -> Self {
        Self {
            anchors: vec![begin_point],
            links: vec![BBVNLink::Begin { at: 0.into() }],
            regions: vec![],
        }
    }

    pub(crate) fn anchor_unchecked(&self, index: BBAnchorIndex) -> Vec2 {
        unsafe { *self.anchors.get_unchecked(index.0) }
    }

    /// Returns the index of the BBPathLink that has `end/at` field that coresponds with the
    /// provided index.
    fn links_from_start_anchor(&self, index: BBAnchorIndex) -> Vec<usize> {
        let mut result = vec![];
        for (link_index, link) in self.links.iter().enumerate() {
            if index == link.start_index() {
                result.push(link_index)
            }
        }

        result
    }
    /// Returns the index of the BBPathLink that has `start/at` field that coresponds with the
    /// provided index.
    fn links_from_end_anchor(&self, index: BBAnchorIndex) -> Vec<usize> {
        let mut result = vec![];
        for (link_index, link) in self.links.iter().enumerate() {
            if index == link.end_index() {
                result.push(link_index)
            }
        }

        result
    }

    pub fn has_anchor(&self, index: BBAnchorIndex) -> bool {
        self.anchors.get(index.0).is_some()
    }

    /// Links two anchor nodes as a straight line.
    ///
    /// * `start`:
    /// * `end`:
    fn link_line(&mut self, start: BBAnchorIndex, end: BBAnchorIndex) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));
        self.links.push(BBVNLink::Line {
            start,
            end,
        })
    }

    /// Links two anchor nodes as a quadratic curve with 1 control node.
    ///
    /// * `start`:
    /// * `end`:
    fn link_quadratic(&mut self, start: BBAnchorIndex, ctrl1: Vec2, end: BBAnchorIndex) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));
        self.links.push(BBVNLink::Quadratic {
            start,
            ctrl1,
            end,
        })
    }

    /// Links two anchor nodes as a cubic curve with 2 control node.
    ///
    /// * `start`:
    /// * `end`:
    fn link_cubic(&mut self, start: BBAnchorIndex, ctrl1: Vec2, ctrl2: Vec2, end: BBAnchorIndex) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));
        self.links.push(BBVNLink::Cubic {
            start,
            ctrl1,
            ctrl2,
            end,
        })
    }

    /// Pushes a new anchor node to the BBVectorNetwork 
    ///
    /// * `value`: 
    fn push_anchor(&mut self, value: Vec2) -> BBAnchorIndex {
        self.anchors.push(value);
        (self.anchors.len() - 1).into()
    }

    /**
    * PUBLIC FACING API
    */

    /// Gets a reference to an anchor
    pub fn anchor(&self, index: BBAnchorIndex) -> Option<&Vec2> {
        self.anchors.get(index.0)
    }

    /// Gets a mutable reference to an anchor
    pub fn anchor_mut(&mut self, index: BBAnchorIndex) -> Option<&mut Vec2> {
        self.anchors.get_mut(index.0)
    }

    /// Creates a line from a pre-existing point to a new point
    pub fn line_from(&mut self, start: BBAnchorIndex, to: Vec2) -> BBAnchorIndex {
        debug_assert!(self.has_anchor(start));

        let end = self.push_anchor(to);
        self.link_line(start, end);
        end
    }

    /// Creates a quadratic curve from a pre-existing point to a new point
    pub fn quadratic_from(&mut self, start: BBAnchorIndex, ctrl1: Vec2, to: Vec2) -> BBAnchorIndex {
        debug_assert!(self.has_anchor(start));

        let end = self.push_anchor(to);
        self.link_quadratic(start, ctrl1, end);
        end
    }

    /// Creates a cubic curve from a pre-existing point to a new point
    pub fn cubic_from(&mut self, start: BBAnchorIndex, ctrl1: Vec2, ctrl2: Vec2, to: Vec2) -> BBAnchorIndex {
        debug_assert!(self.has_anchor(start));

        let end = self.push_anchor(to);
        self.link_cubic(start, ctrl1, ctrl2, end);
        end
    }

    /// Adds a direct line from `start` to `end`, rebuilding shapes as needed.
    pub fn line_from_to(&mut self, start: BBAnchorIndex, end: BBAnchorIndex) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));

        self.link_line(start, end);

        // TODO: Add auto region closing
    }
    /// Adds a quadratic curve from `start` to `end`, rebuilding shapes as needed.
    pub fn quadratic_from_to(&mut self, start: BBAnchorIndex, ctrl1: Vec2, end: BBAnchorIndex) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));

        self.link_quadratic(start, ctrl1, end);

        // TODO: Add auto region closing
    }
    /// Adds a cubic curve from `start` to `end`, rebuilding shapes as needed.
    pub fn cubic_from_to(&mut self, start: BBAnchorIndex, ctrl1: Vec2, ctrl2: Vec2, end: BBAnchorIndex) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));

        self.link_cubic(start, ctrl1, ctrl2, end);

        // TODO: Add auto region closing
    }

    /// Deletes an anchor, deletes associated links and breaks regions containing these links. 
    pub fn delete_anchor(&mut self, index: BBAnchorIndex) {
        debug_assert!(self.has_anchor(index));

        self.anchors.remove(index.0);

        let mut links_to_delete = vec![];
        for (link_index, link) in self.links.iter_mut().enumerate() {
            if link.references_index(index) {
                links_to_delete.push(link_index);
            } else {
                link.deincrement_anchor_index_over(index);
            }
        }

        for link_index in links_to_delete {
            self.links.remove(link_index);
        }

        // TODO delete regions.
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use super::BBVectorNetwork;

    #[test]
    fn makes_triangle() {
        let mut bbvn = BBVectorNetwork::new(Vec2::new(10., 10.));
        let start = 0.into();
        let prev = bbvn.line_from(start, Vec2::new(30., 30.));
        let prev = bbvn.line_from(prev, Vec2::new(10., 60.));
        bbvn.line_from_to(prev, start);
    }
}
