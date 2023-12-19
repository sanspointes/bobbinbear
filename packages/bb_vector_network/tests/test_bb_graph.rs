#[allow(unused_variables)]

mod edges_from_closed_walk {
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
    fn it_should_keep_directionality_if_already_directed() {
        // Create a bbgraph with the directionality all messed up.
        let mut g = BBGraph::new();
        let (e0, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (e1, edge) = g.line_from(first_edge.end_idx(), Vec2::new(0., 5.));
        let (e2, edge) = g.line_from(edge.end_idx(), Vec2::new(-5., 5.));
        let (e3, _) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let closed_walk = vec![e0, e1, e2, e3];
        let result = g.edges_from_closed_walk(&closed_walk).unwrap();
        assert_closed_walk_directionality(&g, result);
    }

    #[test]
    fn it_should_fix_directionality_if_some_edges_reversed() {
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
}

mod get_detached_graphs {
    use bb_vector_network::prelude::*;
    use glam::Vec2;

    #[test]
    fn it_should_return_single_if_no_detached() {
        let mut g = BBGraph::new();
        let (_, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(-2.5, 2.5));
        let (_, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let mut graphs = g.get_detached_graphs().unwrap();

        assert_eq!(graphs.len(), 1);
        let first = graphs.pop().unwrap();
        assert_eq!(first.node_len(), 3);
    }

    #[test]
    fn it_should_return_two_if_there_is_detached_graph() {
        let mut g = BBGraph::new();
        let (_, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(-2.5, 2.5));
        let (_, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let (_, first_edge) = g.line(Vec2::new(2., 0.), Vec2::new(7., 0.));
        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(5.5, 2.5));
        let (_, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let mut graphs = g.get_detached_graphs().unwrap();

        assert_eq!(graphs.len(), 2);
        let first = graphs.pop().unwrap();
        assert_eq!(first.node_len(), 3);
        let second = graphs.pop().unwrap();
        assert_eq!(first.node_len(), 3);
    }
}
