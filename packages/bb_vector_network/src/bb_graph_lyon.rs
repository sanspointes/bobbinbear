use std::collections::HashSet;

use glam::Vec2;

use lyon_path::{builder::NoAttributes, math::Point, path::BuilderImpl, Path};

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
    builder: NoAttributes<BuilderImpl>,
}

impl BBGraph {
    pub fn generate_fill_path(&self) -> BBResult<Path> {
        let mut builder = Path::builder();

        for region in self.regions.values() {
            for cycle in region.cycles.iter() {
                // TODO: Support nested cycles
                let directed_edges = self.edges_directed(&cycle.edges)?;

                let (_, first_edge) = directed_edges.first().unwrap();
                let first_node = first_edge.start_idx();
                builder.begin(self.node(first_node)?.position().to_p2());

                for (_, edge) in directed_edges {
                    match edge {
                        BBEdge::Line { start: _, end } => {
                            builder.line_to(self.node(end)?.position().to_p2());
                        }
                        BBEdge::Quadratic {
                            start: _,
                            ctrl1,
                            end,
                        } => {
                            let ctrl1 = ctrl1.to_p2();
                            let end = self.node(end)?.position().to_p2();
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
                            let end = self.node(end)?.position().to_p2();
                            builder.cubic_bezier_to(ctrl1, ctrl2, end);
                        }
                    }
                }
                builder.end(true);
            }
        }

        Ok(builder.build())
    }

    pub fn generate_stroke_path(&self) -> BBResult<Path> {
        let Some(left_most) = self.get_left_most_node_index() else {
            return Err(crate::prelude::BBError::EmptyGraph);
        };

        let mut builder = Path::builder();
        builder.begin(self.node(left_most)?.position().to_p2());

        let model = StrokeTraverseModel {
            curr_node_idx: left_most,
            visited: HashSet::new(),
            builder,
        };

        let traversal =
            self.traverse_with_model::<StrokeTraverseModel, StrokeTraverseModel>(model, |model| {
                let n = self.node(model.curr_node_idx)?;
                // // If the next edge we're tesselating is not connected to the previous, we'll need
                // // to use the move command before we continue.
                // let mut needs_move = false;
                // Tries to trivially continue the path from current node
                let mut maybe_next_edge = n
                    .adjacents()
                    .iter()
                    .find(|edge_idx| !model.visited.contains(edge_idx))
                    .cloned();

                // Fallback to finding an edge adjacent to an already visited edge
                if maybe_next_edge.is_none() {
                    let next_node_and_edge = model.visited.iter().find_map(|edge_idx| {
                        let e = self.edge(*edge_idx).unwrap();
                        let edge_adjacents = e.adjacents(&self).unwrap();
                        let next_edge_idx = edge_adjacents
                            .iter()
                            .find(|edge_idx| !model.visited.contains(edge_idx))?;

                        let next_e = self.edge(*next_edge_idx).unwrap();
                        let curr_node_idx = e.shared_node(next_e)?;

                        Some((curr_node_idx, *next_edge_idx))
                    });

                    if let Some((next_node_idx, next_edge_idx)) = next_node_and_edge {
                        model.builder.end(false);
                        model
                            .builder
                            .begin(self.node(next_node_idx)?.position().to_p2());
                        model.curr_node_idx = next_node_idx;
                        maybe_next_edge = Some(next_edge_idx);
                    }
                }

                if maybe_next_edge.is_none() {
                    return Ok(TraverseAction::Stop);
                }

                if let Some(next_edge) = maybe_next_edge {
                    let mut e = *self.edge(next_edge)?;
                    if e.end_idx() == model.curr_node_idx {
                        e = e.reversed();
                    }

                    match e {
                        BBEdge::Line { start: _, end } => {
                            model.builder.line_to(self.node(end)?.position().to_p2());
                        }
                        BBEdge::Quadratic {
                            start: _,
                            ctrl1,
                            end,
                        } => {
                            let ctrl1 = ctrl1.to_p2();
                            let end = self.node(end)?.position().to_p2();
                            model.builder.quadratic_bezier_to(ctrl1, end);
                        }
                        BBEdge::Cubic {
                            start: _,
                            ctrl1,
                            ctrl2,
                            end,
                        } => {
                            let ctrl1 = ctrl1.to_p2();
                            let ctrl2 = ctrl2.to_p2();
                            let end = self.node(end)?.position().to_p2();
                            model.builder.cubic_bezier_to(ctrl1, ctrl2, end);
                        }
                    }

                    model.visited.insert(next_edge);
                    model.curr_node_idx = self.edge(next_edge)?.other_node_idx(model.curr_node_idx);
                } else {
                    model.builder.end(false);
                    return Ok(TraverseAction::Stop);
                }

                if model.visited.len() == self.edges.len() {
                    model.builder.end(false);
                    Ok(TraverseAction::Stop)
                } else {
                    Ok(TraverseAction::Continue)
                }
            })?;

        Ok(traversal.builder.build())
    }
}
