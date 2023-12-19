use bb_vector_network::prelude::*;
use glam::Vec2;

fn assert_closed_walk_directionality(g: &BBGraph, result: Vec<(BBEdgeIndex, BBEdge)>) {
    for result in result.windows(2) {
        let (_, prev_edge) = result[0];
        let (_, edge) = result[1];
        println!("({prev_edge})->({edge})");
        assert_eq!(prev_edge.end_idx(), edge.start_idx());
    }
}

#[test]
fn edges_from_closed_walk_directionality_unchanged() {
    // Create a bbgraph with the directionality all messed up.
    let mut g = BBGraph::new();
    let (e0, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
    let (e1, edge) = g.line_from(first_edge.end_idx(), Vec2::new(0., 5.));
    let (e2, edge) = g.line_from(edge.start_idx(), Vec2::new(-5., 5.));
    let (e3, _) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

    let closed_walk = vec![e0, e1, e2, e3];
    let result = g.edges_from_closed_walk(&closed_walk).unwrap();
    assert_closed_walk_directionality(&g, result);
}

#[test]
fn edges_from_closed_walk_directionality_simple() {
    // Create a bbgraph with the directionality all messed up.
    let mut g = BBGraph::new();
    let (e0, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
    let (e1, edge) = g.line_to(Vec2::new(0., 5.), first_edge.end_idx());
    let (e2, edge) = g.line_from(edge.start_idx(), Vec2::new(-5., 5.));
    let (e3, _) = g.line_from_to(first_edge.start_idx(), edge.end_idx());

    let closed_walk = vec![e0, e1, e2, e3];

    let result = g.edges_from_closed_walk(&closed_walk).unwrap();
    assert_closed_walk_directionality(&g, result);
}
