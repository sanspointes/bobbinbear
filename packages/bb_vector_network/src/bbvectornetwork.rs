#![allow(dead_code)]

use std::ops::Add;

/// BBVectorNetwork:
///
/// This struct represents a Figma style Vector Network.
///
/// TODO:
///  - Implement `link/quad/cube_from_to` methods for closing a region.
use glam::Vec2;

use crate::{
    bbindex::{BBAnchorIndex, BBLinkIndex},
    bbvnlink::BBVNLink,
    bbvnregion::BBVNRegion,
};

#[derive(Debug, Clone)]
pub struct BBVectorNetwork {
    pub anchors: Vec<Vec2>,
    pub links: Vec<BBVNLink>,
    pub regions: Vec<BBVNRegion>,
}

impl BBVectorNetwork {
    pub fn anchor_unchecked(&self, index: BBAnchorIndex) -> Vec2 {
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
    fn link_line(&mut self, start: BBAnchorIndex, end: BBAnchorIndex) -> BBLinkIndex {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));
        self.links.push(BBVNLink::Line { start, end });

        (self.links.len() - 1).into()
    }

    /// Links two anchor nodes as a quadratic curve with 1 control node.
    ///
    /// * `start`:
    /// * `end`:
    fn link_quadratic(
        &mut self,
        start: BBAnchorIndex,
        ctrl1: Vec2,
        end: BBAnchorIndex,
    ) -> BBLinkIndex {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));
        self.links.push(BBVNLink::Quadratic { start, ctrl1, end });
        (self.links.len() - 1).into()
    }

    /// Links two anchor nodes as a cubic curve with 2 control node.
    ///
    /// * `start`:
    /// * `end`:
    fn link_cubic(
        &mut self,
        start: BBAnchorIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBAnchorIndex,
    ) -> BBLinkIndex {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));
        self.links.push(BBVNLink::Cubic {
            start,
            ctrl1,
            ctrl2,
            end,
        });
        (self.links.len() - 1).into()
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

    /// Creates a new BBVectorNetwork starting at a given point.
    ///
    /// * `begin_point`: Position of first point
    pub fn new() -> Self {
        Self {
            anchors: vec![],
            links: vec![],
            regions: vec![],
        }
    }

    pub fn link(&self, index: BBLinkIndex) -> Option<&BBVNLink> {
        self.links.get(index.0)
    }

    /// Gets a reference to an anchor
    pub fn anchor(&self, index: BBAnchorIndex) -> Option<&Vec2> {
        self.anchors.get(index.0)
    }

    /// Gets a mutable reference to an anchor
    pub fn anchor_mut(&mut self, index: BBAnchorIndex) -> Option<&mut Vec2> {
        self.anchors.get_mut(index.0)
    }

    pub fn anchor_len(&self) -> usize {
        self.anchors.len()
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
    pub fn cubic_from(
        &mut self,
        start: BBAnchorIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        to: Vec2,
    ) -> BBAnchorIndex {
        debug_assert!(self.has_anchor(start));

        let end = self.push_anchor(to);
        self.link_cubic(start, ctrl1, ctrl2, end);
        end
    }

    /// Creates a line, using new anchor points, from start -> end.
    ///
    /// * `start`:
    /// * `end`:
    pub fn line(&mut self, start: Vec2, end: Vec2) -> BBAnchorIndex {
        let start_index = self.push_anchor(start);
        self.line_from(start_index, end)
    }
    /// Creates a quadratic curve, using new anchor points, from start -> end.
    ///
    /// * `start`:
    /// * `end`:
    pub fn quadratic(&mut self, start: Vec2, ctrl1: Vec2, end: Vec2) -> BBAnchorIndex {
        let start_index = self.push_anchor(start);
        self.quadratic_from(start_index, ctrl1, end)
    }
    /// Creates a cubic curve, using new anchor points, from start -> end.
    ///
    /// * `start`:
    /// * `end`:
    pub fn cubic(&mut self, start: Vec2, ctrl1: Vec2, ctrl2: Vec2, end: Vec2) -> BBAnchorIndex {
        let start_index = self.push_anchor(start);
        self.cubic_from(start_index, ctrl1, ctrl2, end)
    }

    /// Creates a line from a new anchor point to a prexisting anchor point.
    ///
    /// * `start`:
    /// * `end`:
    pub fn line_to(&mut self, start: Vec2, end: BBAnchorIndex) -> BBAnchorIndex {
        let start_index = self.push_anchor(start);
        self.line_from_to(start_index, end);
        end
    }
    /// Creates a quadratic curve from a new anchor point to a prexisting anchor point.
    ///
    /// * `start`:
    /// * `end`:
    pub fn quadratic_to(&mut self, start: Vec2, ctrl1: Vec2, end: BBAnchorIndex) -> BBAnchorIndex {
        let start_index = self.push_anchor(start);
        self.quadratic_from_to(start_index, ctrl1, end);
        end
    }
    /// Creates a cubic curve from a new anchor point to a prexisting anchor point.
    ///
    /// * `start`:
    /// * `end`:
    pub fn cubic_to(
        &mut self,
        start: Vec2,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBAnchorIndex,
    ) -> BBAnchorIndex {
        let start_index = self.push_anchor(start);
        self.cubic_from_to(start_index, ctrl1, ctrl2, end);
        end
    }

    /// Adds a direct line from `start` to `end`, rebuilding shapes as needed.
    pub fn line_from_to(&mut self, start: BBAnchorIndex, end: BBAnchorIndex) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));

        let link_index = self.link_line(start, end);

        // TODO: Delete effected regions
        let region = BBVNRegion::from_link(self, link_index);
        self.regions.push(region);
    }
    /// Adds a quadratic curve from `start` to `end`, rebuilding shapes as needed.
    pub fn quadratic_from_to(&mut self, start: BBAnchorIndex, ctrl1: Vec2, end: BBAnchorIndex) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));

        let link_index = self.link_quadratic(start, ctrl1, end);

        // TODO: Delete effected regions
        let region = BBVNRegion::from_link(self, link_index);
        self.regions.push(region);
    }
    /// Adds a cubic curve from `start` to `end`, rebuilding shapes as needed.
    pub fn cubic_from_to(
        &mut self,
        start: BBAnchorIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBAnchorIndex,
    ) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));

        let link_index = self.link_cubic(start, ctrl1, ctrl2, end);

        // TODO: Delete effected regions
        let region = BBVNRegion::from_link(self, link_index);
        self.regions.push(region);
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
                link.deincrement_anchor_index_over_value(index);
            }
        }

        for link_index in links_to_delete {
            self.links.remove(link_index);
        }

        // TODO delete regions.
    }

    #[cfg(feature = "debug_draw")]
    pub fn debug_draw(&self) {
        for link in self.links.iter() {
            link.debug_draw(self);
        }

        for anchor in self.anchors.iter() {
            comfy::draw_circle(*anchor, 0.1, comfy::Color::rgb8(255, 0, 0), 1);
        }
    }

    pub fn translate(&mut self, translation: Vec2) {
        for v in self.anchors.iter_mut() {
            *v = v.add(translation);
        }
        for l in self.links.iter_mut() {
            l.translate(translation);
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::bbindex::BBAnchorIndex;

    use super::BBVectorNetwork;

    fn build_basic_triangle() -> BBVectorNetwork {
        let mut bbvn = BBVectorNetwork::new();
        let start = bbvn.anchor_len();
        let prev = bbvn.line(Vec2::new(10., 10.), Vec2::new(30., 30.));
        let prev = bbvn.line_from(prev, Vec2::new(10., 60.));
        bbvn.line_from_to(prev, start.into());

        bbvn
    }

    #[test]
    fn makes_triangle() {
        let bbvn = build_basic_triangle();
        println!("makes_triangle() bbvn: \n{bbvn:?}\n\n");
        assert_eq!(bbvn.anchor_len(), 3);
        assert_eq!(bbvn.regions.len(), 1);
    }

    #[test]
    fn makes_three_prong() {
        let mut bbvn = BBVectorNetwork::new();
        let prev_center = bbvn.line(Vec2::new(0., 0.), Vec2::new(0., 10.));
        bbvn.line_from(prev_center, Vec2::new(0., 20.));
        bbvn.line_from(prev_center, Vec2::new(10., 10.));
        println!("makes_three_prong() bbvn: \n{bbvn:?}\n\n");
        assert_eq!(bbvn.anchor_len(), 4);
        assert_eq!(bbvn.regions.len(), 0);
    }

    #[test]
    fn adds_tag_to_triangle() {
        let mut bbvn = build_basic_triangle();
        bbvn.line_from(BBAnchorIndex(2), Vec2::new(30., 60.));
        println!("makes_three_prong() bbvn: \n{bbvn:?}\n\n");
        assert_eq!(bbvn.anchor_len(), 4);
        assert_eq!(bbvn.regions.len(), 1);
    }

    #[test]
    fn adds_triangle_after_tag() {
        let mut bbvn = BBVectorNetwork::new();
        let p0 = bbvn.anchor_len();
        let p2 = bbvn.line(Vec2::new(10., 10.), Vec2::new(30., 30.));
        bbvn.line_from(p2, Vec2::new(30., 50.));

        let p4 = bbvn.line_from(p2, Vec2::new(10., 60.));
        bbvn.line_from_to(p4, p0.into());
        println!("makes_three_prong() bbvn: \n{bbvn:?}\n\n");
        assert_eq!(bbvn.anchor_len(), 4);
        assert_eq!(bbvn.regions.len(), 1);
    }

    #[test]
    fn makes_square_then_adds_square() {
        let mut bbvn = BBVectorNetwork::new();
        // Make the first box.
        let prev = bbvn.line(Vec2::new(10., 10.), Vec2::new(10., 20.));
        let p0 = bbvn.line_from(prev, Vec2::new(20., 20.));
        let p1 = bbvn.line_from(p0, Vec2::new(20., 10.));
        bbvn.line_from_to(p1, BBAnchorIndex(0));

        assert_eq!(bbvn.anchor_len(), 4);
        assert_eq!(bbvn.regions.len(), 1);

        let prev = bbvn.line_from(p0, Vec2::new(30., 20.));
        let prev = bbvn.line_from(prev, Vec2::new(30., 10.));
        bbvn.line_from_to(prev, p1);
        assert_eq!(bbvn.anchor_len(), 6);
        assert_eq!(bbvn.regions.len(), 2);
        println!("makes_square_then_adds_square() bbvn: \n{bbvn:?}\n\n");
    }
}
