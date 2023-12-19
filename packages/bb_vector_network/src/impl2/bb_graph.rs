use std::{
    backtrace,
    fmt::Display,
    ops::{Mul, Sub},
};

use glam::Vec2;
use std::collections::{HashMap, HashSet, VecDeque};

#[cfg(feature = "debug_draw")]
use crate::debug_draw::draw_det_arc;
use crate::{prelude::mcb, Determinate};

use super::{
    bb_edge::{BBEdge, BBEdgeIndex},
    bb_node::{BBNode, BBNodeIndex},
    errors::{BBError, BBResult},
    mcb::{perform_closed_walk_from_node, ClosedWalk},
};

#[derive(Debug, Clone)]
pub struct BBGraph {
    next_idx: usize,
    pub nodes: HashMap<BBNodeIndex, BBNode>,
    pub edges: HashMap<BBEdgeIndex, BBEdge>,
    // pub regions: std::collections::HashMap<BBRegionIndex, BBVNRegion>,
}

impl Display for BBGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BBGraph {{")?;
        for (node_idx, node) in self.nodes.iter() {
            writeln!(f, "\t{node_idx} {node}")?;
        }
        for (edge_idx, edge) in self.edges.iter() {
            writeln!(f, "\t{edge_idx} {edge}")?;
        }
        writeln!(f, "}}")
    }
}

impl BBGraph {
    pub fn new() -> Self {
        Self {
            next_idx: 0,
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Generates a new BBGraph from the edges of another BBGraph
    ///
    /// * `other`:
    /// * `edges`:
    pub fn try_new_from_other_edges(
        other: &BBGraph,
        edge_indices: &[BBEdgeIndex],
    ) -> BBResult<Self> {
        let mut next_idx = 0;
        let mut nodes = HashMap::new();
        let mut edges = HashMap::new();

        for edge_idx in edge_indices {
            let edge = other.edge(*edge_idx)?;
            edges.insert(*edge_idx, *edge);

            let start_idx = edge.start_idx();
            if !nodes.contains_key(&start_idx) {
                let node_pos = other.node(start_idx)?.position();
                let node = BBNode::new(node_pos);
                nodes.insert(start_idx, node);
            }
            nodes.get_mut(&start_idx).unwrap().adjacents.push(*edge_idx);

            let end_idx = edge.end_idx();
            if !nodes.contains_key(&end_idx) {
                let node_pos = other.node(end_idx)?.position();
                let node = BBNode::new(node_pos);
                nodes.insert(end_idx, node);
            }
            nodes.get_mut(&end_idx).unwrap().adjacents.push(*edge_idx);
        }

        Ok(Self {
            next_idx,
            nodes,
            edges,
            // regions: HashMap::new(),
        })
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
        match self.edges.get(&index) {
            Some(edge) => Ok(edge),
            None => Err(BBError::MissingEdge(index)),
        }
    }
    /// Gets a reference to a Vector Network edge between two nodes
    pub fn edge_mut(&mut self, index: BBEdgeIndex) -> BBResult<&mut BBEdge> {
        self.edges
            .get_mut(&index)
            .ok_or(BBError::MissingEdge(index))
    }
    /// Given a list of edge idxs (closed walk), returns them all directed
    /// in the same direction.
    pub fn edges_from_closed_walk(
        &self,
        closed_walk: &ClosedWalk,
    ) -> BBResult<Vec<(BBEdgeIndex, BBEdge)>> {
        if closed_walk.len() == 0 {
            return Err(BBError::ClosedWalkTooSmall(closed_walk.len()));
        }
        let first_edge_idx = closed_walk.first().unwrap();
        let mut prev_edge = *self.edge(*first_edge_idx)?;

        let mut directed_closed_walk = vec![(*first_edge_idx, prev_edge)];

        for edge_idx in &closed_walk[1..] {
            let edge = self.edge(*edge_idx)?.directed_from(prev_edge.end_idx());
            directed_closed_walk.push((*edge_idx, edge));
            prev_edge = edge;
        }

        Ok(directed_closed_walk)
    }
    /// Gets a reference to an node
    pub fn node(&self, index: BBNodeIndex) -> BBResult<&BBNode> {
        match self.nodes.get(&index) {
            Some(node) => Ok(node),
            None => Err(BBError::MissingNode(index)),
        }
    }
    /// Gets a mutable reference to an node
    pub fn node_mut(&mut self, index: BBNodeIndex) -> BBResult<&mut BBNode> {
        self.nodes
            .get_mut(&index)
            .ok_or(BBError::MissingNode(index))
    }
    /// Gets the number of nodes stored.
    pub fn node_len(&self) -> usize {
        self.nodes.len()
    }

    pub fn has_node(&self, index: BBNodeIndex) -> bool {
        self.nodes.get(&index).is_some()
    }
}

/**
 * Graph Building API
 */
impl BBGraph {
    /// Pushes a new node node to the BBVectorNetwork
    fn add_node(&mut self, value: Vec2) -> BBNodeIndex {
        let node_idx = BBNodeIndex(self.get_next_idx());
        self.nodes.insert(node_idx, BBNode::new(value));
        node_idx
    }
    /// Deletes an node, deletes associated edges and breaks regions containing these edges.
    pub fn delete_node(&mut self, index: BBNodeIndex) -> BBResult<()> {
        debug_assert!(self.has_node(index));

        let adjacents = self.node(index)?.adjacents.clone();
        self.nodes.remove(&index);

        for adj in adjacents {
            self.delete_edge(adj)?;
        }

        // TODO delete regions.
        Ok(())
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

    pub fn delete_edge(&mut self, edge_idx: BBEdgeIndex) -> BBResult<()> {
        let edge = *self.edge(edge_idx).unwrap();
        self.edges.remove(&edge_idx);

        if let Ok(start) = self.node_mut(edge.start_idx()) {
            start.adjacents.retain(|e_idx| *e_idx != edge_idx);
            if start.adjacents().len() == 0 {
                self.delete_node(edge.start_idx())?;
            }
        }

        if let Ok(end) = self.node_mut(edge.end_idx()) {
            end.adjacents.retain(|e_idx| *e_idx != edge_idx);
            if end.adjacents().len() == 0 {
                self.delete_node(edge.end_idx())?;
            }
        }

        Ok(())
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
        for v in self.nodes.values_mut() {
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
    pub fn get_left_most_anchor_index(&self) -> Option<BBNodeIndex> {
        let mut nodes_iter = self.nodes.iter().map(|(idx, node)| (idx, node.position()));

        let Some((mut result_idx, mut result_pos)) = nodes_iter.next() else {
            return None;
        };

        for (idx, pos) in nodes_iter {
            if pos.x < result_pos.x || (pos.x == result_pos.x && result_pos.y < pos.y) {
                result_pos = pos;
                result_idx = idx;
            }
        }

        Some(*result_idx)
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

        let node = self.node(node_idx)?;
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
                let is_parrallel = temp_el_dir.dot(temp_next_dir).abs() < 0.01;
                if is_parrallel && t < 1. {
                    t = t + 1. / 32.;
                    temp_el_dir = el_edge.t_point(self, t) - curr_p;
                    temp_next_dir = next_edge.t_point(self, t) - curr_p;
                    continue;
                }

                let is_convex = curr_dir.determinate(temp_next_dir) < 0.;
                let ccw_of_curr = curr_dir.determinate(temp_el_dir) > 0.;
                let ccw_of_next = temp_el_dir.determinate(temp_next_dir) > 0.;

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

        let node = self.node(node_idx)?;
        let curr_p = node.position();

        let Some((mut next_index, mut next_edge, mut next_dir)) = next_edge_dirs.pop() else {
            return Err(BBError::ClosedWalkDeadEnd);
        };

        for (i, (el_index, el_edge, el_dir)) in next_edge_dirs.into_iter().enumerate() {
            let mut temp_el_dir = el_dir;
            let mut temp_next_dir = next_dir;

            // #[cfg(feature = "debug_draw")]
            // draw_det_arc(curr_p, 0.5 + (i as f32) * 0.5, curr_dir, el_dir, next_dir);

            // When lines a parallel we need to move our test points across the lines until we find
            // one that isn't parallel.  This loop starts at 0 but will iterate forward if there's
            // no good option.
            let mut t = 0.;
            loop {
                let is_parrallel = temp_el_dir.dot(temp_next_dir).abs() < 0.01;
                if is_parrallel && t < 1. {
                    t = t + 1. / 32.;
                    temp_el_dir = el_edge.t_point(self, t) - curr_p;
                    temp_next_dir = next_edge.t_point(self, t) - curr_p;
                    continue;
                }

                let is_convex = curr_dir.determinate(temp_next_dir) > 0.;
                let ccw_of_curr = curr_dir.determinate(temp_el_dir) < 0.;
                let ccw_of_next = temp_el_dir.determinate(temp_next_dir) < 0.;

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

    /// Performs a breadth first search over the graph to return a Vec of each detached graph
    /// within it.
    pub fn get_detached_graphs(&self) -> BBResult<Vec<BBGraph>> {
        let mut result = vec![];

        let mut edges_to_visit: HashSet<BBEdgeIndex> = self.edges.keys().cloned().collect();

        while edges_to_visit.len() != 0 {
            let first = {
                let mut edges_to_visit_queue: Vec<_> = edges_to_visit.iter().collect();
                let Some(first) = edges_to_visit_queue.pop() else {
                    break;
                };
                *first
            };

            edges_to_visit.remove(&first);
            let mut detached_edges = vec![first];
            let mut queue = VecDeque::from(vec![first]);

            while let Some(edge_idx) = queue.pop_back() {
                let edge = self.edge(edge_idx)?;
                detached_edges.push(edge_idx);

                for adj in edge.start(self).adjacents() {
                    if edges_to_visit.contains(adj) {
                        queue.push_back(*adj);
                        edges_to_visit.remove(adj);
                    }
                }

                for adj in edge.end(self).adjacents() {
                    if edges_to_visit.contains(adj) {
                        queue.push_back(*adj);
                        edges_to_visit.remove(adj);
                    }
                }
            }

            let graph = BBGraph::try_new_from_other_edges(self, &detached_edges)?;
            result.push(graph);
        }

        Ok(result)
    }

    pub fn remove_filaments(&mut self) -> BBResult<()> {
        while let Some((node_idx, node)) = self
            .nodes
            .iter()
            .find(|(_, node)| node.adjacents().len() == 1)
        {
            println!("Deleting node {node_idx}.");
            self.delete_node(*node_idx);
        }

        Ok(())
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
                &format!("{}:", index),
                edge.t_point(self, 0.5),
                comfy::WHITE,
                comfy::TextAlign::Center,
            );
        }

        for (i, node) in self.nodes.iter() {
            comfy::draw_circle(node.position(), 0.1, comfy::Color::rgb8(255, 0, 0), 1);
            comfy::draw_text(
                &format!("n{}\np{}", i, node.position()),
                node.position(),
                comfy::WHITE,
                comfy::TextAlign::Center,
            );
        }

        let colors = vec![comfy::SEA_GREEN, comfy::LIME_GREEN, comfy::YELLOW_GREEN];

        for (i, mut graph) in self.get_detached_graphs()?.into_iter().enumerate() {
            let color = colors[i % colors.len()];

            while graph.node_len() > 0 {
                println!("Handling closed walk");
                graph.remove_filaments();

                let Some(left_most) = graph.get_left_most_anchor_index() else {
                    break;
                };
                let node = self.node(left_most)?;
                comfy::draw_circle(node.position(), 0.15, color, 1);

                let (outer_edge_idx, closed_walk) =
                    mcb::perform_closed_walk_from_node(&graph, left_most)?;
                let outer_edge = self.edge(outer_edge_idx)?;
                comfy::draw_line(
                    outer_edge.start_pos(self),
                    outer_edge.end_pos(self),
                    0.08,
                    color,
                    1,
                );

                let (parent_cycle, nested_walks) =
                    mcb::extract_nested_from_closed_walk(&graph, &closed_walk)?;
                for edge_idx in parent_cycle {
                    let edge = self.edge(edge_idx)?;
                    comfy::draw_arrow(
                        edge.t_point(self, 0.25),
                        edge.t_point(self, 0.75),
                        0.05,
                        color,
                        50,
                    );
                }
                let nested_color = color * 0.75;
                for closed_walk in nested_walks {
                    for edge_idx in closed_walk {
                        let edge = self.edge(edge_idx)?;
                        comfy::draw_arrow(
                            edge.t_point(self, 0.25),
                            edge.t_point(self, 0.75),
                            0.05,
                            nested_color,
                            50,
                        );
                    }
                }
            }
        }

        let left_most = self.get_left_most_anchor_index();
        if let Some(left_most) = left_most {
            let node = self.node(left_most)?;
            let color = comfy::Color::rgb8(255, 100, 100);
            comfy::draw_circle(node.position(), 0.15, color, 1);
            comfy::draw_line(
                node.position(),
                node.position() + Vec2::new(0., -1.),
                0.05,
                color,
                1,
            );

            let closed_walk_result = perform_closed_walk_from_node(self, left_most);
            // match closed_walk_result {
            //     Ok((outer_edge, mut closed_walk)) => {
            //         while let Some(idx) = closed_walk.pop() {
            //             let link = self.edge(idx)?;
            //             let thickness = if idx == outer_edge { 0.08 } else { 0.04 };
            //             comfy::draw_arrow(link.start_pos(self), link.end_pos(self), thickness, color, 1);
            //         }
            //     }
            //     Err(reason) => {
            //         comfy::draw_text(&format!("Error: {reason:?}"), node.position, comfy::RED, comfy::TextAlign::Center);
            //     }
            // }
        }

        Ok(())

        // for (region_index, region) in self.regions.values().enumerate() {
        //     for el in region.edge_indicies().iter() {
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
