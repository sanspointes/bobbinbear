use flo_curves::{bezier::Curve, BezierCurveFactory, Coord2};
use glam::Vec2;

use crate::prelude::BBGraph;

use super::BBEdge;

fn v2_to_coord2(v: Vec2) -> Coord2 {
    Coord2(v.x as f64, v.y as f64)
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
                    v2_to_coord2(start),
                    (v2_to_coord2(ctrl1), v2_to_coord2(ctrl2)),
                    v2_to_coord2(end),
                )
            }
            Self::Quadratic { start, ctrl1, end } => {
                let start = graph.node(*start).unwrap().position();
                let end = graph.node(*end).unwrap().position();
                Curve::from_points(
                    v2_to_coord2(start),
                    (v2_to_coord2(*ctrl1), v2_to_coord2(*ctrl1)),
                    v2_to_coord2(end),
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
                    v2_to_coord2(start),
                    (v2_to_coord2(*ctrl1), v2_to_coord2(*ctrl2)),
                    v2_to_coord2(end),
                )
            }
        }
    }
}
