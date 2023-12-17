#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Sub};

/// BBVectorNetwork:
///
/// This struct represents a Figma style Vector Network.
///
/// TODO:
///  - Implement `link/quad/cube_from_to` methods for closing a region.
use glam::Vec2;

use crate::traits::Determinate;
use crate::{
    bbanchor::{BBAnchor, BBAnchorIndex},
    bbindex::{BBLinkIndex, BBRegionIndex},
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

const MIN_EDGES_FOR_CYCLE: usize = 3;

impl BBVectorNetwork {
    /// Creates a new, empty BBVectorNetwork.
    pub fn new() -> Self {
        Self {
            next_idx: 0,
            anchors: vec![],
            links: HashMap::new(),
            regions: HashMap::new(),
        }
    }

    // Creates a new BBVectorNetwork from the links of a pre-existing BBVectorNetwork.
    // WARN: BBAnchorIndex/BBLinkIndex do not transfer between vector networks.
    pub fn new_from_preexisting_links(
        bbvn: &BBVectorNetwork,
        link_indices: &[BBLinkIndex],
    ) -> Self {
        let mut anchors = vec![];
        let mut links: Vec<BBVNLink> = Vec::with_capacity(link_indices.len());

        for link_idx in link_indices {
            let mut link = bbvn.link(*link_idx).unwrap().clone();
            if let Some(start_idx) = anchors.iter().find(|idx| **idx == link.start_index()) {
                link.set_start_index(*start_idx);
            } else {
                anchors.push(link.start_index());
                link.set_start_index(anchors.len().into());
            }

            if let Some(end_idx) = anchors.iter().find(|idx| **idx == link.end_index()) {
                link.set_end_index(*end_idx);
            } else {
                anchors.push(link.end_index());
                link.set_end_index(anchors.len().into());
            }
            links.push(link);
        }
        let next_idx = links.len();
        let links = HashMap::from_iter(
            links
                .into_iter()
                .enumerate()
                .map(|(idx, link)| (idx.into(), link)),
        );

        Self {
            next_idx,
            anchors: anchors
                .into_iter()
                .map(|idx| bbvn.anchor(idx).unwrap().clone())
                .collect(),
            links,
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

/**
* CCW / CW most link of a given anchor
*/
impl BBVectorNetwork {
    // Given an anchor and a current direction, finds the most counter-clockwise link in the graph edge
    // (ignoring visited_links).
    pub fn anchor_ccw_link(
        &self,
        idx: BBAnchorIndex,
        curr_dir: Vec2,
        visited_links: &Vec<BBLinkIndex>,
    ) -> Option<BBLinkIndex> {
        let anchor = self.anchor(idx).unwrap();
        let mut next_link_dirs: Vec<_> = anchor.adjacents()
            .iter()
            .filter_map(|link_index| {
                let link = self.link(*link_index).unwrap_or_else(|| {
                    panic!("BBVNRegion::from_link(..) Trying to get link {link_index:?} but not found.")
                });
                if visited_links.contains(link_index) {
                    return None;
                }
                // Reverse links that are facing the wrong way

                let link = if link.end_index() == idx {
                    link.reversed()
                } else {
                    *link
                };
                let tangent = link.calc_start_tangent(self);
                Some((*link_index, link, tangent))
            })
            .collect();

        let curr_p = anchor.position();

        let Some((mut next_index, mut next_link, mut next_dir)) = next_link_dirs.pop() else {
            return None;
        };

        for (el_index, el_link, el_dir) in next_link_dirs.into_iter() {
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
                    temp_el_dir = el_link.t_point(self, t).sub(curr_p);
                    temp_next_dir = next_link.t_point(self, t).sub(curr_p);
                    continue;
                }

                let is_convex = temp_next_dir.determinate(curr_dir) < 0.;
                let ccw_of_curr = curr_dir.determinate(temp_el_dir) > 0.;
                let ccw_of_next = temp_next_dir.determinate(temp_el_dir) > 0.;

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
    // Given an anchor and a current direction, finds the most clockwise link in the graph edge
    // (ignoring visited_links).
    pub fn anchor_cw_link(
        &self,
        idx: BBAnchorIndex,
        curr_dir: Vec2,
        visited_links: &Vec<BBLinkIndex>,
    ) -> Option<BBLinkIndex> {
        let anchor = self.anchor(idx).unwrap();
        let mut next_link_dirs: Vec<_> = anchor.adjacents()
            .iter()
            .filter_map(|link_index| {
                let link = self.link(*link_index).unwrap_or_else(|| {
                    panic!("BBVNRegion::from_link(..) Trying to get link {link_index:?} but not found.")
                });
                if visited_links.contains(link_index) {
                    return None;
                }
                // Reverse links that are facing the wrong way

                let link = if link.end_index() == idx {
                    link.reversed()
                } else {
                    *link
                };
                let tangent = link.calc_start_tangent(self);
                Some((*link_index, link, tangent))
            })
            .collect();

        let curr_p = anchor.position();

        let Some((mut next_index, mut next_link, mut next_dir)) = next_link_dirs.pop() else {
            return None;
        };

        for (el_index, el_link, el_dir) in next_link_dirs.into_iter() {
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
                    temp_el_dir = el_link.t_point(self, t).sub(curr_p);
                    temp_next_dir = next_link.t_point(self, t).sub(curr_p);
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
}

/**
 * MCB / Traversal
 */
impl BBVectorNetwork {
    /// Regenerates the regions struct within this BBVectorNetwork
    pub fn update_regions(&mut self) {
        let mut graph = self.clone();

        graph.remove_filaments();
    }

    pub fn mcb(&self, regions_out: &mut Vec<BBVNRegion>) {
        let graphs = self.get_detached_graphs();

        for mut graph in graphs {
            while graph.anchors.len() > 0 {
                graph.remove_filaments();

                let Some(a_first) = graph.get_left_most_anchor_index() else {
                    break;
                };
                let Some((outer_edge, closed_walk)) = graph.closed_walk(a_first) else {
                    break;
                };

                let graph = BBVectorNetwork::new_from_preexisting_links(self, &closed_walk);
            }
        }
    }

    fn extract_nested_from_closed_walk(&self, closed_walk: &mut Vec<BBLinkIndex>) {
        // Check nested cycles
        let a_first = self
            .link(*closed_walk.first().unwrap())
            .unwrap()
            .start_index();
        let mut anchors = vec![a_first];
        for link_idx in closed_walk.iter() {
            let link = self.link(*link_idx).unwrap();
            let next_anchor = (*link).other_anchor_index(*anchors.last().unwrap());
            anchors.push(next_anchor);
        }

        let mut i = 1;
        while i < closed_walk.len() {
            let is_first_or_last = i == 1 || i == anchors.len();
            let el_link = self.link(closed_walk[i]).unwrap();
            if let Some(end_i) = closed_walk[i..].iter().position(|v| v == &closed_walk[i]) {
                let diff = end_i - i;
                i -= diff;
            }
            i += 1;
        }
    }

    fn closed_walk(&self, a_first: BBAnchorIndex) -> Option<(BBLinkIndex, Vec<BBLinkIndex>)> {
        let mut closed_walk = vec![];
        let Some(first_link_idx) = self.anchor_cw_link(a_first, Vec2::new(0., -1.), &closed_walk)
        else {
            return None;
        };
        closed_walk.push(first_link_idx);
        let first_link = self.link(first_link_idx).unwrap();

        let mut l_idx_curr = first_link_idx;
        let mut l_curr = first_link;
        let mut a_curr = first_link.end_index();
        let mut d_curr = first_link.calc_end_tangent(self);

        while l_curr.end_index() != first_link.start_index() {
            let Some(next_link_idx) = self.anchor_cw_link(a_curr, d_curr, &closed_walk) else {
                panic!("mcb: No next link.")
            };
            l_idx_curr = next_link_idx;
            closed_walk.push(next_link_idx);

            let Some(next_link) = self.link(l_idx_curr) else {
                panic!("mcb: No {:?} in BBVN", l_idx_curr);
            };

            l_curr = next_link;
            a_curr = next_link.end_index();
            d_curr = next_link.calc_end_tangent(self);
        }

        if closed_walk.len() < MIN_EDGES_FOR_CYCLE {
            return None;
        } else {
            return Some((first_link_idx, closed_walk));
        }
    }
    /// Performs a breadth first search over the graph to return a Vec of each detached graph
    /// within it.
    pub fn get_detached_graphs(&self) -> Vec<BBVectorNetwork> {
        let mut result = vec![];

        // BBAnchorIndex -> Visited count
        let mut visited_anchors: HashMap<BBAnchorIndex, usize> = HashMap::new();
        for idx in 0..self.anchors.len() {
            visited_anchors.insert(BBAnchorIndex(idx), 0);
        }

        //
        while let Some(idx) = visited_anchors
            .iter()
            .find(|(idx, visited_count)| **visited_count == 0)
            .map(|(idx, _)| idx)
        {
            let mut queue: VecDeque<BBAnchorIndex> = VecDeque::new();
            let mut next_links: HashSet<BBLinkIndex> = HashSet::new();
            queue.push_back(*idx);
            while let Some(anchor_idx) = queue.pop_front() {
                let prev_visited = visited_anchors.get(&anchor_idx).expect(&format!(
                    "get_detached_graphs: Can't get visited node {:?}",
                    anchor_idx
                ));
                visited_anchors.insert(anchor_idx, prev_visited + 1);

                let anchor = self.anchor(anchor_idx).expect(&format!(
                    "get_detached_graphs: Can't get anchor {:?}.",
                    anchor_idx
                ));

                for link_idx in anchor.adjacents().iter() {
                    next_links.insert(*link_idx);

                    let link = self
                        .link(*link_idx)
                        .expect(&format!("get_detached_graphs: Can't get link index"));
                    let other_anchor_idx = link.other_anchor_index(anchor_idx);
                    queue.push_back(other_anchor_idx);
                }
            }

            let next_links: Vec<_> = next_links.into_iter().collect();

            let graph = BBVectorNetwork::new_from_preexisting_links(self, &next_links[..]);
            result.push(graph);
        }

        result
    }

    fn remove_filaments(&mut self) {
        let filament_anchors: Vec<_> = self
            .anchors
            .iter()
            .enumerate()
            .filter_map(|(i, anchor)| {
                if anchor.adjacents.len() == 1 {
                    Some(BBAnchorIndex(i))
                } else {
                    None
                }
            })
            .collect();

        for anchor_index in filament_anchors {
            self.delete_anchor(anchor_index);
        }
    }
    fn get_left_most_anchor_index(&self) -> Option<BBAnchorIndex> {
        let Some(mut result_pos) = self.anchors.first().map(|a| a.position) else {
            return None;
        };
        let mut result_index = BBAnchorIndex(0);
        for (i, anchor) in self.anchors.iter().enumerate() {
            if result_pos.x < anchor.position.x {
                result_pos = anchor.position;
                result_index = BBAnchorIndex(i);
            }
        }

        Some(result_index)
    }
}
