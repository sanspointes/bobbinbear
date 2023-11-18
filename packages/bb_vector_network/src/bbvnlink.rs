#![allow(dead_code)]

use std::ops::Sub;

use glam::Vec2;

use crate::{bbindex::{BBAnchorIndex, BBLinkIndex}, bbvectornetwork::BBVectorNetwork};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum BBVNLink {
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

    /// Gets the index of the `end` anchor
    pub fn end_index(&self) -> BBAnchorIndex {
        match self {
            BBVNLink::Line { end, .. }
            | BBVNLink::Quadratic { end, .. }
            | BBVNLink::Cubic { end, .. } => *end,
        }
    }

    /// Calculates the tangent of the bezier/line at `t=0`
    ///
    /// * `bbvn`: The BBVectorNetwork to source the point data from
    pub fn calc_start_tangent(&self, bbvn: &BBVectorNetwork) -> Vec2 {
        match self {
            BBVNLink::Line { start, end } => {
                let v_start = bbvn.anchor(*start).unwrap_or_else(|| panic!("BBVNLink: Missing start index ({start:?})."));
                let v_end = bbvn.anchor(*end).unwrap_or_else(|| panic!("BBVNLink: Missing end index ({end:?})."));
                
                v_end.sub(*v_start)
            }
            BBVNLink::Quadratic { start, ctrl1, .. } => {
                let v_start = bbvn.anchor(*start).unwrap_or_else(|| panic!("BBVNLink: Missing start index ({start:?})."));
                
                ctrl1.sub(*v_start)
            }
            BBVNLink::Cubic { start, ctrl1, .. } => {
                let v_start = bbvn.anchor(*start).unwrap_or_else(|| panic!("BBVNLink: Missing start index ({start:?})."));
                
                ctrl1.sub(*v_start)
            }
        }
    }

    /// Calculates the tangent of the bezier/line at `t=1`
    ///
    /// * `bbvn`: The BBVectorNetwork to source the point data from
    pub fn calc_end_tangent(&self, bbvn: &BBVectorNetwork) -> Vec2 {
        match self {
            BBVNLink::Line { start, end } => {
                let v_start = bbvn.anchor(*start).unwrap_or_else(|| panic!("BBVNLink: Missing start index ({start:?})."));
                let v_end = bbvn.anchor(*end).unwrap_or_else(|| panic!("BBVNLink: Missing end index ({end:?})."));

                v_end.sub(*v_start)
            }
            BBVNLink::Quadratic { end, ctrl1, .. } => {
                let v_end = bbvn.anchor(*end).unwrap_or_else(|| panic!("BBVNLink: Missing end index ({end:?})."));

                v_end.sub(*ctrl1)
            }
            BBVNLink::Cubic { end, ctrl1, .. } => {
                let v_end = bbvn.anchor(*end).unwrap_or_else(|| panic!("BBVNLink: Missing end index ({end:?})."));
                
                v_end.sub(*ctrl1)
            }
        }
    }

    /// Gets a Vec<BBLinkIndex> of links that follow from the current link.
    ///
    /// * `bbvn`: BBVectorNetwork to source the data from
    pub fn next_links(&self, bbvn: &BBVectorNetwork) -> Vec<BBLinkIndex> {
        let mut result = vec![];
        for (i, bbvn_link) in bbvn.links.iter().enumerate() {
            if bbvn_link.references_index(self.end_index()) && bbvn_link != self {
                result.push(i.into());
            }
        }
        result
    }

    /// Gets a Vec<BBLinkIndex> of links that lead into the current link.
    ///
    /// * `bbvn`: BBVectorNetwork to source the data from
    pub fn prev_links(&self, bbvn: &BBVectorNetwork) -> Vec<BBLinkIndex> {
        let mut result = vec![];
        for (i, bbvn_link) in bbvn.links.iter().enumerate() {
            if bbvn_link.references_index(self.start_index()) && bbvn_link != self {
                result.push(i.into());
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
    pub fn deincrement_anchor_index_over(&mut self, index: BBAnchorIndex) {
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
}
