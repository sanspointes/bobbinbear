#![allow(dead_code)]

pub enum BBVNWindingRule {
    Default,
    NonZero,
}

/// Represents a filled region of the vector network.
///
/// * `winding_rule`: 
/// * `loops`: 
pub struct BBVNRegion {
    winding_rule: BBVNWindingRule,
    loops: Vec<Vec<usize>>,
}

impl BBVNRegion {
    /// Checks if a region contains an anchor index.
    ///
    /// * `index`: 
    fn contains_anchor(&self, index: usize) -> bool {
        for region_loop in &self.loops {
            for anchor_index in region_loop {
                if *anchor_index == index {
                    return true;
                }
            }
        }
        false
    }
}
