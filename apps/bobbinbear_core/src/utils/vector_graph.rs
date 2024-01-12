use bb_vector_network::bb_graph::BBGraph;
use bevy::{prelude::*, math::vec2};

use crate::plugins::vector_graph_plugin::VectorGraph;

pub fn build_vector_graph_box(origin: Vec2, size: Vec2) -> VectorGraph {
    let mut g = BBGraph::new();
    let (_, f) = g.line(vec2(0., 0.), vec2(0., size.y));
    let (_, e) = g.line_from(f.end_idx(), vec2(size.x, size.y));
    let (_, e) = g.line_from(e.end_idx(), vec2(size.x, 0.));
    let _ = g.line_from_to(e.end_idx(), f.start_idx());

    VectorGraph(g)
}
