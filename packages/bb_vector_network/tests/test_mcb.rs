#[allow(unused_variables)]
mod mcb {
    use bb_vector_network::prelude::*;
    use glam::Vec2;

    // #[test]
    fn it_should_pass_smoke_test() {
        let mut g = BBGraph::new();


        // Left Cycle
        let (_, first_edge) = g.line(Vec2::new(-6., 0.), Vec2::new(0., 0.));
        let (_, middle_edge) = g.line_from(first_edge.start_idx(), Vec2::new(0., -5.));
        let (_, edge) = g.line_from(middle_edge.start_idx(), Vec2::new(-5., -5.));
        g.line_from_to(edge.end_idx(), first_edge.start_idx());

        // Right Cycle 
        let (_, edge) = g.line_from(middle_edge.start_idx(), Vec2::new(5., 0.));
        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(5., -5.));
        let (_, edge) = g.line_from_to(edge.end_idx(), middle_edge.end_idx());


        // let (e5, edge) = g.line_from(fork_edge.end_idx(), Vec2::new(-2., 2.));
        // let (e6, edge) = g.line_from(edge.end_idx(), Vec2::new(2., 2.));
        // let (e7, edge) = g.line_from_to(edge.end_idx(), fork_edge.end_idx());

        let result = 
        match mcb::mcb(&g) {
            Ok(result) => {
                println!("{result:?}");
                todo!();
            }
            Err(reason) => {
                panic!("FOUND ERROR: {reason:?}");
            }
        };
    }
}

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

#[allow(unused_variables)]
mod extract_nested_from_closed_walk {
    use bb_vector_network::prelude::*;
    use glam::Vec2;

    #[test]
    /// Contains a simple nested cycle branching directly off fork_edge.end_idx().
    fn it_should_ok_simple() {
        let mut g = BBGraph::new();

        let (e0, first_edge) = g.line(Vec2::new(-5., 5.), Vec2::new(-4., 0.));
        let (e1, fork_edge) = g.line_from(first_edge.end_idx(), Vec2::new(-0., 0.));
        let (e2, edge) = g.line_from(fork_edge.end_idx(), Vec2::new(5., 0.));
        let (e3, edge) = g.line_from(edge.end_idx(), Vec2::new(5., 5.));
        let (e4, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());


        let (e5, edge) = g.line_from(fork_edge.end_idx(), Vec2::new(-2., 2.));
        let (e6, edge) = g.line_from(edge.end_idx(), Vec2::new(2., 2.));
        let (e7, edge) = g.line_from_to(edge.end_idx(), fork_edge.end_idx());

        let (outer_edge, closed_walk) =
            mcb::perform_closed_walk_from_node(&g, first_edge.start_idx()).unwrap();

        assert_eq!(closed_walk.len(), 8, "Closed walk length");

        let result = mcb::extract_nested_from_closed_walk(&g, &closed_walk);
        let Ok((parent, nesteds)) = result else {
            panic!("Error Result {result:?}");
        };

        assert_eq!(parent.len(), 5);
        assert_eq!(parent, vec![e0, e1, e2, e3, e4]);

        assert_eq!(nesteds.len(), 1);
        let nested = nesteds.first().unwrap();
        assert_eq!(nested.len(), 3);
        assert_eq!(nested, &vec![e5, e6, e7]);
    }

    /// Contains a nested cycle, that is connected to the parent cycle via a filament, which connects off fork_edge.end_idx().
    #[test]
    fn it_should_ok_on_nested_with_filament() {
        let mut g = BBGraph::new();

        let (e0, first_edge) = g.line(Vec2::new(-5., 5.), Vec2::new(-4., 0.));
        let (e1, fork_edge) = g.line_from(first_edge.end_idx(), Vec2::new(-0., 0.));
        let (e2, edge) = g.line_from(fork_edge.end_idx(), Vec2::new(5., 0.));
        let (e3, edge) = g.line_from(edge.end_idx(), Vec2::new(5., 5.));
        let (e4, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());


        let (e5, prong_edge) = g.line_from(fork_edge.end_idx(), Vec2::new(0., 1.));
        let (e6, edge) = g.line_from(prong_edge.end_idx(), Vec2::new(-2., 2.));
        let (e7, edge) = g.line_from(edge.end_idx(), Vec2::new(2., 2.));
        let (e8, edge) = g.line_from_to(edge.end_idx(), prong_edge.end_idx());

        let (outer_edge, closed_walk) =
            mcb::perform_closed_walk_from_node(&g, first_edge.start_idx()).unwrap();

        assert_eq!(closed_walk, vec![e0, e1, e5, e6, e7, e8, e5, e2, e3, e4]);

        let result = mcb::extract_nested_from_closed_walk(&g, &closed_walk);
        let Ok((parent, nesteds)) = result else {
            panic!("Error Result {result:?}");
        };

        assert_eq!(parent, vec![e0, e1, e2, e3, e4]);

        assert_eq!(nesteds.len(), 1);
        let nested = nesteds.first().unwrap();
        assert_eq!(nested, &vec![e5, e6, e7, e8, e5]);
    }

    #[test]
    fn it_should_ok_when_branch_is_on_first_node() {
        let mut g = BBGraph::new();

        let (e0, first_edge) = g.line(Vec2::new(-5., 5.), Vec2::new(-4., 0.));
        let (e1, edge) = g.line_from(first_edge.end_idx(), Vec2::new(-0., 0.));
        let (e2, edge) = g.line_from(edge.end_idx(), Vec2::new(5., 0.));
        let (e3, edge) = g.line_from(edge.end_idx(), Vec2::new(5., 5.));
        let (e4, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());


        let (e5, edge) = g.line_from(first_edge.start_idx(), Vec2::new(-3., 4.));
        let (e6, edge) = g.line_from(edge.end_idx(), Vec2::new(-3., 2.));
        let (e7, edge) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let (outer_edge, closed_walk) =
            mcb::perform_closed_walk_from_node(&g, first_edge.start_idx()).unwrap();

        assert_eq!(closed_walk, vec![e0, e1, e2, e3, e4, e5, e6, e7]);

        let result = mcb::extract_nested_from_closed_walk(&g, &closed_walk);
        let Ok((parent, nesteds)) = result else {
            panic!("Error Result {result:?}");
        };

        assert_eq!(parent.len(), 5);
        assert_eq!(parent, vec![e0, e1, e2, e3, e4]);

        assert_eq!(nesteds.len(), 1);
        let nested = nesteds.first().unwrap();
        assert_eq!(nested.len(), 3);
        assert_eq!(nested, &vec![e5, e6, e7]);
    }
}
