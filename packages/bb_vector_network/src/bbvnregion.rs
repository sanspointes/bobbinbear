#![allow(dead_code)]

use crate::{
    bbindex::{BBAnchorIndex, BBLinkIndex},
    bbvectornetwork::BBVectorNetwork,
};

/// Maximum search iterations for trying to generate a BBVNRegion using the "Minimal Cycle Basis"
/// method.
const MCB_MAX_ITERS: usize = 5000;

#[derive(Debug, Clone)]
pub enum BBVNWindingRule {
    Default,
    NonZero,
}

/// Represents a filled region of the vector network.
///
/// * `winding_rule`:
/// * `loops`:
#[derive(Debug, Clone)]
pub struct BBVNRegion {
    winding_rule: BBVNWindingRule,
    loops: Vec<Vec<BBLinkIndex>>,
}

impl BBVNRegion {
    /// Creates a new BBVNRegion within a BBVectorNetwork from a given BBVNLink using "Minimal Cycle
    /// Basis" method.
    /// This will assume that previous BBVNRegions (if any) will have been deleted already.
    ///
    /// * `bbvn`:
    /// * `bbvn_link`:
    pub(crate) fn from_link(bbvn: &BBVectorNetwork, link_index: BBLinkIndex) -> Self {
        // Stores (link_index, Vec<next_links>)
        let mut curr_iter = 0;
        let mut link_stack: Vec<(BBLinkIndex, Vec<BBLinkIndex>)> = vec![];
        let mut visited_links: Vec<BBLinkIndex> = vec![];
        #[allow(unused_assignments)]
        let mut curr_link_index = link_index;
        let mut curr_link = bbvn.links.get(link_index.0).unwrap_or_else(|| panic!("BBVNRegion::from_link(link_index: {link_index:?}) - Link for `link_index` does not exist."));
        let start_link_index = link_index;

        #[cfg(test)]
        {
            println!("BBVNRegion::from_link(link_index: {link_index:?})");
            println!("\tstart link: {:?}", start_link_index);
            println!("\tlinks: {:?}", bbvn.links);
        }

        // Traverse the vector network, trying to find Minimal Cycle Basis to reconnect with start.
        while curr_iter <= MCB_MAX_ITERS {
            curr_iter += 1;
            let next_links = curr_link.next_links(bbvn);

            #[cfg(test)]
            {
                let start_anchor = curr_link.start_index();
                let end_anchor = curr_link.end_index();
                println!("from_link - curr_iter: ({curr_iter:0>5}). link_index: {curr_link_index:?}.  start_anchor: {start_anchor:?}. end_anchor: {end_anchor:?}.");
                println!("\t - visited_links: {:?}", visited_links);
                println!(
                    "\t - link_stack: {:?}",
                    link_stack.iter().map(|(id, _)| id).collect::<Vec<_>>()
                );
                println!("\t - next_links: {:?}", next_links);
            }

            // If current link reconnects with start link, close the loop
            if next_links.contains(&start_link_index) {
                // WARN: Not getting the actual next links because I don't think it's needed.
                link_stack.push((start_link_index, curr_link.next_links(bbvn).clone()));
                break;
            }

            // Calculate the directions of all next links
            let next_links: Vec<_> = next_links.iter()
                .filter(|link_index| !visited_links.contains(link_index))
                .cloned()
                .collect();

            // If there are no valid links to continue to, we walk back the graph traversal.
            let Some(ccw_most_link) = curr_link.ccw_most_next_link(bbvn, &next_links) else {
                loop {
                    let Some(prev_link) = link_stack.pop() else {
                        panic!("Could not find a valid path back to start index.");
                    };
                    #[cfg(test)]
                    println!(
                        "\t\t -> Traversal step back to {:?} due to no good links...",
                        prev_link.0
                    );
                    // If we've visited all next_links of previous link, step back again.
                    if let Some(link_index) = prev_link
                        .1
                        .iter()
                        .find(|link_index| !visited_links.contains(link_index))
                    {
                        curr_link_index = *link_index;
                        curr_link = bbvn.links.get(curr_link_index.0).unwrap_or_else(|| panic!("BBVNRegion::from_link(link_index: {link_index:?}) - Link for `link_index` does not exist."));
                        break;
                    }
                }
                continue;
            };

            println!("\t -> Traversing to {ccw_most_link:?}");

            // Traverse to the next link
            curr_link_index = ccw_most_link;
            curr_link = bbvn.links.get(curr_link_index.0).unwrap_or_else(|| panic!("BBVNRegion::from_link(link_index: {link_index:?}) - Link for `link_index` does not exist."));
            link_stack.push((curr_link_index, curr_link.next_links(bbvn).clone()));
            if curr_iter != 0 {
                visited_links.push(link_index);
            }
        }

        // TODO: Maybe add enclave detection?  Not sure if necessary.
        let curr_loop: Vec<_> = link_stack
            .iter()
            .map(|(link_index, _)| link_index)
            .cloned()
            .collect();

        Self {
            loops: vec![curr_loop],
            winding_rule: BBVNWindingRule::NonZero,
        }
    }
    /// Checks if a region contains an anchor index.
    ///
    /// * `index`:
    fn contains_anchor(&self, bbvn: &BBVectorNetwork, index: BBAnchorIndex) -> bool {
        for region_loop in &self.loops {
            for link_index in region_loop {
                let link = bbvn.links.get(link_index.0).unwrap_or_else(|| panic!("BBVNRegion::contains_anchor(index: {index:?}) - Could not get link for link_index: {link_index:?}"));
                if link.references_index(index) {
                    return true;
                }
            }
        }
        false
    }

    fn contains_link(&self, link: BBLinkIndex) -> bool {
        for region_loop in &self.loops {
            for link_index in region_loop {
                if link == *link_index {
                    return true;
                }
            }
        }
        false
    }
}
