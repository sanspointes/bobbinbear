#![allow(dead_code)]

use glam::Vec2;

use crate::bbindex::BBAnchorIndex;

#[derive(Clone, Copy)]
pub(super) enum BBVNLink {
    Begin {
        at: BBAnchorIndex,
    },
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
    pub fn start_index(&self) -> BBAnchorIndex {
        match self {
            BBVNLink::Begin { at } => *at,
            BBVNLink::Line { start, .. }
            | BBVNLink::Quadratic { start, .. }
            | BBVNLink::Cubic { start, .. } => *start,
        }
    }
    pub fn end_index(&self) -> BBAnchorIndex {
        match self {
            BBVNLink::Begin { at } => *at,
            BBVNLink::Line { end, .. }
            | BBVNLink::Quadratic { end, .. }
            | BBVNLink::Cubic { end, .. } => *end,
        }
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
            BBVNLink::Begin { at } => {
                if *at > index {
                    *at -= 1;
                }
            }
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
