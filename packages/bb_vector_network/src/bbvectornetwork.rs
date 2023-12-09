#![allow(dead_code)]

use std::collections::HashMap;
use std::ops::Add;

/// BBVectorNetwork:
///
/// This struct represents a Figma style Vector Network.
///
/// TODO:
///  - Implement `link/quad/cube_from_to` methods for closing a region.
use glam::Vec2;

use crate::{
    bbanchor::BBAnchor,
    bbindex::{BBAnchorIndex, BBLinkIndex, BBRegionIndex},
    bbvnlink::BBVNLink,
    bbvnregion::BBVNRegion,
};

#[derive(Debug, Clone)]
pub struct BBVectorNetwork {
    next_idx: usize,
    pub anchors: Vec<BBAnchor>,
    pub links: std::collections::HashMap<BBLinkIndex, BBVNLink>,
    pub regions: std::collections::HashMap<BBRegionIndex, BBVNRegion>,
}

impl BBVectorNetwork {
    /// Creates a new BBVectorNetwork starting at a given point.
    ///
    /// * `begin_point`: Position of first point
    pub fn new() -> Self {
        Self {
            next_idx: 0,
            anchors: vec![],
            links: HashMap::new(),
            regions: HashMap::new(),
        }
    }

    fn get_next_idx(&mut self) -> usize {
        let v = self.next_idx;
        self.next_idx += 1;
        v
    }

    /*
     * Public Getters
     */

    /// Gets a reference to a Vector Network link between two anchors
    pub fn link(&self, index: BBLinkIndex) -> Option<&BBVNLink> {
        self.links.get(&index)
    }
    /// Gets a reference to an anchor
    pub fn anchor(&self, index: BBAnchorIndex) -> Option<&BBAnchor> {
        self.anchors.get(index.0)
    }
    /// Gets a mutable reference to an anchor
    pub fn anchor_mut(&mut self, index: BBAnchorIndex) -> Option<&mut BBAnchor> {
        self.anchors.get_mut(index.0)
    }
    /// Gets the number of anchors stored.
    pub fn anchor_len(&self) -> usize {
        self.anchors.len()
    }

    pub fn has_anchor(&self, index: BBAnchorIndex) -> bool {
        self.anchors.get(index.0).is_some()
    }

    /*
     * GRAPH BUILDING API - Anchor Methods
     */

    /// Pushes a new anchor node to the BBVectorNetwork
    fn add_anchor(&mut self, value: Vec2) -> BBAnchorIndex {
        self.anchors.push(BBAnchor::new(value));
        (self.anchors.len() - 1).into()
    }
    /// Deletes an anchor, deletes associated links and breaks regions containing these links.
    pub fn delete_anchor(&mut self, index: BBAnchorIndex) {
        debug_assert!(self.has_anchor(index));

        self.anchors.remove(index.0);

        let mut links_to_delete = vec![];
        for (link_index, link) in self.links.iter_mut() {
            if link.references_index(index) {
                links_to_delete.push(*link_index);
            } else {
                link.deincrement_anchor_index_over_value(index);
            }
        }

        for link_index in links_to_delete {
            self.links.remove(&link_index);
        }

        // TODO delete regions.
    }

    /*
     * GRAPH BUILDING API - Link functions
     */

    /// Links two anchor nodes as a straight line.
    fn link_line(&mut self, start: BBAnchorIndex, end: BBAnchorIndex) -> (BBLinkIndex, BBVNLink) {
        let index = BBLinkIndex(self.get_next_idx());
        self.anchor_mut(start)
            .expect(&format!(
                "BBVectorNetwork::link_line() Could not get {start:?} anchor."
            ))
            .adjacents
            .push(index);
        self.anchor_mut(end)
            .expect(&format!(
                "BBVectorNetwork::link_line() Could not get {end:?} anchor."
            ))
            .adjacents
            .push(index);

        let link = BBVNLink::Line { start, end };
        self.links.insert(index, link);

        (index, link)
    }
    /// Links two anchor nodes as a quadratic curve with 1 control node.
    fn link_quadratic(
        &mut self,
        start: BBAnchorIndex,
        ctrl1: Vec2,
        end: BBAnchorIndex,
    ) -> (BBLinkIndex, BBVNLink) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));
        let link = BBVNLink::Quadratic { start, ctrl1, end };
        let index = BBLinkIndex(self.get_next_idx());
        self.links.insert(index, link);
        (index, link)
    }
    /// Links two anchor nodes as a cubic curve with 2 control node.
    fn link_cubic(
        &mut self,
        start: BBAnchorIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBAnchorIndex,
    ) -> (BBLinkIndex, BBVNLink) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));
        let link = BBVNLink::Cubic {
            start,
            ctrl1,
            ctrl2,
            end,
        };
        let index = BBLinkIndex(self.get_next_idx());
        self.links.insert(index, link);
        (index, link)
    }

    /// Creates a line, using new anchor points, from start -> end.
    pub fn line(&mut self, start: Vec2, end: Vec2) -> (BBLinkIndex, BBVNLink) {
        let start_index = self.add_anchor(start);
        self.line_from(start_index, end)
    }
    /// Creates a quadratic curve, using new anchor points, from start -> end.
    pub fn quadratic(&mut self, start: Vec2, ctrl1: Vec2, end: Vec2) -> (BBLinkIndex, BBVNLink) {
        let start_index = self.add_anchor(start);
        self.quadratic_from(start_index, ctrl1, end)
    }
    /// Creates a cubic curve, using new anchor points, from start -> end.
    pub fn cubic(
        &mut self,
        start: Vec2,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: Vec2,
    ) -> (BBLinkIndex, BBVNLink) {
        let start_index = self.add_anchor(start);
        self.cubic_from(start_index, ctrl1, ctrl2, end)
    }
    /// Creates a line from a pre-existing point to a new point
    pub fn line_from(&mut self, start: BBAnchorIndex, to: Vec2) -> (BBLinkIndex, BBVNLink) {
        debug_assert!(self.has_anchor(start));

        let end = self.add_anchor(to);
        self.link_line(start, end)
    }
    /// Creates a quadratic curve from a pre-existing point to a new point
    pub fn quadratic_from(
        &mut self,
        start: BBAnchorIndex,
        ctrl1: Vec2,
        to: Vec2,
    ) -> (BBLinkIndex, BBVNLink) {
        debug_assert!(self.has_anchor(start));

        let end = self.add_anchor(to);
        self.link_quadratic(start, ctrl1, end)
    }

    /// Creates a cubic curve from a pre-existing point to a new point
    pub fn cubic_from(
        &mut self,
        start: BBAnchorIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        to: Vec2,
    ) -> (BBLinkIndex, BBVNLink) {
        debug_assert!(self.has_anchor(start));

        let end = self.add_anchor(to);
        self.link_cubic(start, ctrl1, ctrl2, end)
    }

    /// Creates a line from a new anchor point to a prexisting anchor point.
    pub fn line_to(&mut self, start: Vec2, end: BBAnchorIndex) -> (BBLinkIndex, BBVNLink) {
        let start_index = self.add_anchor(start);
        self.line_from_to(start_index, end)
    }
    /// Creates a quadratic curve from a new anchor point to a prexisting anchor point.
    pub fn quadratic_to(
        &mut self,
        start: Vec2,
        ctrl1: Vec2,
        end: BBAnchorIndex,
    ) -> (BBLinkIndex, BBVNLink) {
        let start_index = self.add_anchor(start);
        self.quadratic_from_to(start_index, ctrl1, end)
    }
    /// Creates a cubic curve from a new anchor point to a prexisting anchor point.
    pub fn cubic_to(
        &mut self,
        start: Vec2,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBAnchorIndex,
    ) -> (BBLinkIndex, BBVNLink) {
        let start_index = self.add_anchor(start);
        self.cubic_from_to(start_index, ctrl1, ctrl2, end)
    }
    /// Adds a direct line from `start` to `end`, rebuilding shapes as needed.
    pub fn line_from_to(
        &mut self,
        start: BBAnchorIndex,
        end: BBAnchorIndex,
    ) -> (BBLinkIndex, BBVNLink) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));

        self.link_line(start, end)
    }
    /// Adds a quadratic curve from `start` to `end`, rebuilding shapes as needed.
    pub fn quadratic_from_to(
        &mut self,
        start: BBAnchorIndex,
        ctrl1: Vec2,
        end: BBAnchorIndex,
    ) -> (BBLinkIndex, BBVNLink) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));

        self.link_quadratic(start, ctrl1, end)
    }
    /// Adds a cubic curve from `start` to `end`, rebuilding shapes as needed.
    pub fn cubic_from_to(
        &mut self,
        start: BBAnchorIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBAnchorIndex,
    ) -> (BBLinkIndex, BBVNLink) {
        debug_assert!(self.has_anchor(start));
        debug_assert!(self.has_anchor(end));

        self.link_cubic(start, ctrl1, ctrl2, end)
    }

    /// Regenerates the regions struct within this BBVectorNetwork
    // pub fn update_regions(&mut self) {
    // }

    #[cfg(feature = "debug_draw")]
    pub fn debug_draw(&self) {
        for (index, link) in self.links.iter() {
            link.debug_draw(self);
            comfy::draw_text(
                &format!("#{}", index.0),
                link.t_point(self, 0.5),
                comfy::WHITE,
                comfy::TextAlign::Center,
            );
        }

        for anchor in self.anchors.iter() {
            comfy::draw_circle(anchor.position(), 0.1, comfy::Color::rgb8(255, 0, 0), 1);
        }

        for (region_index, region) in self.regions.values().enumerate() {
            for el in region.link_indicies().iter() {
                println!("Loop {}: {:?}", region_index, el);
                for link_index in el.iter() {
                    let link = self.link(*link_index).expect(
                        "BBVectorNetwork::debug_draw() -> No link index for {link_index:?}",
                    );
                    let pos = link.t_point(self, 0.5);
                    comfy::draw_text(
                        &format!("#{}:{}", region_index, link_index.0),
                        pos + Vec2::new(0., 0.4 * (region_index + 1) as f32),
                        comfy::GRAY,
                        comfy::TextAlign::Center,
                    );
                }
            }
            // region.debug_draw(self);
        }
    }

    pub fn translate(&mut self, translation: Vec2) {
        for v in self.anchors.iter_mut() {
            v.position = v.position.add(translation);
        }
        for l in self.links.values_mut() {
            l.translate(translation);
        }
    }
}
