use std::collections::HashSet;

use glam::Vec2;

use lyon_path::{builder::NoAttributes, math::Point, path::BuilderImpl, traits::PathBuilder, Path};

use crate::{bb_graph::TraverseAction, prelude::*};

trait ToPoint {
    fn to_p2(&self) -> Point;
}

impl ToPoint for Vec2 {
    fn to_p2(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

struct StrokeTraverseModel {
    curr_node_idx: BBNodeIndex,
    visited: HashSet<BBEdgeIndex>,
    needs_end: bool,
    builder: NoAttributes<BuilderImpl>,
}

impl BBGraph {
    fn build_path_for_edge_list(
        &self,
        builder: &mut NoAttributes<BuilderImpl>,
        edges: &[BBEdgeIndex],
    ) -> BBResult<()> {
        if edges.len() < 1 {
            return Err(BBError::ClosedWalkTooSmall(1));
        };
        let directed = self.edges_directed(edges)?;

        let (_, first_edge) = directed.first().unwrap();
        builder.begin(first_edge.start_pos(self).to_p2());

        for (_, edge) in &directed {
            match edge {
                BBEdge::Line { start: _, end } => {
                    builder.line_to(self.node(*end)?.position().to_p2());
                }
                BBEdge::Quadratic {
                    start: _,
                    ctrl1,
                    end,
                } => {
                    let ctrl1 = ctrl1.to_p2();
                    let end = self.node(*end)?.position().to_p2();
                    builder.quadratic_bezier_to(ctrl1, end);
                }
                BBEdge::Cubic {
                    start: _,
                    ctrl1,
                    ctrl2,
                    end,
                } => {
                    let ctrl1 = ctrl1.to_p2();
                    let ctrl2 = ctrl2.to_p2();
                    let end = self.node(*end)?.position().to_p2();
                    builder.cubic_bezier_to(ctrl1, ctrl2, end);
                }
            }
        }

        builder.end(false);
        Ok(())
    }

    pub fn generate_fill_path(&self) -> BBResult<Path> {
        let mut builder = Path::builder();

        for region in self.regions.values() {
            for cycle in region.cycles.iter() {
                self.build_path_for_edge_list(&mut builder, &cycle.edges)?
            }
        }

        Ok(builder.build())
    }

    pub fn generate_stroke_path(&self) -> BBResult<Path> {
        let mut g = self.clone();

        let mut builder = Path::builder();

        // Add the paths for all the cycles.
        let cycles: Vec<_> = g
            .regions
            .values()
            .flat_map(|r| r.cycles.iter().map(|c| c.edges.clone()))
            .collect();
        for edges in cycles {
            g.build_path_for_edge_list(&mut builder, &edges)?;
            for e in edges {
                g.delete_edge(e)?;
            }
        }

        // Add paths for all the filaments
        loop {
            // Find a node with 1 adjacent (filament)
            let Some(mut node_idx) = g.nodes.iter().find_map(|(idx, node)| {
                if node.adjacents.len() == 1 {
                    Some(*idx)
                } else {
                    None
                }
            }) else {
                break;
            };

            // Walk along the edges until you hit an end or a fork.
            let mut edges = vec![];
            loop {
                let node = g.node(node_idx)?;
                if !edges.is_empty() && node.adjacents.len() != 2 {
                    break;
                }
                let edge_idx = node.adjacents.first().unwrap();
                edges.push(*edge_idx);
                let edge = g.edge(*edge_idx)?;
                node_idx = edge.other_node_idx(node_idx);
            }

            println!("generate_stroke_path: {edges:?}");
            // Build and remove from graph.
            g.build_path_for_edge_list(&mut builder, &edges)?;
            for e in edges {
                g.delete_edge(e)?;
            }
        }

        Ok(builder.build())
    }
}
