pub mod clipping;
#[cfg(feature = "lyon_path")]
pub mod lyon;

use std::collections::hash_map::{self};

#[allow(unused_imports)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    ops::{Mul, Sub},
};

use crate::prelude::Determinate;
use glam::{vec2, Vec2};

use super::{
    bb_edge::{BBEdge, BBEdgeIndex},
    bb_node::{BBNode, BBNodeIndex},
    bb_region::{BBCycle, BBRegion, BBRegionIndex},
    errors::{BBError, BBResult},
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BBGraph {
    next_idx: usize,
    #[cfg_attr(feature = "serde", serde_as(as = "Vec<(_, _)>"))]
    pub nodes: HashMap<BBNodeIndex, BBNode>,
    #[cfg_attr(feature = "serde", serde_as(as = "Vec<(_, _)>"))]
    pub edges: HashMap<BBEdgeIndex, BBEdge>,
    #[cfg_attr(feature = "serde", serde_as(as = "Vec<(_, _)>"))]
    pub regions: HashMap<BBRegionIndex, BBRegion>,
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

impl Default for BBGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl BBGraph {
    pub fn new() -> Self {
        Self {
            next_idx: 0,
            nodes: HashMap::new(),
            edges: HashMap::new(),
            regions: HashMap::new(),
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
        let next_idx = 0;
        let mut nodes = HashMap::new();
        let mut edges = HashMap::new();

        for edge_idx in edge_indices {
            let edge = other.edge(*edge_idx)?;
            edges.insert(*edge_idx, *edge);

            let start_idx = edge.start_idx();
            if let hash_map::Entry::Vacant(e) = nodes.entry(start_idx) {
                let node_pos = other.node(start_idx)?.position();
                let node = BBNode::new(node_pos);
                e.insert(node);
            }
            nodes.get_mut(&start_idx).unwrap().adjacents.push(*edge_idx);

            let end_idx = edge.end_idx();
            if let hash_map::Entry::Vacant(e) = nodes.entry(end_idx) {
                let node_pos = other.node(end_idx)?.position();
                let node = BBNode::new(node_pos);
                e.insert(node);
            }
            nodes.get_mut(&end_idx).unwrap().adjacents.push(*edge_idx);
        }

        Ok(Self {
            next_idx,
            nodes,
            edges,
            regions: HashMap::new(),
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
    /// Gets a reference to a Vector Network edge by ID
    ///
    /// * `index`: ID of edge to get
    pub fn edge(&self, index: BBEdgeIndex) -> BBResult<&BBEdge> {
        match self.edges.get(&index) {
            Some(edge) => Ok(edge),
            None => Err(BBError::MissingEdge(index)),
        }
    }
    /// Gets a mutable reference to a Vector Network edge by ID
    ///
    /// * `index`: ID of edge to get
    pub fn edge_mut(&mut self, index: BBEdgeIndex) -> BBResult<&mut BBEdge> {
        self.edges
            .get_mut(&index)
            .ok_or(BBError::MissingEdge(index))
    }

    /// Given a list of edge indices, returns the edges with all directions oriented in a
    /// continuous direction.
    ///
    /// * `edges`: A slice of edges that you want to get the edge data of.
    pub fn edges_directed(&self, edges: &[BBEdgeIndex]) -> BBResult<Vec<(BBEdgeIndex, BBEdge)>> {
        if edges.is_empty() {
            return Err(BBError::ClosedWalkTooSmall(edges.len()));
        }
        let first_edge_idx = edges.first().unwrap();
        let mut first_edge = *self.edge(*first_edge_idx)?;

        if edges.len() == 1 {
            return Ok(vec![(*first_edge_idx, first_edge)]);
        }

        // Reverse first edge if it isn't continuous w second
        let second_edge = self.edge(edges[1])?;
        let first_second_shared_node = first_edge.shared_node(second_edge).unwrap();
        if first_edge.start_idx() == first_second_shared_node {
            first_edge = first_edge.reversed();
        }

        // Iterate over vec
        let mut prev_edge = first_edge;
        let mut directed = vec![(*first_edge_idx, first_edge)];
        for edge_idx in &edges[1..] {
            let edge = self.edge(*edge_idx)?.directed_from(prev_edge.end_idx());
            directed.push((*edge_idx, edge));
            prev_edge = edge;
        }

        Ok(directed)
    }

    /// Returns the count of edges in the graph
    pub fn edges_count(&self) -> usize {
        self.edges.len()
    }

    /// Gets a reference to a node by ID
    ///
    /// * `index`: ID of node to get
    pub fn node(&self, index: BBNodeIndex) -> BBResult<&BBNode> {
        match self.nodes.get(&index) {
            Some(node) => Ok(node),
            None => Err(BBError::MissingNode(index)),
        }
    }
    /// Gets a mutable reference to a node by ID
    ///
    /// * `index`: ID of node to get
    pub fn node_mut(&mut self, index: BBNodeIndex) -> BBResult<&mut BBNode> {
        self.nodes
            .get_mut(&index)
            .ok_or(BBError::MissingNode(index))
    }

    /// Returns the count of nodes in the graph
    pub fn nodes_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true if the BBGraph contain a node at the given node ID
    ///
    /// * `index`: ID of node to check
    pub fn has_node(&self, index: BBNodeIndex) -> bool {
        self.nodes.contains_key(&index)
    }
}

/**
 * Graph Building API
 */
impl BBGraph {
    /// Pushes a new node node to the BBVectorNetwork
    ///
    /// * `value`: Position of the node
    pub(crate) fn add_node(&mut self, position: Vec2) -> BBNodeIndex {
        let node_idx = BBNodeIndex(self.get_next_idx());
        self.nodes.insert(node_idx, BBNode::new(position));
        node_idx
    }
    /// Removes a node from the graph by ID.  Will delete connected edges and regions.
    ///
    /// * `index`: Index of the node to delete
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

    /// Adds an edge to the graph.  This is used internally by the line/quadratic/cubic to/from API
    /// to push the edges to the graph.  Returns the edge ID + edge struct itself.
    ///
    /// * `edge`: The edge data to add to the BBGraph
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

    /// Removes an edge from the graph.  If necessary, will also delete the nodes this edge
    /// connects.
    ///
    /// * `edge_idx`: ID of the edge to delete.
    pub fn delete_edge(&mut self, edge_idx: BBEdgeIndex) -> BBResult<BBEdge> {
        let edge = *self.edge(edge_idx)?;
        self.edges.remove(&edge_idx);

        if let Ok(start) = self.node_mut(edge.start_idx()) {
            start.adjacents.retain(|e_idx| *e_idx != edge_idx);
            if start.adjacents().is_empty() {
                self.delete_node(edge.start_idx())?;
            }
        }

        if let Ok(end) = self.node_mut(edge.end_idx()) {
            end.adjacents.retain(|e_idx| *e_idx != edge_idx);
            if end.adjacents().is_empty() {
                self.delete_node(edge.end_idx())?;
            }
        }

        Ok(edge)
    }

    /*
     * GRAPH BUILDING API - edge functions
     */

    /// Adds a line between two nodes.
    ///
    /// Used internallyedge by [line()], [line_from()],
    /// [line_to()], and [line_from_to()]
    ///
    /// * `start`: ID of node to start at
    /// * `end`: ID of node to end at
    fn edge_line(&mut self, start: BBNodeIndex, end: BBNodeIndex) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));
        debug_assert!(self.has_node(end));
        let edge = BBEdge::Line { start, end };
        self.add_edge(edge)
    }

    /// Creates a line from the start position to the end position, creating new nodes for both.
    ///
    /// * `start`: Position of the new start node
    /// * `end`: Position of the new end node
    pub fn line(&mut self, start: Vec2, end: Vec2) -> (BBEdgeIndex, BBEdge) {
        let start_index = self.add_node(start);
        let end_index = self.add_node(end);
        self.edge_line(start_index, end_index)
    }

    /// Creates a new node at `end` and adds a straight line edge between it and the start node.
    ///
    /// * `start`: ID of the start node
    /// * `ctrl1` : Position of the control point
    /// * `end`: Position of the new end node
    pub fn line_from(&mut self, start: BBNodeIndex, to: Vec2) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));

        let end_index = self.add_node(to);
        self.edge_line(start, end_index)
    }

    /// Creates a new node at `start` and adds a straight line edge between it and the end node.
    ///
    /// * `start`: Position of the start node
    /// * `end`: ID of the new end node
    pub fn line_to(&mut self, start: Vec2, end: BBNodeIndex) -> (BBEdgeIndex, BBEdge) {
        let start_index = self.add_node(start);
        self.edge_line(start_index, end)
    }

    /// Adds a straight line edge between two nodes.
    ///
    /// * `start`: ID of the start node
    /// * `ctrl1` : Position of the control node
    /// * `end`: ID of the end node
    pub fn line_from_to(&mut self, start: BBNodeIndex, end: BBNodeIndex) -> (BBEdgeIndex, BBEdge) {
        debug_assert!(self.has_node(start));
        debug_assert!(self.has_node(end));
        self.edge_line(start, end)
    }

    /// Adds a quadratic curve between two nodes.
    ///
    /// Used internally by [quadratic()], [quadratic_from()],
    /// [quadratic_to()], and [quadratic_from_to()]
    ///
    /// * `start`: ID of the node to start at
    /// * `ctrl1`: Position of the control point
    /// * `end`: ID of the node to end at
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

    /// Creates a quadratic curve from the start position to the end position, creating new nodes for both.
    ///
    /// * `start`: Position of the new start node
    /// * `ctrl1` : Position of the control point
    /// * `end`: Position of the new end node
    pub fn quadratic(&mut self, start: Vec2, ctrl1: Vec2, end: Vec2) -> (BBEdgeIndex, BBEdge) {
        // println!("quadratic {start} {end}");
        let start_index = self.add_node(start);
        let end_index = self.add_node(end);
        self.edge_quadratic(start_index, ctrl1, end_index)
    }

    /// Creates a new node at `end` and adds a quadratic curve edge between it and the start node.
    ///
    /// * `start`: ID of the start node
    /// * `ctrl1` : Position of the control point
    /// * `end`: Position of the new end node
    pub fn quadratic_from(
        &mut self,
        start: BBNodeIndex,
        ctrl1: Vec2,
        to: Vec2,
    ) -> (BBEdgeIndex, BBEdge) {
        // println!("quadratic_from {start} {to}");
        debug_assert!(self.has_node(start));

        let end = self.add_node(to);
        self.edge_quadratic(start, ctrl1, end)
    }

    /// Creates a new node at `start` and adds a quadratic curve edge between it and the end node.
    ///
    /// * `start`: Position of the start node
    /// * `ctrl1` : Position of the control node
    /// * `end`: ID of the new end node
    pub fn quadratic_to(
        &mut self,
        start: Vec2,
        ctrl1: Vec2,
        end: BBNodeIndex,
    ) -> (BBEdgeIndex, BBEdge) {
        // println!("quadratic_to {start} {end}");
        let start_index = self.add_node(start);
        self.edge_quadratic(start_index, ctrl1, end)
    }

    /// Adds a quadratic curve edge between two nodes.
    ///
    /// * `start`: ID of the start node
    /// * `ctrl1` : Position of the control node
    /// * `end`: ID of the end node
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

    /// Adds a cubic curve between two nodes.
    ///
    /// Used internally by [cubic()], [cubic_from()],
    /// [cubic_to()], and [cubic_from_to()]
    ///
    /// * `start`: ID of the node to start at
    /// * `ctrl1`: Position of the first control point
    /// * `ctrl2`: Position of the second control point
    /// * `end`: ID of the node to end at
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

    /// Creates a cubic curve from the start position to the end position, creating new nodes for both.
    ///
    /// * `start`: Position of the new start node
    /// * `ctrl1` : Position of the first control point
    /// * `ctrl2` : Position of the second control point
    /// * `end`: Position of the new end node
    pub fn cubic(
        &mut self,
        start: Vec2,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: Vec2,
    ) -> (BBEdgeIndex, BBEdge) {
        // println!("cubic {start} {end}");
        let start_index = self.add_node(start);
        let end_index = self.add_node(end);
        self.edge_cubic(start_index, ctrl1, ctrl2, end_index)
    }

    /// Creates a new node at `end` and adds a cubic curve edge between it and the start node.
    ///
    /// * `start`: ID of the start node
    /// * `ctrl1` : Position of the first control point
    /// * `ctrl2` : Position of the second control point
    /// * `end`: Position of the new end node
    pub fn cubic_from(
        &mut self,
        start: BBNodeIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        to: Vec2,
    ) -> (BBEdgeIndex, BBEdge) {
        // println!("cubic_from {start} {to}");
        debug_assert!(self.has_node(start));
        let end = self.add_node(to);
        self.edge_cubic(start, ctrl1, ctrl2, end)
    }

    /// Creates a new node at `start` and adds a cubic curve edge between it and the end node.
    ///
    /// * `start`: Position of the start node
    /// * `ctrl1` : Position of the first control node
    /// * `ctrl2` : Position of the second control node
    /// * `end`: ID of the new end node
    pub fn cubic_to(
        &mut self,
        start: Vec2,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBNodeIndex,
    ) -> (BBEdgeIndex, BBEdge) {
        // println!("cubic_to {start} {end}");
        let start_index = self.add_node(start);
        self.edge_cubic(start_index, ctrl1, ctrl2, end)
    }

    /// Adds a cubic curve edge between two nodes.
    ///
    /// * `start`: ID of the start node
    /// * `ctrl1` : Position of the first control node
    /// * `ctrl2` : Position of the second control node
    /// * `end`: ID of the end node
    pub fn cubic_from_to(
        &mut self,
        start: BBNodeIndex,
        ctrl1: Vec2,
        ctrl2: Vec2,
        end: BBNodeIndex,
    ) -> (BBEdgeIndex, BBEdge) {
        // println!("cubic_from_to {start} {end}");
        debug_assert!(self.has_node(start));
        debug_assert!(self.has_node(end));

        self.edge_cubic(start, ctrl1, ctrl2, end)
    }

    pub fn translate(&mut self, translation: Vec2) {
        for v in self.nodes.values_mut() {
            v.position += translation;
        }
        for l in self.edges.values_mut() {
            l.translate(translation);
        }
    }
}

/**
 * MCB helper methods
 */
impl BBGraph {
    /// Tries to return the index left most node.  
    ///
    /// If two nodes have the same `x` value, it will pick the one with the lower `y` value.
    pub fn get_left_most_node_index(&self) -> Option<BBNodeIndex> {
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
    ) -> BBResult<Vec<(BBEdgeIndex, BBEdge)>> {
        let node = self.node(node_idx)?;
        // Get list of next edges, omitting the previous edge (if provided)
        node.adjacents()
            .iter()
            .filter(|edge_idx| {
                prev_edge_idx.map_or(true, |prev_edge_idx: BBEdgeIndex| {
                    **edge_idx != prev_edge_idx
                })
            })
            .map(|edge_idx| {
                let edge = self.edge(*edge_idx)?.directed_from(node_idx);
                Ok((*edge_idx, edge))
            })
            .collect()
    }

    /// Given a node and a current direction, calculates which edge is the most clockwise.
    /// It uses matrix2 determinates to calculate if it's clockwise.
    /// https://alexharri.medium.com/the-engineering-behind-figmas-vector-networks-688568e37110
    ///
    /// * `node_idx`: Current Node idx
    /// * `curr_dir`: Current direction
    /// * `prev_edge_idx`: Previous edge to exclude from results
    pub fn get_cw_edge_of_node(
        &self,
        node_idx: BBNodeIndex,
        curr_dir: Vec2,
        prev_edge_idx: Option<BBEdgeIndex>,
    ) -> BBResult<BBEdgeIndex> {
        let mut next_edge_dirs = self.next_edges_of_node(node_idx, prev_edge_idx)?;

        let Some((first_idx, first_edge)) = next_edge_dirs.pop() else {
            return Err(BBError::ClosedWalkDeadEnd);
        };
        if next_edge_dirs.is_empty() {
            return Ok(first_idx);
        }

        let mut best_idx = first_idx;
        let mut best_dir = first_edge.calc_start_tangent(self)?;

        for (idx, edge) in next_edge_dirs.iter() {
            let dir = edge.calc_start_tangent(self)?;
            let needs_convex_check = curr_dir.determinate(best_dir) > 0.;

            let ccw_of_curr = curr_dir.determinate(dir) <= 0.;
            let ccw_of_best = best_dir.determinate(dir) <= 0.;

            if needs_convex_check {
                if ccw_of_curr || ccw_of_best {
                    best_idx = *idx;
                    best_dir = dir;
                }
            } else {
                if ccw_of_curr && ccw_of_best {
                    best_idx = *idx;
                    best_dir = dir;
                }
            }
        }

        Ok(best_idx)
    }

    /// Given a node and a current direction, calculates which edge is the most counterclockwise.
    /// It uses matrix2 determinates to calculate if it's counter clockwise.
    /// https://alexharri.medium.com/the-engineering-behind-figmas-vector-networks-688568e37110
    ///
    /// * `node_idx`: Current Node idx
    /// * `curr_dir`: Current direction
    /// * `prev_edge_idx`: Previous edge to exclude from results
    pub fn get_ccw_edge_of_node(
        &self,
        node_idx: BBNodeIndex,
        curr_dir: Vec2,
        prev_edge_idx: Option<BBEdgeIndex>,
    ) -> BBResult<BBEdgeIndex> {
        let mut next_edge_dirs = self.next_edges_of_node(node_idx, prev_edge_idx)?;

        let Some((first_idx, first_edge)) = next_edge_dirs.pop() else {
            return Err(BBError::ClosedWalkDeadEnd);
        };
        if next_edge_dirs.is_empty() {
            return Ok(first_idx);
        }

        let mut best_idx = first_idx;
        let mut best_dir = first_edge.calc_start_tangent(self)?;

        for (idx, edge) in next_edge_dirs.iter() {
            let dir = edge.calc_start_tangent(self)?;
            let needs_convex_check = curr_dir.determinate(best_dir) < 0.;

            let ccw_of_curr = curr_dir.determinate(dir) > 0.;
            let ccw_of_best = best_dir.determinate(dir) > 0.;

            if needs_convex_check {
                if ccw_of_curr || ccw_of_best {
                    best_idx = *idx;
                    best_dir = dir;
                }
            } else {
                if ccw_of_curr && ccw_of_best {
                    best_idx = *idx;
                    best_dir = dir;
                }
            }
        }

        Ok(best_idx)
    }

    /// Performs a breadth first search over the graph to return a Vec of each detached graph
    /// within it.
    pub fn get_detached_graphs(&self) -> BBResult<Vec<BBGraph>> {
        let mut result = vec![];

        let mut edges_to_visit: HashSet<BBEdgeIndex> = self.edges.keys().cloned().collect();

        while !edges_to_visit.is_empty() {
            let first = {
                let mut edges_to_visit_queue: Vec<_> = edges_to_visit.iter().collect();
                let Some(first) = edges_to_visit_queue.pop() else {
                    break;
                };
                *first
            };

            edges_to_visit.remove(&first);
            let mut queue = VecDeque::from(vec![first]);
            let mut detached_edges = vec![];

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

    /// Removes all dead-end paths from self.
    pub fn remove_filaments(&mut self) -> BBResult<()> {
        while let Some((node_idx, _)) = self
            .nodes
            .iter()
            .find(|(_, node)| node.adjacents().len() == 1)
        {
            self.delete_node(*node_idx)?;
        }

        Ok(())
    }
}

/**
 * Graph traversal helper methods.
 */
struct ClosedWalkModel {
    traversals: usize,
    curr_node_idx: BBNodeIndex,
    curr_edge_idx: BBEdgeIndex,
    curr_dir: Vec2,
    outer_edge: BBEdgeIndex,
    edges: Vec<BBEdgeIndex>,
}

#[derive(Debug)]
pub struct ClosedWalkResult {
    pub outer_edge: BBEdgeIndex,
    pub edges: Vec<BBEdgeIndex>,
}

impl From<ClosedWalkModel> for ClosedWalkResult {
    fn from(value: ClosedWalkModel) -> Self {
        Self {
            outer_edge: value.outer_edge,
            edges: value.edges,
        }
    }
}

pub enum TraverseAction {
    Continue,
    Stop,
}

impl BBGraph {
    /// Helper method to make custom iterations/traversals on the graph easier.
    /// Will call the traversal `strategy()` closure over and over until `TraverseAction::Stop`
    /// is returned.  It will then return the model used during the traversal.
    ///
    /// * `initial_model`:
    /// * `strategy`:
    pub fn traverse_with_model<TModel: Sized, TResult: From<TModel>>(
        &self,
        initial_model: TModel,
        strategy: impl Fn(&mut TModel) -> BBResult<TraverseAction>,
    ) -> BBResult<TResult> {
        let mut model = initial_model;

        loop {
            let action = strategy(&mut model)?;

            if let TraverseAction::Stop = action {
                break;
            }
        }

        Ok(model.into())
    }

    /// Peforms a closed walk from a given node.  Firstly finds the counterclockwise most edge and
    /// then traverses around the next counterclockwise most edges until returning to the starting
    /// node and returning.
    ///
    /// This walk is useful for finding the outer perimiter of a shape, if you start from the left
    /// most edge and perform the walk, it will traverse the perimiter of the graph in a clockwise
    /// direction.
    ///
    /// * `node_idx`: Node ID that you want to start this walk from.
    pub fn closed_walk_with_ccw_start_and_ccw_traverse(
        &self,
        node_idx: BBNodeIndex,
    ) -> BBResult<ClosedWalkResult> {
        // Get the clockwise most edge of the start node
        let first_edge_idx = self.get_ccw_edge_of_node(node_idx, vec2(0., 1.), None)?;
        let first_edge = self.edge(first_edge_idx)?.directed_from(node_idx);

        let model = ClosedWalkModel {
            traversals: 0,
            curr_node_idx: first_edge.end_idx(),
            curr_dir: first_edge.calc_end_tangent(self)?,
            curr_edge_idx: first_edge_idx,
            edges: vec![first_edge_idx],
            outer_edge: first_edge_idx,
        };
        // Walk around the graph finding the counterclockwise most edges until returning to start.
        let result = self.traverse_with_model(model, |model| {
            if model.traversals > 1000 {
                return Err(BBError::TraversalLimit(model.edges.clone()));
            }

            let next_edge_idx = self.get_ccw_edge_of_node(
                model.curr_node_idx,
                model.curr_dir,
                Some(model.curr_edge_idx),
            )?;
            if first_edge_idx == next_edge_idx {
                return Ok(TraverseAction::Stop);
            }
            let next_edge = self.edge(next_edge_idx)?.directed_from(model.curr_node_idx);

            model.traversals += 1;
            model.curr_edge_idx = next_edge_idx;
            model.curr_node_idx = next_edge.other_node_idx(model.curr_node_idx);
            model.curr_dir = next_edge.calc_end_tangent(self)?;
            model.edges.push(next_edge_idx);

            Ok(TraverseAction::Continue)
        })?;

        Ok(result)
    }

    /// Peforms a closed walk from a given node.  Firstly finds the clockwise most edge and then
    /// traverses around the next counterclockwise most edges until returning to the starting
    /// node and returning.
    ///
    /// This walk is useful for finding minimal cycle bases within the graph (to do so, start from
    /// the left most node and perfomr the traverse, then process nested graphs, if any).
    ///
    /// * `node_idx`: Node ID that you want to start this walk from.
    pub fn closed_walk_with_cw_start_and_ccw_traverse(
        &self,
        node_idx: BBNodeIndex,
    ) -> BBResult<ClosedWalkResult> {
        // Get the clockwise most edge of the start node
        let first_edge_idx = self.get_cw_edge_of_node(node_idx, vec2(0., 1.), None)?;
        let first_edge = self.edge(first_edge_idx)?.directed_from(node_idx);

        let model = ClosedWalkModel {
            traversals: 0,
            curr_node_idx: first_edge.end_idx(),
            curr_dir: first_edge.calc_end_tangent(self)?,
            curr_edge_idx: first_edge_idx,
            edges: vec![first_edge_idx],
            outer_edge: first_edge_idx,
        };
        // Walk around the graph finding the counterclockwise most edges until returning to start.
        let result: ClosedWalkResult = self.traverse_with_model(model, |model| {
            if model.traversals > 1000 {
                return Err(BBError::TraversalLimit(model.edges.clone()));
            }

            let next_edge_idx = self.get_ccw_edge_of_node(
                model.curr_node_idx,
                model.curr_dir,
                Some(model.curr_edge_idx),
            )?;
            if first_edge_idx == next_edge_idx {
                return Ok(TraverseAction::Stop);
            }
            let next_edge = self.edge(next_edge_idx)?.directed_from(model.curr_node_idx);

            model.traversals += 1;
            model.curr_edge_idx = next_edge_idx;
            model.curr_node_idx = next_edge.end_idx();
            model.curr_dir = next_edge.calc_end_tangent(self)?;
            model.edges.push(next_edge_idx);

            Ok(TraverseAction::Continue)
        })?;

        Ok(result)
    }
}

/**
 * Region calculation via MCB
 */
impl BBGraph {
    /// Adds a region to the BBGraph returning its BBRegionIndex
    ///
    /// * `region`: The region to add
    fn add_region(&mut self, region: BBRegion) -> BBRegionIndex {
        let index = BBRegionIndex(self.get_next_idx());
        self.regions.insert(index, region);
        index
    }

    /// Recalculates all of the regions for the bb_graph.  This will make any pre-existing `BBRegionIndex` invalid.
    pub fn update_regions(&mut self) -> BBResult<Vec<BBRegionIndex>> {
        self.regions.clear();

        let mut region_indices = vec![];

        for mut graph in self.get_detached_graphs()? {
            graph.remove_filaments()?;

            let Some(start_id) = graph.get_left_most_node_index() else {
                continue;
            };

            let perimiter = match graph.closed_walk_with_ccw_start_and_ccw_traverse(start_id) {
                Ok(perimiter) => perimiter,
                Err(reason) => {
                    println!("Skipping graph for reason {reason:?}.");
                    continue;
                }
            };
            let mut cycle = BBCycle::new(perimiter.edges);

            graph.extract_cycles(&mut cycle)?;

            let region = BBRegion::new(cycle);
            region_indices.push(self.add_region(region));
        }

        Ok(region_indices)
    }

    fn extract_cycles(&mut self, parent_cycle: &mut BBCycle) -> BBResult<()> {
        while self.nodes_count() > 0 {
            // Need to cleanup filaments as it can prevent closed walks
            self.remove_filaments()?;
            if self.nodes_count() <= 2 || self.edges_count() <= 2 {
                break;
            }

            let Some(left_most) = self.get_left_most_node_index() else {
                break;
            };
            let ClosedWalkResult { outer_edge, edges } =
                self.closed_walk_with_ccw_start_and_ccw_traverse(left_most)?;

            let (edges, nested_edges) =
                self.extract_nested_cycle_from_closed_walk(&edges)?;

            let mut cycle = BBCycle::new(edges);
            for walk in nested_edges {
                let mut nested_graph = BBGraph::try_new_from_other_edges(self, &walk)?;
                let result = nested_graph.extract_cycles(&mut cycle);

                // We will not throw all errors, only if they're critical (of the missing variant).
                if let Err(reason) = result {
                    if reason.is_missing_variant() {
                        return Err(reason);
                    }
                }
            }

            self.delete_edge(outer_edge)?;

            parent_cycle.children.push(cycle);
        }

        Ok(())
    }

    fn extract_nested_cycle_from_closed_walk(
        &self,
        closed_walk: &[BBEdgeIndex],
    ) -> BBResult<(Vec<BBEdgeIndex>, Vec<Vec<BBEdgeIndex>>)> {
        let mut closed_walk = self.edges_directed(closed_walk)?;
        let mut nested_closed_walk = vec![];

        let mut next_i = 2;
        while next_i < closed_walk.len() {
            let i = next_i - 2;
            let (_, edge) = closed_walk[i];

            // Need to find nodes that are traversed twice as this indicates a nested cycle.
            let mut nested_range = None;
            for (end_i, (other_edge_idx, other_edge)) in
                closed_walk[next_i..].iter().enumerate().rev()
            {
                let is_not_first_el = i != 0;
                let is_not_last_el = end_i != closed_walk.len();
                let is_traversed_twice = edge.start_idx() == other_edge.end_idx();
                if is_not_first_el && is_not_last_el && is_traversed_twice {
                    nested_range = Some(i..(end_i + next_i + 1));
                    break;
                }
            }

            if let Some(nested_range) = nested_range {
                let start = nested_range.start;
                let end = nested_range.end;

                // Need to delete the nested from the parent cycle.
                let nested_walk: Vec<_> = closed_walk.drain(nested_range).collect();
                nested_closed_walk.push(nested_walk.into_iter().map(|(idx, _)| idx).collect());
            }

            next_i += 1;
        }

        let parent_closed_walk: Vec<_> = closed_walk.into_iter().map(|(idx, _)| idx).collect();
        Ok((parent_closed_walk, nested_closed_walk))
    }
}
