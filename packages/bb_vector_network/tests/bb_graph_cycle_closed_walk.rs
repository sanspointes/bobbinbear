mod common;

use bb_vector_network::{bb_edge::BBEdgeIndex, bb_graph::{BBGraph, ClosedWalkResult}};
use glam::vec2;

use crate::common::{draw::{draw_all_edges, draw_edge, SnapshotCtx}, fuzz_snapshot::FuzzSnapshot};

fn build_line_graph() -> (BBGraph, [BBEdgeIndex; 7]) {
    let mut g = BBGraph::new();

    let (e0, f) = g.line(vec2(20., 20.), vec2(100., 20.));
    // Right section
    let (e1, p) = g.line_from(f.end_idx(), vec2(180., 20.));
    let (e2, p) = g.line_from(p.end_idx(), vec2(180., 180.));
    let (e3, c) = g.line_from(p.end_idx(), vec2(100., 180.));
    // middle edge
    let (e4, _) = g.line_from_to(c.end_idx(), f.end_idx());
    // Left section
    let (e5, p) = g.line_from(c.end_idx(), vec2(20., 180.));
    let (e6, _) = g.line_from_to(p.end_idx(), f.start_idx());

    (g, [e0, e1, e2, e3, e4, e5, e6])
}

#[test]
pub fn perimiter_line_basic() {
    let (g, _edges) = build_line_graph();

    let fuzz = FuzzSnapshot::new(g);
    fuzz.run_reversed_edges(|key, mut g| {
        let mut cx = SnapshotCtx::default();

        draw_all_edges(&mut cx, &g).unwrap();

        g.remove_filaments().unwrap();

        let left_most = g.get_left_most_node_index().unwrap();
        let ClosedWalkResult { outer_edge, edges } = g.closed_walk_with_ccw_start_and_ccw_traverse(left_most).unwrap();

        cx.with_stroke_color(255, 0, 0, 255);
        for id in &edges {
            draw_edge(&mut cx, &g, *id).unwrap();
        }

        cx.with_stroke_color(255, 0, 0, 255);
        draw_edge(&mut cx, &g, outer_edge).unwrap();

        let path = "./tests/images/closed_walk__perimiter_line_basic.png".to_string();

        cx.compare_threshold_disk(&path, 0.01, &key)

    })
}

#[test]
pub fn perimiter_line_fuzz() {
    let (g, _edges) = build_line_graph();

    let fuzz = FuzzSnapshot::new(g);
    fuzz.run_with_randomized_positions(10, 20., |key, mut g| {
        let mut cx = SnapshotCtx::default();

        draw_all_edges(&mut cx, &g).unwrap();

        g.remove_filaments().unwrap();

        let left_most = g.get_left_most_node_index().unwrap();
        let ClosedWalkResult { outer_edge, edges } = g.closed_walk_with_ccw_start_and_ccw_traverse(left_most).unwrap();

        cx.with_stroke_color(255, 0, 0, 255);
        for id in &edges {
            draw_edge(&mut cx, &g, *id).unwrap();
        }

        cx.with_stroke_color(255, 0, 0, 255);
        draw_edge(&mut cx, &g, outer_edge).unwrap();

        let path = format!("./tests/images/closed_walk__perimiter_line_fuzz.{key}.png");

        cx.compare_threshold_disk(&path, 0.01, &key)

    })
}

#[test]
pub fn mcb_line_basic() {
    let (g, _edges) = build_line_graph();

    let fuzz = FuzzSnapshot::new(g);
    fuzz.run_reversed_edges(|key, mut g| {
        let mut cx = SnapshotCtx::default();

        draw_all_edges(&mut cx, &g).unwrap();

        g.remove_filaments().unwrap();

        let left_most = g.get_left_most_node_index().unwrap();
        let ClosedWalkResult { outer_edge, edges } = g.closed_walk_with_cw_start_and_ccw_traverse(left_most).unwrap();

        cx.with_stroke_color(255, 0, 0, 255);
        for id in &edges {
            draw_edge(&mut cx, &g, *id).unwrap();
        }

        cx.with_stroke_color(255, 0, 0, 255);
        draw_edge(&mut cx, &g, outer_edge).unwrap();

        let path = "./tests/images/closed_walk__mcb_line_basic.png".to_string();

        cx.compare_threshold_disk(&path, 0.01, &key)

    })
}

#[test]
pub fn mcb_line_fuzz() {
    let (g, _edges) = build_line_graph();

    let fuzz = FuzzSnapshot::new(g);
    fuzz.run_with_randomized_positions(10, 20., |key, mut g| {
        let mut cx = SnapshotCtx::default();

        draw_all_edges(&mut cx, &g).unwrap();

        g.remove_filaments().unwrap();

        let left_most = g.get_left_most_node_index().unwrap();
        let ClosedWalkResult { outer_edge, edges } = g.closed_walk_with_cw_start_and_ccw_traverse(left_most).unwrap();

        cx.with_stroke_color(255, 0, 0, 255);
        for id in &edges {
            draw_edge(&mut cx, &g, *id).unwrap();
        }

        cx.with_stroke_color(255, 0, 0, 255);
        draw_edge(&mut cx, &g, outer_edge).unwrap();

        let path = format!("./tests/images/closed_walk__mcb_line_fuzz.{key}.png");

        cx.compare_threshold_disk(&path, 0.01, &key)

    })
}

pub fn build_curve_graph_1() -> BBGraph {
    let mut g = BBGraph::new();

    let (_, f) = g.cubic(vec2(50., 50.), vec2(70., 30.), vec2(80., 30.), vec2(100., 50.));
    let (_, p) = g.cubic_from(f.end_idx(), vec2(125., 70.), vec2(130., 70.), vec2(150., 50.));
    let (_, p) = g.cubic_from(p.end_idx(), vec2(160., 30.), vec2(160., 100.), vec2(140., 150.));
    let (_, p) = g.cubic_from_to(p.end_idx(), vec2(120., 150.), vec2(100., 180.), f.end_idx());

    let (_, p) = g.cubic_from(p.start_idx(), vec2(120., 180.), vec2(80., 180.), vec2(80., 120.));
    g.cubic_from_to(p.end_idx(), vec2(40., 100.), vec2(80., 60.), f.start_idx());

    g
}

#[test]
pub fn perimiter_curve_1() {
    let g = build_curve_graph_1();

    let fuzz = FuzzSnapshot::new(g);
    fuzz.run_reversed_edges(|key, mut g| {
        let mut cx = SnapshotCtx::default();

        draw_all_edges(&mut cx, &g).unwrap();

        g.expand_intersections().unwrap();

        g.remove_filaments().unwrap();

        let left_most = g.get_left_most_node_index().unwrap();
        let ClosedWalkResult { outer_edge, edges } = g.closed_walk_with_cw_start_and_ccw_traverse(left_most).unwrap();

        cx.with_stroke_color(255, 0, 0, 255);
        for id in &edges {
            draw_edge(&mut cx, &g, *id).unwrap();
        }

        cx.with_stroke_color(255, 0, 0, 255);
        draw_edge(&mut cx, &g, outer_edge).unwrap();

        let path = "./tests/images/closed_walk__perimiter_curve_1.png".to_string();

        cx.compare_threshold_disk(&path, 0.01, &key)

    })
}

#[test]
pub fn perimiter_curve_1_fuzz() {
    let g = build_curve_graph_1();

    let fuzz = FuzzSnapshot::new(g);
    fuzz.run_with_randomized_positions(5, 60., |key, mut g| {
        let mut cx = SnapshotCtx::default();

        draw_all_edges(&mut cx, &g).unwrap();

        g.expand_intersections().unwrap();

        g.remove_filaments().unwrap();

        let left_most = g.get_left_most_node_index().unwrap();
        let ClosedWalkResult { outer_edge, edges } = g.closed_walk_with_cw_start_and_ccw_traverse(left_most).unwrap();

        cx.with_stroke_color(255, 0, 0, 255);
        for id in &edges {
            draw_edge(&mut cx, &g, *id).unwrap();
        }

        cx.with_stroke_color(255, 0, 0, 255);
        draw_edge(&mut cx, &g, outer_edge).unwrap();

        let path = format!("./tests/images/closed_walk__perimiter_curve_1_fuzz.{key}.png");

        cx.compare_threshold_disk(&path, 0.01, &key)

    })
}
