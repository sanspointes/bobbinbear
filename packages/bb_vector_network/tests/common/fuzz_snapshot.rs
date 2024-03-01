use std::{collections::HashSet};

use bb_vector_network::{bb_edge::{BBEdge, BBEdgeIndex}, bb_graph::BBGraph, bb_node::BBNodeIndex};
use glam::Vec2;
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[allow(dead_code)]
pub struct FuzzSnapshot {
    graph: BBGraph,
    protected_edges: HashSet<BBEdgeIndex>,
}

#[allow(dead_code)]
impl FuzzSnapshot {
    pub fn new(graph: BBGraph) -> Self {
        Self {
            graph,
            protected_edges: HashSet::new(),
        }
    }

    pub fn with_protected_edges(
        mut self,
        protected: impl IntoIterator<Item = BBEdgeIndex>,
    ) -> Self {
        self.protected_edges.extend(protected);
        self
    }

    fn random_vec2(rand: &mut impl Rng, range: f32) -> Vec2 {
        let mut x = rand.gen::<u32>() as f32 / u32::MAX as f32;
        let mut y = rand.gen::<u32>() as f32 / u32::MAX as f32;
        x = (x * range) - range / 2.;
        y = (y * range) - range / 2.;
        Vec2::new(x, y)
    }

    fn randomize_graph_positions(graph: &mut BBGraph, range: f32, seed: u64) {
        let mut rand = ChaCha8Rng::seed_from_u64(seed);

        let mut node_idxes = vec![];
        for i in 0..graph.nodes_count() {
            node_idxes.push(BBNodeIndex(i));
        }
        for id in node_idxes {
            let Ok(node) = graph.node_mut(id) else {
                continue;
            };
            let pos = node.position();
            let r = Self::random_vec2(&mut rand, range);
            node.set_position(pos + r);
        }

        let mut edge_idxes = vec![];
        for i in 0..graph.edges_count() {
            edge_idxes.push(BBEdgeIndex(i));
        }
        for id in edge_idxes {
            let Ok(edge) = graph.edge_mut(id) else {
                continue;
            };
            match edge {
                BBEdge::Line { .. } => {},
                BBEdge::Quadratic { ref mut ctrl1, .. } => {
                    let r = Self::random_vec2(&mut rand, range);
                    *ctrl1 += r;
                }
                BBEdge::Cubic { ref mut ctrl1, ref mut ctrl2, .. } => {
                    let r = Self::random_vec2(&mut rand, range);
                    *ctrl1 += r;
                    let r = Self::random_vec2(&mut rand, range);
                    *ctrl2 += r;
                }
            }
        }
    }

    pub fn run_with_randomized_positions(self, variations: u64, range: f32, test: impl Fn(String, BBGraph) -> bool) {
        let mut vars = vec![];
        for i in 0..variations {
            let mut g = self.graph.clone();
            Self::randomize_graph_positions(&mut g, range, i);
            vars.push(g);
        }

        let results = vars.into_iter().enumerate().map(|(i, g)| {
            let key = format!("{i:04}");
            let did_pass = (test)(key.clone(), g);
            (key, did_pass)
        });

        // Collect failed cases
        let errors: Vec<_> = results
            .filter_map(|(key, did_pass)| match did_pass {
                true => None,
                false => Some(key),
            })
            .collect();

        if !errors.is_empty() {
            panic!(
                "Failed fuzz test: Problem keys {}.",
                errors.iter().join(", ")
            );
        }
    }
    
    /// Creates a new graph for every variation of reversed edges possible.
    /// Calls the runner to return true or false.
    ///
    /// * `test`: Test function receiving (unique string, graph variation), returns true (pass) or
    /// false (fail)
    pub fn run_reversed_edges(self, test: impl Fn(String, BBGraph) -> bool) {
        let deletable_edges: Vec<_> = self
            .graph
            .edges
            .keys()
            .filter(|id| !self.protected_edges.contains(id))
            .cloned()
            .collect();

        // Build all test cases
        let mut cases: HashSet<Vec<BBEdgeIndex>> = HashSet::new();
        for r in 1..deletable_edges.len() {
            for edges in deletable_edges.iter().combinations(r) {
                let mut edges: Vec<_> = edges.into_iter().copied().collect();
                edges.sort();

                cases.insert(edges);
            }
        }

        // Apply test cases to graph and run
        let results = cases.iter().map(|case| {
            let mut g = self.graph.clone();
            for e in case {
                // Reverse the edges
                let edge = *g.edges.get_mut(e).unwrap();
                g.edges.insert(*e, edge.reversed());
            }
            g.update_regions().unwrap();
            let key = case.iter().map(|v| format!("{}", v.0)).join("-");
            let did_pass = (test)(key.clone(), g);
            (key, did_pass)
        });

        // Collect failed cases
        let errors: Vec<_> = results
            .filter_map(|(key, did_pass)| match did_pass {
                true => None,
                false => Some(key),
            })
            .collect();

        if !errors.is_empty() {
            panic!(
                "Failed fuzz test: Problem keys {}.",
                errors.iter().join(", ")
            );
        }
    }

    /// Creates a new graph for every variation of deleted edges possible.
    /// Calls the runner to return true or false.
    ///
    /// * `test`:
    pub fn run_deleted_edges(self, test: impl Fn(String, BBGraph) -> bool) {
        let deletable_edges: Vec<_> = self
            .graph
            .edges
            .keys()
            .filter(|id| !self.protected_edges.contains(id))
            .cloned()
            .collect();

        let mut cases: HashSet<Vec<BBEdgeIndex>> = HashSet::new();
        for r in 1..deletable_edges.len() {
            for edges in deletable_edges.iter().combinations(r) {
                let mut edges: Vec<_> = edges.into_iter().copied().collect();
                edges.sort();

                cases.insert(edges);
            }
        }

        let results = cases.iter().map(|case| {
            let mut g = self.graph.clone();
            for e in case {
                let _ = g.delete_edge(*e);
            }
            let key = case.iter().map(|v| format!("{}", v.0)).join("-");
            let did_pass = (test)(key.clone(), g);
            (key, did_pass)
        });

        let errors: Vec<_> = results
            .filter_map(|(key, did_pass)| match did_pass {
                true => None,
                false => Some(key),
            })
            .collect();

        if !errors.is_empty() {
            panic!(
                "Failed fuzz test: Problem keys {}.",
                errors.iter().join("-")
            );
        }
    }
}
