use std::ops::Sub;

use glam::Vec2;
use std::collections::HashMap;

use crate::traits::Determinate;

use super::{
    bb_edge::{BBEdge, BBEdgeIndex},
    bb_node::{BBNode, BBNodeIndex},
    errors::{BBError, BBResult},
    mcb::perform_closed_walk_from_node,
};

#[derive(Debug, Clone)]
pub struct BBGraph {
    next_idx: usize,
    pub nodes: Vec<BBNode>,
    pub edges: HashMap<BBEdgeIndex, BBEdge>,
    // pub regions: std::collections::HashMap<BBRegionIndex, BBVNRegion>,
}

impl BBGraph {
    pub fn new() -> Self {
        Self {
            next_idx: 0,
            nodes: vec![],
            edges: HashMap::new(),
        }
    }

    /// Generates a new BBGraph from the edges of another BBGraph
    ///
    /// * `other`:
    /// * `edges`:
    pub fn new_from_other_edges(other: &BBGraph, edge_indices: &[BBEdgeIndex]) -> Self {
        let mut nodes = vec![];
        let mut edges: Vec<BBEdge> = Vec::with_capacity(edge_indices.len());

        for edge_idx in edge_indices {
            let mut edge = other.edge(*edge_idx).unwrap().clone();
            if let Some(start_idx) = nodes.iter().find(|idx| **idx == edge.start_idx()) {
                edge.set_start_idx(*start_idx);
            } else {
                nodes.push(edge.start_idx());
                edge.set_start_idx(nodes.len().into());
            }

            if let Some(end_idx) = nodes.iter().find(|idx| **idx == edge.end_idx()) {
                edge.set_end_idx(*end_idx);
            } else {
                nodes.push(edge.end_idx());
                edge.set_end_idx(nodes.len().into());
            }
            edges.push(edge);
        }
        let next_idx = edges.len();
        let edges = HashMap::from_iter(
            edges
                .into_iter()
                .enumerate()
                .map(|(idx, edge)| (idx.into(), edge)),
        );

        Self {
            next_idx,
            nodes: nodes
                .into_iter()
                .map(|idx| other.node(idx).unwrap().clone())
                .collect(),
            edges,
            // regions: HashMap::new(),
        }
    }

    fn get_next_idx(&mut self) -> usize {
        let v = self.next_idx;
        self.next_idx += 1;
        v
    }
}

/**
 * Node/Edge getters and setters
 */
impl BBGraph {
    /// Gets a reference to a Vector Network edge between two nodes
    pub fn edge(&self, index: BBEdgeIndex) -> BBResult<&BBEdge> {
        self.edges.get(&index).ok_or(BBError::MissingEdge(index))
    }
    /// Gets a reference to an node
    pub fn node(&self, index: BBNodeIndex) -> BBResult<&BBNode> {
        self.nodes.get(index.0).ok_or(BBError::MissingNode(index))
    }
    /// Gets a mutable reference to an node
    pub fn node_mut(&mut self, index: BBNodeIndex) -> BBResult<&mut BBNode> {
        self.nodes
            .get_mut(index.0)
            .ok_or(BBError::MissingNode(index))
    }
    /// Gets the number of nodes stored.
    pub fn node_len(&self) -> usize {
        self.nodes.len()
    }

    pub fn has_node(&self, index: BBNodeIndex) -> bool {
        self.nodes.get(index.0).is_some()
    }
}

/**
 * Graph Building API
 */
impl BBGraph {
    /// Pushes a new node node to the BBVectorNetwork
    fn add_node(&mut self, value: Vec2) -> BBNodeIndex {
        self.nodes.push(BBNode::new(value));
        (self.nodes.len() - 1).into()
    }
    /// Deletes an node, deletes associated edges and breaks regions containing these edges.
    pub fn delete_node(&mut self, index: BBNodeIndex) {
        debug_assert!(self.has_node(index));

        self.nodes.remove(index.0);

        let mut edges_to_delete = vec![];
        for (edge_idx, edge) in self.edges.iter_mut() {
            if edge.references_idx(index) {
                edges_to_delete.push(*edge_idx);
            } else {
                match edge {
                    BBEdge::Line { start, end }
                    | BBEdge::Quadratic { start, end, .. }
                    | BBEdge::Cubic { start, end, .. } => {
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

        for edge_idx in edges_to_delete {
            self.edges.remove(&edge_idx);
        }

        // TODO delete regions.
    }

    fn add_edge(&mut self, edge: BBEdge) -> (BBEdgeIndex, BBEdge) {
        let index = BBEdgeIndex(self.get_next_idx());
        self.edges.insert(index, edge);
        self.node_mut(edge.start_idx())
            .unwrap()
            .adjacents
            .push(index);
        self.node_mut(edge.end_idx()).unwrap().adjacents.push(index);
        (index, edge)
    }

    fn delete_edge(&mut self, edge_idx: BBEdgeIndex) {
        let edge = *self.edge(edge_idx).unwrap();

        let start = self.node_mut(edge.start_idx()).unwrap();
        start.adjacents.retain(|e_idx| *e_idx != edge_idx);
        if start.adjacents().len() == 0 {
            self.delete_node(edge.start_idx());
        }

        let end = self.node_mut(edge.end_idx()).unwrap();
        end.adjacents.retain(|e_idx| *e_idx != edge_idx);
        if end.adjacents().len() == 0 {
            self.delete_node(edge.end_idx());
        }
    }

    /*
     * GRAPH BUILDING API - edge functions
     */

    /// edges two node nodes as a straight line.
    fn edge_line(&mut self, start: BBNodeIndex, end: BBNodeIndex) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));
        debug_assert!(self.has_node(end));
        let edge = BBEdge::Line { start, end };
        self.add_edge(edge)
    }
    /// edges two node nodes as a quadratic curve with 1 control node.
    fn edge_quadratic(
        &mut self,
        start: BBNodeIndex,
        ctrl1: Vec2,
        end: BBNodeIndex,
    ) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));
        debug_assert!(self.has_node(end));
        let edge = BBEdge::Quadratic { start, ctrl1, end };
        self.add_edge(edge)
    }
    /// edges two node nodes as a cubic curve with 2 control node.
    fn edge_cubic(
        &mut self,
        start: BBNodeIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBNodeIndex,
    ) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));
        debug_assert!(self.has_node(end));
        let edge = BBEdge::Cubic {
            start,
            ctrl1,
            ctrl2,
            end,
        };
        self.add_edge(edge)
    }

    /// Creates a line, using new node points, from start -> end.
    pub fn line(&mut self, start: Vec2, end: Vec2) -> (BBEdgeIndex, BBEdge) {
        let start_index = self.add_node(start);
        self.line_from(start_index, end)
    }
    /// Creates a quadratic curve, using new node points, from start -> end.
    pub fn quadratic(&mut self, start: Vec2, ctrl1: Vec2, end: Vec2) -> (BBEdgeIndex, BBEdge) {
        let start_index = self.add_node(start);
        self.quadratic_from(start_index, ctrl1, end)
    }
    /// Creates a cubic curve, using new node points, from start -> end.
    pub fn cubic(
        &mut self,
        start: Vec2,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: Vec2,
    ) -> (BBEdgeIndex, BBEdge) {
        let start_index = self.add_node(start);
        self.cubic_from(start_index, ctrl1, ctrl2, end)
    }
    /// Creates a line from a pre-existing point to a new point
    pub fn line_from(&mut self, start: BBNodeIndex, to: Vec2) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));

        let end = self.add_node(to);
        self.edge_line(start, end)
    }
    /// Creates a quadratic curve from a pre-existing point to a new point
    pub fn quadratic_from(
        &mut self,
        start: BBNodeIndex,
        ctrl1: Vec2,
        to: Vec2,
    ) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));

        let end = self.add_node(to);
        self.edge_quadratic(start, ctrl1, end)
    }

    /// Creates a cubic curve from a pre-existing point to a new point
    pub fn cubic_from(
        &mut self,
        start: BBNodeIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        to: Vec2,
    ) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));

        let end = self.add_node(to);
        self.edge_cubic(start, ctrl1, ctrl2, end)
    }

    /// Creates a line from a new node point to a prexisting node point.
    pub fn line_to(&mut self, start: Vec2, end: BBNodeIndex) -> (BBEdgeIndex, BBEdge) {
        let start_index = self.add_node(start);
        self.line_from_to(start_index, end)
    }
    /// Creates a quadratic curve from a new node point to a prexisting node point.
    pub fn quadratic_to(
        &mut self,
        start: Vec2,
        ctrl1: Vec2,
        end: BBNodeIndex,
    ) -> (BBEdgeIndex, BBEdge) {
        let start_index = self.add_node(start);
        self.quadratic_from_to(start_index, ctrl1, end)
    }
    /// Creates a cubic curve from a new node point to a prexisting node point.
    pub fn cubic_to(
        &mut self,
        start: Vec2,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBNodeIndex,
    ) -> (BBEdgeIndex, BBEdge) {
        let start_index = self.add_node(start);
        self.cubic_from_to(start_index, ctrl1, ctrl2, end)
    }
    /// Adds a direct line from `start` to `end`, rebuilding shapes as needed.
    pub fn line_from_to(&mut self, start: BBNodeIndex, end: BBNodeIndex) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));
        debug_assert!(self.has_node(end));

        self.edge_line(start, end)
    }
    /// Adds a quadratic curve from `start` to `end`, rebuilding shapes as needed.
    pub fn quadratic_from_to(
        &mut self,
        start: BBNodeIndex,
        ctrl1: Vec2,
        end: BBNodeIndex,
    ) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));
        debug_assert!(self.has_node(end));

        self.edge_quadratic(start, ctrl1, end)
    }
    /// Adds a cubic curve from `start` to `end`, rebuilding shapes as needed.
    pub fn cubic_from_to(
        &mut self,
        start: BBNodeIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBNodeIndex,
    ) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));
        debug_assert!(self.has_node(end));

        self.edge_cubic(start, ctrl1, ctrl2, end)
    }

    pub fn translate(&mut self, translation: Vec2) {
        for v in self.nodes.iter_mut() {
            v.position = v.position + translation;
        }
        for l in self.edges.values_mut() {
            l.translate(translation);
        }
    }
}

/**
 * MCB related methods
 */
impl BBGraph {
    fn get_left_most_anchor_index(&self) -> Option<BBNodeIndex> {
        let Some(mut result_pos) = self.nodes.first().map(|a| a.position) else {
            return None;
        };
        let mut result_index = BBNodeIndex(0);
        for (i, anchor) in self.nodes.iter().enumerate() {
            if result_pos.x > anchor.position.x {
                result_pos = anchor.position;
                result_index = BBNodeIndex(i);
            }
        }

        Some(result_index)
    }
    /// Gets the next edges from a given point.
    /// Because the edges struct is directed, it reverses the edge if necessary.
    ///
    /// * `node_idx`:
    /// * `prev_edge_idx`:
    fn next_edges_of_node(
        &self,
        node_idx: BBNodeIndex,
        prev_edge_idx: Option<BBEdgeIndex>,
    ) -> BBResult<Vec<(BBEdgeIndex, BBEdge, Vec2)>> {
        let node = self.node(node_idx).unwrap();
        println!("Getting adjacents of {node_idx:?}. Filtering {prev_edge_idx:?} from {:?}", node.adjacents);
        // Get list of next edges, omitting the previous edge (if provided)
        node.adjacents()
            .iter()
            .filter(|edge_idx| {
                return prev_edge_idx.map_or(true, |prev_edge_idx: BBEdgeIndex| {
                    **edge_idx != prev_edge_idx
                });
            })
            .map(|edge_idx| {
                let edge = self.edge(*edge_idx)?.directed_from(node_idx);
                let tangent = edge.calc_start_tangent(self)?;
                Ok((*edge_idx, edge, tangent))
            })
            .collect()
    }

    pub fn get_cw_edge_of_node(
        &self,
        node_idx: BBNodeIndex,
        curr_dir: Vec2,
        prev_edge_idx: Option<BBEdgeIndex>,
    ) -> BBResult<BBEdgeIndex> {
        let mut next_edge_dirs = self.next_edges_of_node(node_idx, prev_edge_idx)?;

        let node = self.node(node_idx).unwrap();
        let curr_p = node.position();

        let Some((mut next_index, mut next_edge, mut next_dir)) = next_edge_dirs.pop() else {
            return Err(BBError::ClosedWalkDeadEnd);
        };

        for (el_index, el_edge, el_dir) in next_edge_dirs.into_iter() {
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
                if is_parrallel && t < 1. {
                    t = t + 1. / 32.;
                    temp_el_dir = el_edge.t_point(self, t) - curr_p;
                    temp_next_dir = next_edge.t_point(self, t) - curr_p;
                    continue;
                }

                let is_convex = temp_next_dir.determinate(curr_dir) < 0.;
                let ccw_of_curr = curr_dir.determinate(temp_el_dir) > 0.;
                let ccw_of_next = temp_next_dir.determinate(temp_el_dir) > 0.;

                if (!is_convex && ccw_of_curr && ccw_of_next)
                    || (is_convex && (ccw_of_curr || ccw_of_next))
                {
                    next_index = el_index;
                    next_edge = el_edge;
                    next_dir = temp_el_dir;
                }
                break;
            }
        }

        Ok(next_index)
    }

    pub fn get_ccw_edge_of_node(
        &self,
        node_idx: BBNodeIndex,
        curr_dir: Vec2,
        prev_edge_idx: Option<BBEdgeIndex>,
    ) -> BBResult<BBEdgeIndex> {
        let mut next_edge_dirs = self.next_edges_of_node(node_idx, prev_edge_idx)?;

        let node = self.node(node_idx).unwrap();
        let curr_p = node.position();

        let Some((mut next_index, mut next_edge, mut next_dir)) = next_edge_dirs.pop() else {
            return Err(BBError::ClosedWalkDeadEnd);
        };

        for (el_index, el_edge, el_dir) in next_edge_dirs.into_iter() {
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
                if is_parrallel && t < 1. {
                    t = t + 1. / 32.;
                    temp_el_dir = el_edge.t_point(self, t) - curr_p;
                    temp_next_dir = next_edge.t_point(self, t) - curr_p;
                    continue;
                }

                let is_convex = temp_next_dir.determinate(curr_dir) > 0.;
                let ccw_of_curr = curr_dir.determinate(temp_el_dir) >= 0.;
                let ccw_of_next = temp_next_dir.determinate(temp_el_dir) >= 0.;

                if (!is_convex && ccw_of_curr && ccw_of_next)
                    || (is_convex && (ccw_of_curr || ccw_of_next))
                {
                    next_index = el_index;
                    next_edge = el_edge;
                    next_dir = temp_el_dir;
                }
                break;
            }
        }

        Ok(next_index)
    }
}

/**
 * Debug drawing methods
 */
#[cfg(feature = "debug_draw")]
impl BBGraph {
    pub fn debug_draw(&self) -> BBResult<()> {
        for (index, edge) in self.edges.iter() {
            edge.debug_draw(self);
            comfy::draw_text(
                &format!("e{}", index.0),
                edge.t_point(self, 0.5),
                comfy::WHITE,
                comfy::TextAlign::Center,
            );
        }

        for ( i, node ) in self.nodes.iter().enumerate() {
            comfy::draw_circle(node.position(), 0.1, comfy::Color::rgb8(255, 0, 0), 1);
            comfy::draw_text(
                &format!("n{}", i),
                node.position(),
                comfy::WHITE,
                comfy::TextAlign::Center,
            );
        }

        let left_most = self.get_left_most_anchor_index();
        if let Some(left_most) = left_most {
            let node = self.node(left_most)?;
            let color = comfy::Color::rgb8(255, 100, 100);
            comfy::draw_circle(node.position(), 0.15, color, 1);
            comfy::draw_line(node.position(), node.position() + Vec2::new(0., -1.), 0.05, color, 1);

            let closed_walk_result = perform_closed_walk_from_node(self, left_most);
            match closed_walk_result {
                Ok((outer_edge, mut closed_walk)) => {
                    while let Some(idx) = closed_walk.pop() {
                        let link = self.edge(idx)?;
                        let thickness = if idx == outer_edge { 0.08 } else { 0.04 };
                        comfy::draw_arrow(link.start_pos(self), link.end_pos(self), thickness, color, 1);
                    }
                }
                Err(reason) => {
                    comfy::draw_text(&format!("Error: {reason:?}"), node.position, comfy::RED, comfy::TextAlign::Center);
                }
            }
        }

        Ok(())

        // for (region_index, region) in self.regions.values().enumerate() {
        //     for el in region.edge_indicies().iter() {
        //         println!("Loop {}: {:?}", region_index, el);
        //         for edge_index in el.iter() {
        //             let edge = self.edge(*edge_index).expect(
        //                 "BBVectorNetwork::debug_draw() -> No edge index for {edge_index:?}",
        //             );
        //             let pos = edge.t_point(self, 0.5);
        //             comfy::draw_text(
        //                 &format!("#{}:{}", region_index, edge_index.0),
        //                 pos + Vec2::new(0., 0.4 * (region_index + 1) as f32),
        //                 comfy::GRAY,
        //                 comfy::TextAlign::Center,
        //             );
        //         }
        //     }
        //     // region.debug_draw(self);
        // }
    }
}
