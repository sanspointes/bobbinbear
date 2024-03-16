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

            let path = format!("./tests/images/next_edge__cw.{id}.png");
            
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

            let path = format!("./tests/images/next_edge__rotated_cw.{id}.png");
            
            let diff = cx.save_or_difference_with_disk(&path);
            diff < 0.01
        })
    }

    #[test]
    pub fn parallel_fuzz() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(20., 100.), vec2(100., 100.));
        g.quadratic_from(f.end_idx(), vec2(150., 100.), vec2(150., 150.));
        g.quadratic_from(f.end_idx(), vec2(150., 100.), vec2(150., 50.));
        g.quadratic_from(f.end_idx(), vec2(50., 100.), vec2(50., 50.));
        g.quadratic_from(f.end_idx(), vec2(50., 100.), vec2(50., 150.));

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

            let path = format!("./tests/images/next_edge__parallel_fuzz_cw.{id}.png");
            
            let diff = cx.save_or_difference_with_disk(&path);
            diff < 0.01
        });
    }

    #[test]
    pub fn parallel_1() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(20., 100.), vec2(100., 100.));
        let (e_expected, _) = g.quadratic_from(f.end_idx(), vec2(50., 100.), vec2(50., 50.));
        g.quadratic_from(f.end_idx(), vec2(40., 100.), vec2(40., 150.));

        let curr_node = f.end_idx();
        let curr_dir = f.calc_end_tangent(&g).unwrap();
        let edge = g.get_cw_edge_of_node(curr_node, curr_dir, Some(e0)).unwrap();

        let mut cx = SnapshotCtx::default();

        let edges: Vec<_> = g.edges.keys().cloned().collect();
        draw_edge_list(&mut cx, &g, edges.as_slice()).unwrap();

        cx.with_stroke_color(255, 0, 0, 255);
        draw_edge(&mut cx, &g, edge).unwrap();

        let path = format!("./tests/images/next_edge__parallel_1_cw.png");
        
        let diff = cx.save_or_difference_with_disk(&path);
        assert!(diff < 0.01);
        assert_eq!(edge, e_expected);
    }

    #[test]
    pub fn loop_back_1() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(20., 100.), vec2(100., 100.));
        g.cubic_from(f.end_idx(), vec2(150., 50.), vec2(50., 50.), vec2(50., 150.));
        g.line_from(f.end_idx(), vec2(180., 150.));

        let curr_node = f.end_idx();
        let curr_dir = f.calc_end_tangent(&g).unwrap();
        let edge = g.get_cw_edge_of_node(curr_node, curr_dir, Some(e0)).unwrap();

        let mut cx = SnapshotCtx::default();

        let edges: Vec<_> = g.edges.keys().cloned().collect();
        draw_edge_list(&mut cx, &g, edges.as_slice()).unwrap();

        cx.with_stroke_color(255, 0, 0, 255);
        draw_edge(&mut cx, &g, edge).unwrap();

        let path = "./tests/images/next_edge__loop_back_cw.1.png".to_string();
        
        let diff = cx.save_or_difference_with_disk(&path);
        assert!(diff < 0.01);
    }

    #[test]
    pub fn loop_back_2() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(20., 100.), vec2(100., 100.));
        g.cubic_from(f.end_idx(), vec2(150., 150.), vec2(50., 150.), vec2(50., 50.));
        g.line_from(f.end_idx(), vec2(180., 50.));

        let curr_node = f.end_idx();
        let curr_dir = f.calc_end_tangent(&g).unwrap();
        let edge = g.get_cw_edge_of_node(curr_node, curr_dir, Some(e0)).unwrap();

        let mut cx = SnapshotCtx::default();

        let edges: Vec<_> = g.edges.keys().cloned().collect();
        draw_edge_list(&mut cx, &g, edges.as_slice()).unwrap();

        cx.with_stroke_color(255, 0, 0, 255);
        draw_edge(&mut cx, &g, edge).unwrap();

        let path = "./tests/images/next_edge__loop_back_cw.2.png".to_string();
        
        let diff = cx.save_or_difference_with_disk(&path);
        assert!(diff < 0.01);
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

            let path = format!("./tests/images/next_edge__ccw.{id}.png");
            
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

            let path = format!("./tests/images/next_edge__rotated_ccw.{id}.png");
            
            let diff = cx.save_or_difference_with_disk(&path);
            diff < 0.01
        })
    }

    #[test]
    pub fn parallel_fuzz() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(20., 100.), vec2(100., 100.));
        g.quadratic_from(f.end_idx(), vec2(150., 100.), vec2(150., 150.));
        g.quadratic_from(f.end_idx(), vec2(150., 100.), vec2(150., 50.));
        g.quadratic_from(f.end_idx(), vec2(50., 100.), vec2(50., 50.));
        g.quadratic_from(f.end_idx(), vec2(50., 100.), vec2(50., 150.));

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

            let path = format!("./tests/images/next_edge__parallel_fuzz_ccw.{id}.png");
            
            let diff = cx.save_or_difference_with_disk(&path);
            diff < 0.01
        });
    }
}
