
mod perform_closed_walk_from_node {
    use bb_vector_network::prelude::*;
    use glam::Vec2;

    #[test]
    fn it_should_ok_simple() -> BBResult<()> {
        let mut g = BBGraph::new();
        let (e0, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (e1, edge) = g.line_from(first_edge.end_idx(), Vec2::new(0., 5.));
        let (e2, edge) = g.line_from(edge.end_idx(), Vec2::new(-5., 5.));
        let (e3, _) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let result = mcb::perform_closed_walk_from_node(&g, first_edge.start_idx())?;

        println!("Result {result:?}");

        Ok(())
    }

    #[test]
    fn it_should_error_with_dead_end_simple() {
        let mut g = BBGraph::new();
        let (e0, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (e1, edge) = g.line_from(first_edge.end_idx(), Vec2::new(0., 5.));
        let (e2, edge) = g.line_from(edge.end_idx(), Vec2::new(-5., 5.));
        // let (e3, _) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let error = mcb::perform_closed_walk_from_node(&g, first_edge.start_idx()).expect_err("Should not OK.");

        match error {
            BBError::ClosedWalkDeadEnd => (),
            error => {
                panic!("Received wrong error. Expected BBError::ClosedWalkDeadEnd, instead found {error:?}");
            }
        }
    }

    #[test]
    fn it_should_error_with_too_small_simple() {
        let mut g = BBGraph::new();
        let (e0, first_edge) = g.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
        let (e1, edge) = g.line_from_to(first_edge.end_idx(), first_edge.start_idx());

        let error = mcb::perform_closed_walk_from_node(&g, first_edge.start_idx()).expect_err("Should not OK.");

        match error {
            BBError::ClosedWalkTooSmall(_) => (),
            error => {
                panic!("Received wrong error. Expected BBError::ClosedWalkTooSmall, instead found {error:?}");
            }
        }
    }

}


mod extract_nested_from_closed_walk {
    use bb_vector_network::prelude::*;
    use glam::Vec2;

    #[test]
    fn it_should_ok_simple() {
        let mut g = BBGraph::new();

        let (_, first_edge) = g.line(Vec2::new(-5., -5.), Vec2::new(-4., 0.));
        let (_, fork_edge) = g.line_from(first_edge.end_idx(), Vec2::new(-0., 0.));
        let (_, edge) = g.line_from(fork_edge.end_idx(), Vec2::new(5., 0.));
        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(5., -5.));
        let (_, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());


        let (_, edge) = g.line_from(fork_edge.end_idx(), Vec2::new(-2., -2.));
        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(2., -2.));
        let (_, edge) = g.line_from_to(edge.end_idx(), fork_edge.end_idx());

        let (outer_edge, closed_walk) =
            mcb::perform_closed_walk_from_node(&g, first_edge.start_idx()).unwrap();

        assert_eq!(closed_walk.len(), 8, "Closed walk length");

        let result = mcb::extract_nested_from_closed_walk(&g, &closed_walk);
        let Ok((parent, nesteds)) = result else {
            panic!("Error Result {result:?}");
        };

        println!("Parent: {parent:?}");
        println!("Nesteds: {nesteds:?}");
    }
}
