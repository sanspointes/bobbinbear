use std::{
    collections::{HashMap, HashSet},
    result,
};

use bb_vector_network::{bb_edge::BBEdgeIndex, bb_graph::BBGraph};
use itertools::Itertools;
use rand::prelude::*;

pub struct FuzzSnapshot {
    graph: BBGraph,
    protected_edges: HashSet<BBEdgeIndex>,
}

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

    pub fn run(self, test: impl Fn(String, BBGraph) -> bool) {
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

        let results = cases
            .iter()
            .map(|case| {
                let mut g = self.graph.clone();
                for e in case {
                    let _ = g.delete_edge(*e);
                }
                let key = case.iter().map(|v| format!("{}", v.0)).join("-");
                let did_pass = (test)(key.clone(), g);
                (key, did_pass)
            });

        let errors: Vec<_> = results.filter_map(|(key, did_pass)| {
                match did_pass {
                    true => None,
                    false => Some(key),
                }
            }).collect();

        if !errors.is_empty() {
            panic!("Failed fuzz test: Problem keys {}.", errors.iter().join("-"));
        }
    }
}
