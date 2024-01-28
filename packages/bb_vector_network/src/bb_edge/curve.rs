use flo_curves::{
    bezier::{BezierCurve2D, Curve, CurveCategory},
    BezierCurve, BezierCurveFactory, Coord2,
};
use glam::Vec2;

use crate::{
    bb_node::BBNodeIndex,
    prelude::{BBGraph, BBResult},
};

use super::{BBEdge, BBEdgeIndex};

pub fn v2_to_c2(v: Vec2) -> Coord2 {
    Coord2(v.x as f64, v.y as f64)
}
pub fn c2_to_v2(c: Coord2) -> Vec2 {
    Vec2::new(c.0 as f32, c.1 as f32)
}

impl BBEdge {
    pub fn as_curve(&self, graph: &BBGraph) -> Curve<Coord2> {
        match self {
            Self::Line { start, end } => {
                let start = graph.node(*start).unwrap().position();
                let end = graph.node(*end).unwrap().position();
                let ctrl1 = start.lerp(end, 0.33333);
                let ctrl2 = start.lerp(end, 0.66666);
                Curve::from_points(
                    v2_to_c2(start),
                    (v2_to_c2(ctrl1), v2_to_c2(ctrl2)),
                    v2_to_c2(end),
                )
            }
            Self::Quadratic { start, ctrl1, end } => {
                let start = graph.node(*start).unwrap().position();
                let end = graph.node(*end).unwrap().position();
                Curve::from_points(
                    v2_to_c2(start),
                    (v2_to_c2(*ctrl1), v2_to_c2(*ctrl1)),
                    v2_to_c2(end),
                )
            }
            Self::Cubic {
                start,
                ctrl1,
                ctrl2,
                end,
            } => {
                let start = graph.node(*start).unwrap().position();
                let end = graph.node(*end).unwrap().position();
                Curve::from_points(
                    v2_to_c2(start),
                    (v2_to_c2(*ctrl1), v2_to_c2(*ctrl2)),
                    v2_to_c2(end),
                )
            }
        }
    }
    pub fn subdivide_without_delete(&self, graph: &mut BBGraph, t: f32) -> BBResult<()> {
        let curve = self.as_curve(graph);

        let (curve1, curve2): (Curve<Coord2>, Curve<Coord2>) = curve.subdivide(t as f64);

        let middle_position = curve1.end_point();
        let middle_node_idx = graph.add_node(c2_to_v2(middle_position));

        curve1.add_to_graph_with_nodes(graph, Some(self.start_idx()), Some(middle_node_idx));
        curve2.add_to_graph_with_nodes(graph, Some(middle_node_idx), Some(self.end_idx()));

        Ok(())
    }
}

pub trait BBEdgeCurveHelpers {
    fn add_to_graph_with_nodes(
        &self,
        graph: &mut BBGraph,
        start_node: Option<BBNodeIndex>,
        end_node: Option<BBNodeIndex>,
    ) -> (BBEdgeIndex, BBEdge);
}

impl BBEdgeCurveHelpers for Curve<Coord2> {
    fn add_to_graph_with_nodes(
        &self,
        graph: &mut BBGraph,
        start_node: Option<BBNodeIndex>,
        end_node: Option<BBNodeIndex>,
    ) -> (BBEdgeIndex, BBEdge) {
        let characteristics = self.characteristics();

        println!("Adding {self:?} to graph.  characteristics: {characteristics:?}");
        match (characteristics, start_node, end_node) {
            (CurveCategory::Linear, Some(start), Some(end)) => graph.line_from_to(start, end),
            (CurveCategory::Linear, None, Some(end)) => {
                graph.line_to(c2_to_v2(self.start_point), end)
            }
            (CurveCategory::Linear, Some(start), None) => {
                graph.line_from(start, c2_to_v2(self.end_point))
            }
            (CurveCategory::Linear, None, None) => {
                graph.line(c2_to_v2(self.start_point), c2_to_v2(self.end_point))
            }
            (_, start, end) => {
                let (ctrl1, ctrl2) = self.control_points();
                match (start, end) {
                    (Some(start), Some(end)) => {
                        graph.cubic_from_to(start, c2_to_v2(ctrl1), c2_to_v2(ctrl2), end)
                    }
                    (None, Some(end)) => graph.cubic_to(
                        c2_to_v2(self.start_point),
                        c2_to_v2(ctrl1),
                        c2_to_v2(ctrl1),
                        end,
                    ),
                    (Some(start), None) => graph.cubic_from(
                        start,
                        c2_to_v2(ctrl1),
                        c2_to_v2(ctrl2),
                        c2_to_v2(self.end_point),
                    ),
                    (None, None) => graph.cubic(
                        c2_to_v2(self.start_point),
                        c2_to_v2(ctrl1),
                        c2_to_v2(ctrl2),
                        c2_to_v2(self.end_point),
                    ),
                }
            }
        }
    }
}
