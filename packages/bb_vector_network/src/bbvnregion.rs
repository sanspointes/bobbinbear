#![allow(dead_code)]

use crate::{
    bbindex::BBLinkIndex,
    bbvectornetwork::BBVectorNetwork,
    BBVNLink, bbanchor::BBAnchorIndex,
};

#[cfg(feature = "debug_draw")]
use crate::debug_draw::v2_p2;

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
    /// TODO: Make return Result<Self, ErrorType>
    ///
    /// * `bbvn`:
    /// * `bbvn_link`:
    pub(crate) fn try_from_link(bbvn: &BBVectorNetwork, link_index: BBLinkIndex) -> Option<Self> {
        // Stores (link_index, Vec<next_links>)
        let mut curr_iter = 0;
        let mut link_stack: Vec<(BBLinkIndex, Vec<BBLinkIndex>)> = vec![];
        let mut visited_links: Vec<BBLinkIndex> = vec![];
        #[allow(unused_assignments)]
        let mut curr_link_index = link_index;
        let mut curr_link = bbvn.link(link_index).unwrap_or_else(|| panic!("BBVNRegion::from_link(link_index: {link_index:?}) - Link for `link_index` does not exist."));
        let start_link_index = link_index;

        // Traverse the vector network, trying to find Minimal Cycle Basis to reconnect with start.
        while curr_iter <= MCB_MAX_ITERS {
            curr_iter += 1;
            let next_links = curr_link.next_links(bbvn);

            // If current link reconnects with start link, close the loop
            if next_links.contains(&start_link_index) {
                // WARN: Not getting the actual next links because I don't think it's needed.
                link_stack.push((start_link_index, curr_link.next_links(bbvn).clone()));
                break;
            }

            // Calculate the directions of all next links
            let next_links: Vec<_> = next_links
                .iter()
                .filter(|link_index| !visited_links.contains(link_index))
                .cloned()
                .collect();

            // If there are no valid links to continue to, we walk back the graph traversal.
            let Some(ccw_most_link) = curr_link.ccw_most_next_link(bbvn, &next_links) else {
                loop {
                    let Some(prev_link) = link_stack.pop() else {
                        return None;
                    };
                    // If we've visited all next_links of previous link, step back again.
                    if let Some(link_index) = prev_link
                        .1
                        .iter()
                        .find(|link_index| !visited_links.contains(link_index))
                    {
                        curr_link_index = *link_index;
                        curr_link = bbvn.link(curr_link_index).unwrap_or_else(|| panic!("BBVNRegion::from_link(link_index: {link_index:?}) - Link for `link_index` does not exist."));
                        break;
                    }
                }
                continue;
            };

            // Traverse to the next link
            curr_link_index = ccw_most_link;
            curr_link = bbvn.link(curr_link_index).unwrap_or_else(|| panic!("BBVNRegion::from_link(link_index: {link_index:?}) - Link for `link_index` does not exist."));
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

        Some(Self {
            loops: vec![curr_loop],
            winding_rule: BBVNWindingRule::NonZero,
        })
    }

    pub fn links(&self, bbvn: &BBVectorNetwork) -> Vec<Vec<BBVNLink>> {
        self.loops
            .iter()
            .map(|el| {
                el.iter()
                    .map(|index| {
                        let v = *bbvn
                            .link(*index)
                            .expect("BBVNRegion.links() -> Could not get link for {index:?}");
                        v
                    })
                    .collect()
            })
            .collect()
    }
    pub fn link_indicies(&self) -> Vec<Vec<BBLinkIndex>> {
        self.loops.clone()
    }

    /// Checks if a region contains an anchor index.
    ///
    /// * `index`:
    pub fn contains_anchor(&self, bbvn: &BBVectorNetwork, index: BBAnchorIndex) -> bool {
        for region_loop in &self.loops {
            for link_index in region_loop {
                let link = bbvn.link(*link_index).unwrap_or_else(|| panic!("BBVNRegion::contains_anchor(index: {index:?}) - Could not get link for link_index: {link_index:?}"));
                if link.references_index(index) {
                    return true;
                }
            }
        }
        false
    }

    pub fn contains_link(&self, link: BBLinkIndex) -> bool {
        for region_loop in &self.loops {
            for link_index in region_loop {
                if link == *link_index {
                    return true;
                }
            }
        }
        false
    }

    #[cfg(feature = "debug_draw")]
    pub fn debug_draw(&self, bbvn: &BBVectorNetwork) {
        use comfy::SpriteVertex;
        use glam::Vec3;
        use lyon_tessellation::{BuffersBuilder, FillOptions, FillVertex, VertexBuffers};

        for el in self.loops.iter() {
            let mut builder = lyon_path::Path::builder();
            let first_index = el
                .first()
                .expect("BBVNRegion.debug_draw() -> Can't get first link_index");
            let first_link = bbvn
                .link(*first_index)
                .expect("BBVNRegion.debug_draw() -> Can't get link {first_index:?}");
            builder.begin(v2_p2(first_link.start_pos(bbvn)));

            for link_index in el {
                let link = bbvn
                    .link(*link_index)
                    .expect("BBVNRegion.debug_draw() -> Can't get link {link_index:?}");
                match link {
                    BBVNLink::Line { .. } => {
                        builder.line_to(v2_p2(link.end_pos(bbvn)));
                    }
                    BBVNLink::Quadratic { ctrl1, .. } => {
                        builder.quadratic_bezier_to(v2_p2(*ctrl1), v2_p2(link.end_pos(bbvn)));
                    }
                    BBVNLink::Cubic { ctrl1, ctrl2, .. } => {
                        builder.cubic_bezier_to(
                            v2_p2(*ctrl1),
                            v2_p2(*ctrl2),
                            v2_p2(link.end_pos(bbvn)),
                        );
                    }
                }

                if link.end_index() == first_link.start_index() {
                    builder.end(true);
                }
            }

            let path = builder.build();

            let mut tess = lyon_tessellation::FillTessellator::new();
            let mut buffers = VertexBuffers::<comfy::SpriteVertex, u32>::new();
            let _ = tess
                .tessellate_path(
                    &path,
                    &FillOptions::default(),
                    &mut BuffersBuilder::new(&mut buffers, |vertex: FillVertex| {
                        return SpriteVertex::new(
                            Vec3::new(vertex.position().x, vertex.position().y, 0.),
                            glam::Vec2::ZERO,
                            comfy::BLUE.alpha(0.5),
                        );
                    }),
                )
                .map_err(|reason| panic!("BBVNRegion.debug_draw() -> {reason:?}"));

            comfy::draw_mesh(comfy::Mesh {
                vertices: buffers.vertices.into(),
                indices: buffers.indices.into(),
                texture: None,
                z_index: 5,
            })
        }
    }
}
