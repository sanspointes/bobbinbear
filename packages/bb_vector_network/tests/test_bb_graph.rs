#[allow(unused_variables)]

mod common;

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
        let result = g.edges_directed(&closed_walk).unwrap();
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

        let result = g.edges_directed(&closed_walk).unwrap();
        assert_closed_walk_directionality(&g, result);
    }
}

mod new_from_other_edges {
    use bb_vector_network::prelude::*;
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

        assert_eq!(extracted.nodes_count(), 3);
        for edge in extracted.edges.values() {
            extracted
                .node(edge.start_idx())
                .expect(&format!("Start idx not in graph on {edge}."));
            extracted
                .node(edge.end_idx())
                .expect(&format!("Start idx not in graph on {edge}."));
        }
    }
}

mod remove_filaments {
    use bb_vector_network::prelude::*;
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

        assert_eq!(g.nodes_count(), 5);

        let _ = g.remove_filaments();
        assert_eq!(g.nodes_count(), 4);
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

        assert_eq!(g.nodes_count(), 7);
        let _ = g.remove_filaments();
        assert_eq!(g.nodes_count(), 4);
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

        assert_eq!(g.nodes_count(), 3);
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

        assert_eq!(g.nodes_count(), 2);
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

        assert_eq!(g.nodes_count(), 2);
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

        assert_eq!(g.nodes_count(), 2);
        let remaining_edge = g.edge(e1).unwrap();
        assert_eq!(remaining_edge.start_pos(&g), Vec2::new(0., 0.));
        assert_eq!(remaining_edge.end_pos(&g), Vec2::new(-2.5, 2.5));
    }
}

mod get_detached_graphs {
    use std::collections::HashSet;
    use glam::Vec2;

    use bb_vector_network::prelude::*;

    #[test]
    fn it_should_return_single_if_no_detached() {
        let mut g = BBGraph::new();
        let (_, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(-2.5, 2.5));
        let (_, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let mut graphs = g.get_detached_graphs().unwrap();

        assert_eq!(graphs.len(), 1);
        let first = graphs.pop().unwrap();
        assert_eq!(first.nodes_count(), 3);
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
        assert_eq!(first.nodes_count(), 3);
        let second = graphs.pop().unwrap();
        assert_eq!(second.nodes_count(), 3);
    }

    #[test]
    fn it_should_not_duplicate_nodes() {
        let mut g = BBGraph::new();
        let (_, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(-2.5, 2.5));
        let (_, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let (_, first_edge) = g.line(Vec2::new(2., 0.), Vec2::new(7., 0.));
        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(5.5, 2.5));
        let (_, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let graphs = g.get_detached_graphs().unwrap();

        for g in graphs {
            for (node_idx, node) in g.nodes {
                let mut hs: HashSet<BBEdgeIndex> = HashSet::new();
                for adj in node.adjacents() {
                    hs.insert(*adj);
                }
                assert_eq!(
                    node.adjacents().len(),
                    hs.len(),
                    "{node_idx} has incorrect nodes after extraction."
                );
            }
        }
    }
}

mod get_cw_edge_of_node {
    use bb_vector_network::prelude::*;
    use glam::vec2;

    use crate::common::draw::{draw_edge, draw_edge_list, SnapshotCtx};

    #[test]
    fn test_simple() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
        let (e1, e) = g.line_from(f.end_idx(), vec2(5., 0.));
        let (e2, e) = g.line_from(f.end_idx(), vec2(5., 5.));
        let (e3, e) = g.line_from(f.end_idx(), vec2(5., -5.));

        let edge = g.get_cw_edge_of_node(f.end_idx(), f.calc_end_tangent(&g).unwrap(), Some(e0)).unwrap();

        let mut ctx = SnapshotCtx::default();
        draw_edge_list(&mut ctx, &g, &[e0, e1, e2, e3]).unwrap();
        let difference = ctx.save_or_difference_with_disk("./tests/images/test_simple_sharp_angle.png");
        assert!(difference < 0.01);
    }

    #[test]
    fn test_simple_sharp_angle() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
        let (e1, e) = g.line_from(f.end_idx(), vec2(-5., 0.));
        let (e2, e) = g.line_from(f.end_idx(), vec2(5., 5.));
        let (e3, e) = g.line_from(f.end_idx(), vec2(-5., -5.));

        let edge = g.get_cw_edge_of_node(f.end_idx(), f.calc_end_tangent(&g).unwrap(), Some(e0)).unwrap();

        let mut ctx = SnapshotCtx::default();
        draw_edge_list(&mut ctx, &g, &[e0, e1, e2, e3]).unwrap();
        let difference = ctx.save_or_difference_with_disk("./tests/images/test_simple_sharp_angle.png");
        assert!(difference < 0.01);
    }
}

mod get_ccw_edge_of_node {
    use bb_vector_network::prelude::*;
    use glam::vec2;

    #[test]
    fn test_simple() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
        let (e1, _) = g.line_from(f.end_idx(), vec2(5., 0.));
        let (e2, _) = g.line_from(f.end_idx(), vec2(5., 5.));
        let (e3, _) = g.line_from(f.end_idx(), vec2(5., -5.));

        let edge = g.get_ccw_edge_of_node(f.end_idx(), f.calc_end_tangent(&g).unwrap(), Some(e0)).unwrap();

        println!("{}", g);

        assert_eq!(edge, e1);
    }

    #[test]
    fn test_simple_sharp_angle() {
        let mut g = BBGraph::new();
        let (e0, f) = g.line(vec2(-5., 0.), vec2(0., 0.));
        let (e1, _) = g.line_from(f.end_idx(), vec2(-5., 0.));
        let (e2, _) = g.line_from(f.end_idx(), vec2(5., 5.));
        let (e3, _) = g.line_from(f.end_idx(), vec2(-5., -5.));

        let edge = g.get_ccw_edge_of_node(f.end_idx(), f.calc_end_tangent(&g).unwrap(), Some(e0)).unwrap();

        println!("{}", g);

        assert_eq!(edge, e1);
    }
}

mod closed_walk_with_cw_start_and_ccw_traverse {
    use bb_vector_network::prelude::*;
    use glam::vec2;

    #[test]
    fn it_should_generate_closed_walk_on_simple_cycle() {
        let mut g = BBGraph::new();

        let (e0, first_edge) = g.line(vec2(0., 0.), vec2(1., 0.));
        let (e1, middle_edge) = g.line_from(first_edge.end_idx(), vec2(1., 1.));
        let (e2, e) = g.line_from(middle_edge.end_idx(), vec2(0., 1.));
        let (outer_edge, _) = g.line_from_to(e.end_idx(), first_edge.start_idx());

        let (_, e) = g.line_from(middle_edge.start_idx(), vec2(2., 0.));
        let (_, e) = g.line_from(e.end_idx(), vec2(2., 1.));
        g.line_from_to(e.end_idx(), middle_edge.end_idx());

        let node_idx = g.get_left_most_node_index().unwrap();
        let v = g.closed_walk_with_cw_start_and_ccw_traverse(node_idx).unwrap();

        assert_eq!(v.outer_edge, outer_edge);
        assert_eq!(v.edges.len(), 4);
        assert_eq!(v.edges, vec![outer_edge, e0, e1, e2]);
    }
}

mod closed_walk_with_ccw_start_and_ccw_traverse {
    use bb_vector_network::prelude::*;
    use glam::vec2;

    #[test]
    fn it_should_walk_perimiter_on_simple_cycle() {
        let mut g = BBGraph::new();

        let (e2, first_edge) = g.line(vec2(0., 0.), vec2(1., 0.));
        let (e4, middle_edge) = g.line_from(first_edge.end_idx(), vec2(1., 1.));
        let (e6, e) = g.line_from(middle_edge.end_idx(), vec2(0., 1.));
        let (e7, _) = g.line_from_to(e.end_idx(), first_edge.start_idx());

        let (e9, e) = g.line_from(middle_edge.start_idx(), vec2(2., 0.));
        let (e11, e) = g.line_from(e.end_idx(), vec2(2., 1.));
        let (e12, _) = g.line_from_to(e.end_idx(), middle_edge.end_idx());

        let node_idx = g.get_left_most_node_index().unwrap();
        let v = g.closed_walk_with_ccw_start_and_ccw_traverse(node_idx).unwrap();

        assert_eq!(v.edges.len(), 6);
        assert_eq!(v.edges, vec![e6, e12, e11, e9, e2, e7]);
    }
}
