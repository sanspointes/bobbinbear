
mod directed_from {
    use bb_vector_network::prelude::*;
    #[test]
    fn unchanged() {
        let edge = BBEdge::Line { start: BBNodeIndex(0), end: BBNodeIndex(1) };

        let result = edge.directed_from(BBNodeIndex(0));
        assert_eq!(edge.start_idx(), result.start_idx());
        assert_eq!(edge.end_idx(), result.end_idx());
    }

    #[test]
    fn needs_flip() {
        let edge = BBEdge::Line { start: BBNodeIndex(0), end: BBNodeIndex(1) };

        let result = edge.directed_from(BBNodeIndex(1));
        assert_eq!(edge.start_idx(), result.end_idx());
        assert_eq!(edge.end_idx(), result.start_idx());
    }
}

mod calc_start_tangent {
    use glam::vec2;
    use bb_vector_network::prelude::*;
    #[test]
    fn line() {
        let mut g = BBGraph::new();
        let (_, e) = g.line(vec2(0., 0.), vec2(1., 0.));

        assert_eq!(e.calc_start_tangent(&g).unwrap(), vec2(1., 0.))
    }

    #[test]
    fn quadratic() {
        let mut g = BBGraph::new();
        let (_, e) = g.quadratic(vec2(0., 0.), vec2(1., 0.), vec2(1., 1.));

        assert_eq!(e.calc_start_tangent(&g).unwrap(), vec2(1., 0.))
    }

    #[test]
    fn cubic() {
        let mut g = BBGraph::new();
        let (_, e) = g.cubic(vec2(0., 0.), vec2(1., 0.), vec2(1., 1.), vec2(0., 1.));

        assert_eq!(e.calc_start_tangent(&g).unwrap(), vec2(1., 0.))
    }
}


mod calc_end_tangent {
    use glam::vec2;
    use bb_vector_network::prelude::*;
    #[test]
    fn line() {
        let mut g = BBGraph::new();
        let (_, e) = g.line(vec2(0., 0.), vec2(1., 0.));

        assert_eq!(e.calc_end_tangent(&g).unwrap(), vec2(1., 0.))
    }

    #[test]
    fn quadratic() {
        let mut g = BBGraph::new();
        let (_, e) = g.quadratic(vec2(0., 0.), vec2(1., 0.), vec2(1., 1.));

        assert_eq!(e.calc_end_tangent(&g).unwrap(), vec2(0., 1.))
    }

    #[test]
    fn cubic() {
        let mut g = BBGraph::new();
        let (_, e) = g.cubic(vec2(0., 0.), vec2(1., 0.), vec2(1., 1.), vec2(0., 1.));

        assert_eq!(e.calc_end_tangent(&g).unwrap(), vec2(-1., 0.))
    }
}
