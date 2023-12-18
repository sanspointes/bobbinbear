use crate::{BBEdgeIndex, BBGraph, BBNodeIndex};

use super::errors::{BBError, BBResult};

pub fn mcb(graph: &BBGraph) {}

pub type ClosedWalk = Vec<BBEdgeIndex>;
pub const MIN_EDGES_FOR_CYCLE: usize = 3;

/// Attempts to generate a closed walk from a given node around a graph.
///
/// * `graph`: Graph to traverse.
/// * `node_idx`: Starting node
pub fn perform_closed_walk_from_node(
    graph: &BBGraph,
    node_idx: BBNodeIndex,
) -> BBResult<(BBEdgeIndex, ClosedWalk)> {
    let first_link_idx = graph.get_ccw_edge_of_node(node_idx, glam::Vec2::new(0., -1.), None)?;
    let mut closed_walk = vec![first_link_idx];

    let first_link = graph.edge(first_link_idx)?.directed_from(node_idx);

    #[cfg(feature = "debug_draw")]
    comfy::draw_arrow(
        graph.node(first_link.start_idx())?.position() + 0.2,
        graph.node(first_link.end_idx())?.position() + 0.2,
        0.05,
        comfy::GRAY,
        100,
    );

    let mut edge_idx_curr = first_link_idx;
    let mut edge_curr = first_link;
    let mut node_curr = first_link.end_idx();
    let mut dir_curr = first_link.calc_end_tangent(graph)?;

    let mut iterations = 0;

    while iterations < 1000 {
        iterations += 1;

        if iterations >= 3 && edge_curr.references_idx(node_idx) {
            println!("Re-reached start, breaking");
            break;
        }

        let edge_idx_next = graph.get_ccw_edge_of_node(node_curr, dir_curr, Some(edge_idx_curr))?;
        println!("{iterations:?}: {edge_idx_curr:?} -> {edge_idx_next:?}");

        edge_idx_curr = edge_idx_next;
        closed_walk.push(edge_idx_next);

        let edge_next = graph.edge(edge_idx_curr)?.directed_from(node_curr);

        #[cfg(feature = "debug_draw")]
        comfy::draw_arrow(
            graph.node(edge_next.start_idx())?.position() + 0.2,
            graph.node(edge_next.end_idx())?.position() + 0.2,
            0.05,
            comfy::GRAY,
            100,
        );

        edge_curr = edge_next;
        node_curr = edge_next.other_node_idx(node_curr);
        dir_curr = edge_next.calc_end_tangent(graph)?;
    }

    let length = closed_walk.len();
    if length >= MIN_EDGES_FOR_CYCLE {
        Ok((first_link_idx, closed_walk))
    } else {
        Err(BBError::ClosedWalkTooSmall(length))
    }
}

pub fn extract_nested_from_closed_walk(graph: &BBGraph, closed_walk: &ClosedWalk) -> Vec<BBGraph> {
    // let mut result = vec![];

    let node_first = graph
        .edge(*closed_walk.first().unwrap())
        .unwrap()
        .start_idx();
    let mut node_prev_idx = node_first;
    let mut nodes = vec![node_first];
    for edge_idx in closed_walk.iter() {
        let edge = graph.edge(*edge_idx).unwrap();
        let node_next = (*edge).other_node_idx(node_prev_idx);
        nodes.push(node_next);
        node_prev_idx = node_next;
    }

    dbg!(nodes);

    // let mut next_i = 1;
    // while next_i < closed_walk.len() {
    //     let i = next_i - 1;
    //     let edge_idx = closed_walk[i];
    //     let maybe_nested_end = closed_walk[next_i..].iter().position(|el| *el == edge_idx);
    //     if let Some(end_i) = maybe_nested_end {
    //
    //         result.push(BBGraph::new_from_other_edges())
    //     }
    // }
    //
    // result
    panic!("todo")
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::{impl2::mcb::extract_nested_from_closed_walk, BBGraph};

    use super::perform_closed_walk_from_node;

    #[test]
    fn test_basic_closed_walk() {
        let mut g = BBGraph::new();
        let (first_edge_idx, first_edge) = g.line(Vec2::ZERO, Vec2::new(5., 0.));
        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(5., 5.));
        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(0., 5.));
        let (last_edge_idx, _) = g.line_from_to(edge.end_idx(), first_edge.start_idx());

        let (outer_edge, closed_walk) =
            perform_closed_walk_from_node(&g, first_edge.start_idx()).unwrap();

        assert!(closed_walk.len() == 4);
        assert_eq!(outer_edge, last_edge_idx);
    }

    #[test]
    fn test_basic_extract_nested() {
        let mut g = BBGraph::new();
        let (first_edge_idx, first_edge) = g.line(Vec2::ZERO, Vec2::new(5., 0.));
        let (_, edge) = g.line_from(first_edge.end_idx(), Vec2::new(5., 5.));
        let (branch_edge_idx, branch_edge) = g.line_from(edge.end_idx(), Vec2::new(0., 5.));
        // Create the inner nested
        let (_, edge) = g.line_from(branch_edge.end_idx(), Vec2::new(1., 3.));
        let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(2., 4.));
        g.line_from_to(edge.end_idx(), branch_edge.end_idx());

        let (last_edge_idx, _) = g.line_from_to(branch_edge.end_idx(), first_edge.start_idx());

        let (outer_edge, closed_walk) =
            perform_closed_walk_from_node(&g, first_edge.start_idx()).unwrap();

        assert_eq!(closed_walk.len(), 7, "Closed walk length");
        assert_eq!(outer_edge, last_edge_idx, "Outer edge");

        extract_nested_from_closed_walk(&g, &closed_walk);
    }
}
