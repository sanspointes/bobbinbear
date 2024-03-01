#[allow(unused_variables)]

mod common;

pub mod cw {
    use std::f32::consts::PI;

    use bb_vector_network::bb_graph::BBGraph;
    use glam::vec2;

    use crate::common::{draw::{draw_edge, draw_edge_list, SnapshotCtx}, fuzz_snapshot::FuzzSnapshot};

    #[test]
    pub fn fuzz_cw() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(20., 100.), vec2(100., 100.));
        g.line_from(f.end_idx(), vec2(120., 120.));
        g.line_from(f.end_idx(), vec2(80., 80.));
        g.line_from(f.end_idx(), vec2(80., 120.));

        let curr_node = f.end_idx();
        let curr_dir = f.calc_end_tangent(&g).unwrap();
        let fuzz_snapshot = FuzzSnapshot::new(g).with_protected_edges([e0]);
        fuzz_snapshot.run_deleted_edges(|id, graph| {
            let edge = graph.get_cw_edge_of_node(curr_node, curr_dir, Some(e0)).unwrap();

            let mut cx = SnapshotCtx::default();

            let edges: Vec<_> = graph.edges.keys().cloned().collect();
            draw_edge_list(&mut cx, &graph, edges.as_slice()).unwrap();

            cx.with_stroke_color(255, 0, 0, 255);
            draw_edge(&mut cx, &graph, edge).unwrap();

            let path = format!("./tests/images/cw_fuzz_{id}.png");
            
            let diff = cx.save_or_difference_with_disk(&path);
            diff < 0.01
        })
    }

    #[test]
    pub fn fuzz_cw_rotated() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(20., 100.), vec2(100., 100.));
        g.line_from(f.end_idx(), vec2(120., 120.));
        g.line_from(f.end_idx(), vec2(80., 80.));
        g.line_from(f.end_idx(), vec2(80., 120.));

        g.rotate(vec2(100., 100.), PI / 2.);

        let curr_node = f.end_idx();
        let curr_dir = f.calc_end_tangent(&g).unwrap();
        let fuzz_snapshot = FuzzSnapshot::new(g).with_protected_edges([e0]);
        fuzz_snapshot.run_deleted_edges(|id, graph| {
            let edge = graph.get_cw_edge_of_node(curr_node, curr_dir, Some(e0)).unwrap();

            let mut cx = SnapshotCtx::default();

            let edges: Vec<_> = graph.edges.keys().cloned().collect();
            draw_edge_list(&mut cx, &graph, edges.as_slice()).unwrap();

            cx.with_stroke_color(255, 0, 0, 255);
            draw_edge(&mut cx, &graph, edge).unwrap();

            let path = format!("./tests/images/cw_fuzz_rotated_{id}.png");
            
            let diff = cx.save_or_difference_with_disk(&path);
            diff < 0.01
        })
    }
}

pub mod ccw {
    use std::f32::consts::PI;

    use bb_vector_network::bb_graph::BBGraph;
    use glam::vec2;

    use crate::common::{draw::{draw_edge, draw_edge_list, SnapshotCtx}, fuzz_snapshot::FuzzSnapshot};

    #[test]
    pub fn fuzz_ccw() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(20., 100.), vec2(100., 100.));
        g.line_from(f.end_idx(), vec2(120., 120.));
        g.line_from(f.end_idx(), vec2(80., 80.));
        g.line_from(f.end_idx(), vec2(80., 120.));

        let curr_node = f.end_idx();
        let curr_dir = f.calc_end_tangent(&g).unwrap();
        let fuzz_snapshot = FuzzSnapshot::new(g).with_protected_edges([e0]);
        fuzz_snapshot.run_deleted_edges(|id, graph| {
            let edge = graph.get_ccw_edge_of_node(curr_node, curr_dir, Some(e0)).unwrap();

            let mut cx = SnapshotCtx::default();

            let edges: Vec<_> = graph.edges.keys().cloned().collect();
            draw_edge_list(&mut cx, &graph, edges.as_slice()).unwrap();

            cx.with_stroke_color(255, 0, 0, 255);
            draw_edge(&mut cx, &graph, edge).unwrap();

            let path = format!("./tests/images/ccw_fuzz_{id}.png");
            
            let diff = cx.save_or_difference_with_disk(&path);
            diff < 0.01
        })
    }

    #[test]
    pub fn fuzz_ccw_rotated() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(20., 100.), vec2(100., 100.));
        g.line_from(f.end_idx(), vec2(120., 120.));
        g.line_from(f.end_idx(), vec2(80., 80.));
        g.line_from(f.end_idx(), vec2(80., 120.));

        g.rotate(vec2(100., 100.), PI / 2.);

        let curr_node = f.end_idx();
        let curr_dir = f.calc_end_tangent(&g).unwrap();
        let fuzz_snapshot = FuzzSnapshot::new(g).with_protected_edges([e0]);
        fuzz_snapshot.run_deleted_edges(|id, graph| {
            let edge = graph.get_ccw_edge_of_node(curr_node, curr_dir, Some(e0)).unwrap();

            let mut cx = SnapshotCtx::default();

            let edges: Vec<_> = graph.edges.keys().cloned().collect();
            draw_edge_list(&mut cx, &graph, edges.as_slice()).unwrap();

            cx.with_stroke_color(255, 0, 0, 255);
            draw_edge(&mut cx, &graph, edge).unwrap();

            let path = format!("./tests/images/ccw_fuzz_rotated_{id}.png");
            
            let diff = cx.save_or_difference_with_disk(&path);
            diff < 0.01
        })
    }
}
