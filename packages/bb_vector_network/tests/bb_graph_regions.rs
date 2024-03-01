mod common;

use bb_vector_network::bb_graph::BBGraph;
use common::draw::draw_graph;
use glam::vec2;

use crate::common::{draw::SnapshotCtx, fuzz_snapshot::FuzzSnapshot};

#[test]
pub fn simple_line_graph() {
    let mut g = BBGraph::new();

    let (_, f) = g.line(vec2(20., 20.), vec2(100., 20.));
    // Right section
    let (_, p) = g.line_from(f.end_idx(), vec2(180., 20.));
    let (_, p) = g.line_from(p.end_idx(), vec2(180., 180.));
    let (_, c) = g.line_from(p.end_idx(), vec2(100., 180.));
    // middle edge
    g.line_from_to(c.end_idx(), f.end_idx());
    // Left section
    let (_, p) = g.line_from(c.end_idx(), vec2(20., 180.));
    g.line_from_to(p.end_idx(), f.start_idx());

    let fuzz = FuzzSnapshot::new(g);
    fuzz.run_reversed_edges(|_key, g| {
        let mut cx = SnapshotCtx::default();

        draw_graph(&mut cx, &g).unwrap();

        let path = "./tests/images/regions_simple_line_graph.png".to_string();

        let difference = cx.save_or_difference_with_disk(&path);

        difference < 0.01
    });
}
