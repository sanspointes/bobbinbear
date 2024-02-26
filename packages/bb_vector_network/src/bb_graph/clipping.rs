use std::collections::HashMap;
use flo_curves::{
    bezier::{curve_intersects_curve_clip, Curve},
    BezierCurve, BoundingBox, Bounds, Coord2, Coordinate,
};
use glam::Vec2;

use crate::{bb_edge::{c2_to_v2, BBEdgeCurveHelpers}, prelude::*};

const MIN_DISTANCE_TO: f32 = 0.0001;
const MIN_DISTANCE_TO_F64: f64 = 0.0001;

// // Key for storing an intersection from edge 0 to edge 1
// type EdgeToEdgeKey = (BBEdgeIndex, BBEdgeIndex);
// // Value for the EdgeToEdgeKey key, stores the positions along edge 0 where it hit edge 1.
// // The Option<f64> stores the t_position that the edge was hit at.
// type IntersectionResult = Vec<(f64, f64, Vec2)>;
//
#[derive(Debug)]
struct Intersection {
    pub other_idx: BBEdgeIndex,
    pub self_t: f32,
    pub other_t: f32,
    pub pos: Vec2,
}

pub fn map_linear(v: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (v - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

impl BBGraph {
    ///
    pub fn expand_intersections(&mut self) -> BBResult<()> {
        let beziers: Vec<(BBEdgeIndex, Curve<Coord2>, Bounds<Coord2>)> = self
            .edges
            .iter()
            .map(|(idx, edge)| {
                let bezier = edge.as_curve(self);
                let bounds = bezier.bounding_box();
                (*idx, bezier, bounds)
            })
            .collect();

        // Collect all edge -> edge intersections
        let mut intersections: HashMap<BBEdgeIndex, Vec<Intersection>> = HashMap::new();
        for i in 0..beziers.len() {
            for j in 0..beziers.len() {
                if i == j {
                    continue;
                }
                let (idx_a, bez_a, rect_a) = &beziers[i];
                let (idx_b, bez_b, rect_b) = &beziers[j];
                if !rect_a.overlaps(rect_b) {
                    continue;
                }

                let ints: Vec<_> = curve_intersects_curve_clip(bez_a, bez_b, 0.01)
                    .iter()
                    .filter(|(ta, tb)| {
                        let ta_is_at_end = ta.distance_to(&1.) < MIN_DISTANCE_TO_F64
                            || ta.distance_to(&0.) < MIN_DISTANCE_TO_F64;
                        let tb_is_at_end = tb.distance_to(&1.) < MIN_DISTANCE_TO_F64
                            || tb.distance_to(&0.) < MIN_DISTANCE_TO_F64;
                        !(ta_is_at_end || tb_is_at_end)
                    })
                    .map(|(ta, tb)| {
                        let p = bez_a.point_at_pos(*ta);
                        Intersection {
                            other_idx: *idx_b,
                            self_t: *ta as f32,
                            other_t: *tb as f32,
                            pos: c2_to_v2(p),
                        }
                    }).collect();
                if ints.is_empty() {
                    continue;
                }

                let stored_ints = intersections.entry(*idx_a).or_insert(vec![]);
                stored_ints.extend(ints)
            }
        }

        for (_, values) in intersections.iter_mut() {
            values.sort_by(|a, b| a.self_t.partial_cmp(&b.self_t).unwrap());
        }

        // Apply them to self
        let mut endpoint_lookup: HashMap<BBEdgeIndex, Vec<(BBNodeIndex, Vec2)>> = HashMap::new();
        for (idx_a, ints) in &intersections {
            let edge = *self.edge(*idx_a).unwrap();
            // 
            let mut prev_node = edge.start_idx();
            let mut remaining_curve = edge.as_curve(self);
            let mut t_offset = 0.0;

            for int in ints {
                let intersect_pos = remaining_curve.point_at_pos(int.self_t as f64);
                let intersect_pos = c2_to_v2(intersect_pos);
                let existing_end_node = endpoint_lookup
                    .get(&int.other_idx)
                    .and_then(|nodes| {
                        nodes
                            .iter()
                            .find(|(idx, pos)| pos.distance(intersect_pos) < MIN_DISTANCE_TO)
                    })
                    .map(|(idx, _)| idx)
                    .copied();

                let t_mapped = map_linear(int.self_t, t_offset, 1., 0., 1.);
                let (to_add, remaining) = remaining_curve.subdivide::<Curve<Coord2>>(t_mapped as f64);

                let (_, added_edge) = to_add.add_to_graph_with_nodes(self, Some(prev_node), existing_end_node);
                prev_node = added_edge.end_idx();

                t_offset += int.self_t;
                remaining_curve = remaining;

                // Push added node to the lookup so it's re-used for future iters.
                let lookup_entry = endpoint_lookup.entry(*idx_a);
                let edge_nodes = lookup_entry.or_insert(vec![]);
                edge_nodes.push((prev_node, int.pos));
            }

            remaining_curve.add_to_graph_with_nodes(self, Some(prev_node), Some(edge.end_idx()));
        }

        for (idx, _) in endpoint_lookup {
            let edge = self.delete_edge(idx);
        }

        Ok(())
    }
}
