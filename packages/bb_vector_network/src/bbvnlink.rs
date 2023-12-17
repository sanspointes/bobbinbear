#![allow(dead_code)]

use std::ops::{Add, Sub};

use glam::Vec2;

use crate::{
    bbanchor::{BBAnchor, BBAnchorIndex},
    bbindex::BBLinkIndex,
    bbvectornetwork::BBVectorNetwork,
    traits::Determinate,
};

#[cfg(feature = "debug_draw")]
use crate::debug_draw::draw_det_arc;
#[cfg(feature = "debug_draw")]
use comfy::{draw_text, Vec2Extensions, ORANGE, ORANGE_RED, PURPLE};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BBVNLink {
    Line {
        start: BBAnchorIndex,
        end: BBAnchorIndex,
    },
    Quadratic {
        start: BBAnchorIndex,
        ctrl1: Vec2,
        end: BBAnchorIndex,
    },
    Cubic {
        start: BBAnchorIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBAnchorIndex,
    },
}

impl BBVNLink {
    /// Gets the index of the `start` anchor
    pub fn start_index(&self) -> BBAnchorIndex {
        match self {
            BBVNLink::Line { start, .. }
            | BBVNLink::Quadratic { start, .. }
            | BBVNLink::Cubic { start, .. } => *start,
        }
    }
    pub fn start(&self, bbvn: &BBVectorNetwork) -> BBAnchor {
        bbvn.anchor(self.start_index()).unwrap().clone()
    }
    pub fn start_pos(&self, bbvn: &BBVectorNetwork) -> Vec2 {
        self.start(bbvn).position()
    }

    /// Gets the index of the `end` anchor
    pub fn end_index(&self) -> BBAnchorIndex {
        match self {
            BBVNLink::Line { end, .. }
            | BBVNLink::Quadratic { end, .. }
            | BBVNLink::Cubic { end, .. } => *end,
        }
    }
    pub fn end(&self, bbvn: &BBVectorNetwork) -> BBAnchor {
        bbvn.anchor(self.end_index()).unwrap().clone()
    }
    pub fn end_pos(&self, bbvn: &BBVectorNetwork) -> Vec2 {
        self.end(bbvn).position()
    }

    pub fn set_start_index(&mut self, start_idx: BBAnchorIndex) {
        match self {
            BBVNLink::Line { start, .. }
            | BBVNLink::Quadratic { start, .. }
            | BBVNLink::Cubic { start, .. } => *start = start_idx,
        }
    }
    pub fn set_end_index(&mut self, end_idx: BBAnchorIndex) {
        match self {
            BBVNLink::Line { end, .. }
            | BBVNLink::Quadratic { end, .. }
            | BBVNLink::Cubic { end, .. } => *end = end_idx,
        }
    }

    /// Returns the index on the other side, i.e if you provide the start index it will return the
    /// end index and vice versa.
    pub fn other_anchor_index(&self, idx: BBAnchorIndex) -> BBAnchorIndex {
        match self {
            BBVNLink::Line { end, start }
            | BBVNLink::Quadratic { end, start, .. }
            | BBVNLink::Cubic { end, start, .. } => {
                    if idx == *end {
                        *start
                    } else {
                        *end
                    }
                }
        }
    }

    pub fn t_point(&self, bbvn: &BBVectorNetwork, t: f32) -> Vec2 {
        match self {
            BBVNLink::Line { .. } => Vec2::lerp(self.start_pos(bbvn), self.end_pos(bbvn), t),
            BBVNLink::Quadratic { ctrl1, .. } => {
                let p1 = Vec2::lerp(self.start_pos(bbvn), *ctrl1, t);
                let p2 = Vec2::lerp(*ctrl1, self.end_pos(bbvn), t);
                Vec2::lerp(p1, p2, t)
            }
            BBVNLink::Cubic { ctrl1, ctrl2, .. } => {
                let p1 = Vec2::lerp(self.start_pos(bbvn), *ctrl1, t);
                let p2 = Vec2::lerp(*ctrl1, *ctrl2, t);
                let p3 = Vec2::lerp(*ctrl2, self.end_pos(bbvn), t);
                Vec2::lerp(Vec2::lerp(p1, p2, t), Vec2::lerp(p2, p3, t), t)
            }
        }
    }

    /// Calculates the tangent of the bezier/line at `t=0`
    ///
    /// * `bbvn`: The BBVectorNetwork to source the point data from
    pub fn calc_start_tangent(&self, bbvn: &BBVectorNetwork) -> Vec2 {
        match self {
            BBVNLink::Line { start, end } => {
                let v_start = bbvn
                    .anchor(*start)
                    .unwrap_or_else(|| panic!("BBVNLink: Missing start index ({start:?})."))
                    .position();
                let v_end = bbvn
                    .anchor(*end)
                    .unwrap_or_else(|| panic!("BBVNLink: Missing end index ({end:?})."))
                    .position();

                v_end.sub(v_start)
            }
            BBVNLink::Quadratic { start, ctrl1, .. } => {
                let v_start = bbvn
                    .anchor(*start)
                    .unwrap_or_else(|| panic!("BBVNLink: Missing start index ({start:?})."))
                    .position();

                ctrl1.sub(v_start)
            }
            BBVNLink::Cubic { start, ctrl1, .. } => {
                let v_start = bbvn
                    .anchor(*start)
                    .unwrap_or_else(|| panic!("BBVNLink: Missing start index ({start:?})."))
                    .position();

                ctrl1.sub(v_start)
            }
        }
    }

    /// Calculates the tangent of the bezier/line at `t=1`
    ///
    /// * `bbvn`: The BBVectorNetwork to source the point data from
    pub fn calc_end_tangent(&self, bbvn: &BBVectorNetwork) -> Vec2 {
        match self {
            BBVNLink::Line { start, end } => {
                let v_start = bbvn
                    .anchor(*start)
                    .unwrap_or_else(|| panic!("BBVNLink: Missing start index ({start:?})."))
                    .position();
                let v_end = bbvn
                    .anchor(*end)
                    .unwrap_or_else(|| panic!("BBVNLink: Missing end index ({end:?})."))
                    .position();

                v_end.sub(v_start)
            }
            BBVNLink::Quadratic { end, ctrl1, .. } => {
                let v_end = bbvn
                    .anchor(*end)
                    .unwrap_or_else(|| panic!("BBVNLink: Missing end index ({end:?})."))
                    .position();

                v_end.sub(*ctrl1)
            }
            BBVNLink::Cubic { end, ctrl2, .. } => {
                let v_end = bbvn
                    .anchor(*end)
                    .unwrap_or_else(|| panic!("BBVNLink: Missing end index ({end:?})."))
                    .position();

                v_end.sub(*ctrl2)
            }
        }
    }

    /// Gets a Vec<BBLinkIndex> of links that follow from the current link.
    ///
    /// * `bbvn`: BBVectorNetwork to source the data from
    pub fn next_links(&self, bbvn: &BBVectorNetwork) -> Vec<BBLinkIndex> {
        let mut result = vec![];
        for (i, bbvn_link) in bbvn.links.iter() {
            if bbvn_link.references_index(self.end_index()) && bbvn_link != self {
                result.push(*i);
            }
        }
        result
    }

    /// Gets a Vec<BBLinkIndex> of links that lead into the current link.
    ///
    /// * `bbvn`: BBVectorNetwork to source the data from
    pub fn prev_links(&self, bbvn: &BBVectorNetwork) -> Vec<BBLinkIndex> {
        let mut result = vec![];
        for (i, bbvn_link) in bbvn.links.iter() {
            if bbvn_link.references_index(self.start_index()) && bbvn_link != self {
                result.push(*i);
            }
        }
        result
    }
    /// Returns true if this BBVNLink references the given index.
    ///
    /// * `index`:
    pub fn references_index(&self, index: BBAnchorIndex) -> bool {
        self.start_index() == index || self.end_index() == index
    }

    /// If this link references an anchor with an index greater than `index`, de-incremement the
    /// references.
    /// Used for when a node is deleted and the indexes need to be shifted so everything is linked
    /// correctly.
    pub fn deincrement_anchor_index_over_value(&mut self, index: BBAnchorIndex) {
        match self {
            BBVNLink::Line { start, end }
            | BBVNLink::Quadratic { start, end, .. }
            | BBVNLink::Cubic { start, end, .. } => {
                if *start > index {
                    *start -= 1;
                }
                if *end > index {
                    *end -= 1;
                }
            }
        }
    }

    /// Returns a clone of the BBVNLink but in the reversed direction
    pub fn reversed(&self) -> Self {
        match self {
            BBVNLink::Line { start, end } => BBVNLink::Line {
                start: *end,
                end: *start,
            },
            BBVNLink::Quadratic { start, ctrl1, end } => BBVNLink::Quadratic {
                start: *end,
                ctrl1: *ctrl1,
                end: *start,
            },
            BBVNLink::Cubic {
                start,
                ctrl1,
                ctrl2,
                end,
            } => BBVNLink::Cubic {
                start: *end,
                ctrl1: *ctrl2,
                ctrl2: *ctrl1,
                end: *start,
            },
        }
    }

    pub fn translate(&mut self, translation: Vec2) {
        match self {
            BBVNLink::Quadratic { ctrl1, .. } => {
                *ctrl1 = ctrl1.add(translation);
            }
            BBVNLink::Cubic { ctrl1, ctrl2, .. } => {
                *ctrl1 = ctrl1.add(translation);
                *ctrl2 = ctrl2.add(translation);
            }
            _ => (),
        }
    }

    /// Given a list of `next_links` find the clockwise most next link
    pub fn cw_most_next_link(
        &self,
        bbvn: &BBVectorNetwork,
        next_links: &[BBLinkIndex],
    ) -> Option<BBLinkIndex> {
        let mut next_link_dirs: Vec<_> = next_links
            .iter()
            .map(|link_index| {
                let link = bbvn.link(*link_index).unwrap_or_else(|| {
                    panic!("BBVNRegion::from_link(..) Trying to get link {link_index:?} but not found.")
                });
                // Reverse links that are facing the wrong way
                let link = if self.end_index() == link.end_index() || self.start_index() == link.start_index() {
                    link.reversed()
                } else {
                    *link
                };
                let tangent = link.calc_start_tangent(bbvn);
                (*link_index, link, tangent)
            })
            .collect();

        let curr_dir = self.calc_start_tangent(bbvn);
        let curr_p = self.start_pos(bbvn);

        let Some((mut next_index, mut next_link, mut next_dir)) = next_link_dirs.pop() else {
            return None;
        };

        for (i, (el_index, el_link, el_dir)) in next_link_dirs.into_iter().enumerate() {
            let mut temp_el_dir = el_dir;
            let mut temp_next_dir = next_dir;

            // #[cfg(feature = "debug_draw")]
            // draw_det_arc(self.end_pos(bbvn), 0.5 + (i as f32) * 0.5, curr_dir, el_dir, next_dir);

            // When lines a parallel we need to move our test points across the lines until we find
            // one that isn't parallel.  This loop starts at 0 but will iterate forward if there's
            // no good option.
            let mut t = 0.;
            loop {
                let is_parrallel = temp_el_dir.determinate(temp_next_dir).abs() < 0.01;
                if is_parrallel {
                    t = t + 1. / 32.;
                    temp_el_dir = el_link.t_point(bbvn, t).sub(curr_p);
                    temp_next_dir = next_link.t_point(bbvn, t).sub(curr_p);
                    continue;
                }

                let is_convex = temp_next_dir.determinate(curr_dir) > 0.;
                let ccw_of_curr = curr_dir.determinate(temp_el_dir) <= 0.;
                let ccw_of_next = temp_next_dir.determinate(temp_el_dir) <= 0.;

                if (!is_convex && ccw_of_curr && ccw_of_next)
                    || (is_convex && (ccw_of_curr || ccw_of_next))
                {
                    next_index = el_index;
                    next_link = el_link;
                    next_dir = temp_el_dir;
                }
                break;
            }
        }

        Some(next_index)
    }

    /// Given a list of `next_links` find the counter-clockwise most next link
    pub fn ccw_most_next_link(
        &self,
        bbvn: &BBVectorNetwork,
        next_links: &[BBLinkIndex],
    ) -> Option<BBLinkIndex> {
        let mut next_link_dirs: Vec<_> = next_links
            .iter()
            .map(|link_index| {
                let link = bbvn.link(*link_index).unwrap_or_else(|| {
                    panic!("BBVNRegion::from_link(..) Trying to get link {link_index:?} but not found.")
                });
                // Reverse links that are facing the wrong way
                let link = if self.end_index() == link.end_index() || self.start_index() == link.start_index() {
                    link.reversed()
                } else {
                    *link
                };
                let tangent = link.calc_start_tangent(bbvn);
                (*link_index, link, tangent)
            })
            .collect();

        let curr_dir = self.calc_start_tangent(bbvn);
        let curr_p = self.start_pos(bbvn);

        // CCW Most node
        // 1. Try find link that is to left of current dir, else take first link
        // 2. Iterate through links trying to find if a link is further to the left than the
        //    current left most link.

        let Some((mut next_index, mut next_link, mut next_dir)) = next_link_dirs.pop() else {
            return None;
        };

        for (i, (el_index, el_link, el_dir)) in next_link_dirs.into_iter().enumerate() {
            let mut temp_el_dir = el_dir;
            let mut temp_next_dir = next_dir;

            // #[cfg(feature = "debug_draw")]
            // draw_det_arc(self.end_pos(bbvn), 0.5 + (i as f32) * 0.5, curr_dir, el_dir, next_dir);

            // When lines a parallel we need to move our test points across the lines until we find
            // one that isn't parallel.  This loop starts at 0 but will iterate forward if there's
            // no good option.
            let mut t = 0.;
            loop {
                let is_parrallel = temp_el_dir.determinate(temp_next_dir).abs() < 0.01;
                if is_parrallel {
                    t = t + 1. / 32.;
                    temp_el_dir = el_link.t_point(bbvn, t).sub(curr_p);
                    temp_next_dir = next_link.t_point(bbvn, t).sub(curr_p);
                    continue;
                }

                let is_convex = temp_next_dir.determinate(curr_dir) > 0.;
                let ccw_of_curr = curr_dir.determinate(temp_el_dir) >= 0.;
                let ccw_of_next = temp_next_dir.determinate(temp_el_dir) >= 0.;

                if (!is_convex && ccw_of_curr && ccw_of_next)
                    || (is_convex && (ccw_of_curr || ccw_of_next))
                {
                    next_index = el_index;
                    next_link = el_link;
                    next_dir = temp_el_dir;
                }
                break;
            }
        }

        Some(next_index)
    }

    #[cfg(feature = "debug_draw")]
    pub fn debug_draw(&self, bbvn: &BBVectorNetwork) {
        self.debug_draw_with_color_and_z_index(bbvn, comfy::Color::rgb8(0, 255, 0), 10);
    }
    #[cfg(feature = "debug_draw")]
    pub fn debug_draw_with_color_and_z_index(
        &self,
        bbvn: &BBVectorNetwork,
        color: comfy::Color,
        z_index: i32,
    ) {
        match self {
            BBVNLink::Line { .. } => {
                let start_point = bbvn.anchor(self.start_index());
                let end_pos = bbvn.anchor(self.end_index());

                if let (Some(start), Some(end)) = (start_point, end_pos) {
                    comfy::draw_arrow(start.position, end.position, 0.03, color, z_index);
                }
            }
            link @ BBVNLink::Quadratic { .. } | link @ BBVNLink::Cubic { .. } => {
                let mut p_prev = self.start_pos(bbvn);
                for i in 0..20 {
                    let i = i + 1;
                    let t = i as f32 / 20.;
                    let p = self.t_point(bbvn, t);
                    if i == 20 {
                        comfy::draw_arrow(p_prev, p, 0.03, color, z_index);
                    } else {
                        comfy::draw_line(p_prev, p, 0.03, color, z_index);
                    }
                    p_prev = p;
                }

                match link {
                    BBVNLink::Quadratic { ctrl1, .. } => {
                        comfy::draw_line(self.start_pos(bbvn), *ctrl1, 0.025, color, z_index);
                        comfy::draw_line(*ctrl1, self.end_pos(bbvn), 0.025, color, z_index);
                    }
                    BBVNLink::Cubic { ctrl1, ctrl2, .. } => {
                        comfy::draw_line(self.start_pos(bbvn), *ctrl1, 0.025, color, z_index);
                        comfy::draw_line(*ctrl2, self.end_pos(bbvn), 0.025, color, z_index);
                    }
                    BBVNLink::Line { .. } => (),
                }
            }
        }

        let p = self.start_pos(bbvn);
        comfy::draw_line(
            p,
            p + self.calc_start_tangent(bbvn).normalize(),
            0.015,
            comfy::GRAY,
            z_index + 1,
        );
        let p = self.end_pos(bbvn);
        comfy::draw_line(
            p,
            p + self.calc_end_tangent(bbvn).normalize(),
            0.015,
            comfy::GRAY,
            z_index + 1,
        );
    }
}
