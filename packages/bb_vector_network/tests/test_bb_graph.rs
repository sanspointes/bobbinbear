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

mod new_from_other_edges {
    use bb_vector_network::BBGraph;
    use glam::Vec2;
    #[test]
    fn it_should_correct_indices_to_reference_copied_nodes() {
        let mut g = BBGraph::new();

        let (e0, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));

        // Missdirect, we aren't copying these to the new position.
        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(2., 0.));
        let (_, _) = g.line_from(edge.end_idx(), Vec2::new(4., 0.));

        let (e3, edge) = g.line_from(first_edge.end_idx(), Vec2::new(-2.5, 2.5));
        let (e4, _) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let extracted = BBGraph::try_new_from_other_edges(&g, &vec![e0, e3, e4]).unwrap();

        assert_eq!(extracted.node_len(), 3);
        for edge in extracted.edges.values() {
            extracted.node(edge.start_idx()).expect(&format!("Start idx not in graph on {edge}."));
            extracted.node(edge.end_idx()).expect(&format!("Start idx not in graph on {edge}."));
        }
    }
}

mod remove_filaments {
    use bb_vector_network::BBGraph;
    use glam::Vec2;
    #[test]
    fn it_should_remove_single_filament() {
        let mut g = BBGraph::new();

        // Box
        let (_, first_edge) = g.line(Vec2::new(-6., 0.), Vec2::new(0., 0.));
        let (_, middle_edge) = g.line_from(first_edge.end_idx(), Vec2::new(0., -5.));
        let (_, edge) = g.line_from(middle_edge.end_idx(), Vec2::new(-5., -5.));
        g.line_from_to(edge.end_idx(), first_edge.start_idx());

        g.line_from(middle_edge.end_idx(), Vec2::new(5., -5.));

        assert_eq!(g.node_len(), 5);

        let _ = g.remove_filaments();
        assert_eq!(g.node_len(), 4);
    }

    #[test]
    fn it_should_remove_filament_chain() {
        let mut g = BBGraph::new();

        // Box
        let (_, first_edge) = g.line(Vec2::new(-6., 0.), Vec2::new(0., 0.));
        let (_, middle_edge) = g.line_from(first_edge.end_idx(), Vec2::new(0., -5.));
        let (_, edge) = g.line_from(middle_edge.end_idx(), Vec2::new(-5., -5.));
        g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let (_, edge) = g.line_from(middle_edge.end_idx(), Vec2::new(5., -5.));
        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(10., -5.));
        g.line_from(edge.end_idx(), Vec2::new(15., -5.));

        assert_eq!(g.node_len(), 7);
        let _ = g.remove_filaments();
        assert_eq!(g.node_len(), 4);
    }
}

mod delete_edge {
    use bb_vector_network::prelude::*;
    use glam::Vec2;

    #[test]
    fn it_should_delete_adjacents() {
        let mut g = BBGraph::new();
        let (e0, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (e1, edge) = g.line_from(first_edge.end_idx(), Vec2::new(-2.5, 2.5));
        let (e2, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let top_node_idx = edge.start_idx();

        g.delete_edge(e0).unwrap();

        assert_eq!(g.node_len(), 3);
        assert_eq!(g.edge(e0).is_err(), true);
        assert_eq!(g.edge(e1).is_ok(), true);
        assert_eq!(g.edge(e2).is_ok(), true);
    }

    #[test]
    fn it_should_maintain_indices() {
        let mut g = BBGraph::new();
        let (e0, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (e1, edge) = g.line_from(first_edge.end_idx(), Vec2::new(-2.5, 2.5));
        let (e2, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let left_node_idx = edge.end_idx();

        g.delete_node(left_node_idx).unwrap();

        assert_eq!(g.node_len(), 2);
        let remaining_edge = g.edge(e1).unwrap();
        assert_eq!(remaining_edge.start_pos(&g), Vec2::new(0., 0.));
        assert_eq!(remaining_edge.end_pos(&g), Vec2::new(-2.5, 2.5));
    }
}

mod delete_node {
    use bb_vector_network::prelude::*;
    use glam::Vec2;

    #[test]
    fn it_should_delete_adjacents() {
        let mut g = BBGraph::new();
        let (e0, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (e1, edge) = g.line_from(first_edge.end_idx(), Vec2::new(-2.5, 2.5));
        let (e2, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let top_node_idx = edge.start_idx();

        g.delete_node(top_node_idx).unwrap();

        assert_eq!(g.node_len(), 2);
        assert_eq!(g.edge(e0).is_ok(), true);
        assert_eq!(g.edge(e1).is_err(), true);
        assert_eq!(g.edge(e2).is_err(), true);
    }
    #[test]
    fn it_should_delete_adjacents_line() {
        let mut g = BBGraph::new();
        let (e0, first_edge) = g.line(Vec2::new(-4., 0.), Vec2::new(-2., 0.));
        let (e1, edge) = g.line_from(first_edge.end_idx(), Vec2::new(0., 0.));
        let (e2, edge) = g.line_from(edge.end_idx(), Vec2::new(2., 0.));
        let (e3, edge) = g.line_from(edge.end_idx(), Vec2::new(4., 0.));

        assert_eq!(first_edge.start(&g).adjacents().len(), 1);
        assert_eq!(first_edge.end(&g).adjacents().len(), 2);
        assert_eq!(edge.start(&g).adjacents().len(), 2);
        assert_eq!(edge.end(&g).adjacents().len(), 1);

        let right_most = edge.end_idx();
        g.delete_node(right_most).unwrap();

        assert_eq!(edge.start(&g).adjacents().len(), 1);
    }

    #[test]
    fn it_should_maintain_indices() {
        let mut g = BBGraph::new();
        let (e0, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (e1, edge) = g.line_from(first_edge.end_idx(), Vec2::new(-2.5, 2.5));
        let (e2, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let left_node_idx = edge.end_idx();

        g.delete_node(left_node_idx);

        assert_eq!(g.node_len(), 2);
        let remaining_edge = g.edge(e1).unwrap();
        assert_eq!(remaining_edge.start_pos(&g), Vec2::new(0., 0.));
        assert_eq!(remaining_edge.end_pos(&g), Vec2::new(-2.5, 2.5));
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
        assert_eq!(second.node_len(), 3);
    }
}
