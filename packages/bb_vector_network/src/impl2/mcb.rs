use std::ops::Mul;

use crate::{impl2::bb_region::BBCycle, BBEdgeIndex, BBGraph, BBNodeIndex};

use super::{
    bb_region::BBRegion,
    errors::{BBError, BBResult},
};

pub fn mcb(graph: &BBGraph) -> BBResult<Vec<BBRegion>> {
    let mut regions = vec![];

    for mut graph in graph.get_detached_graphs() {
        println!("Handling detached graph");
        let mut cycles = vec![];
        extract_cycles(&mut graph, &mut cycles)?;
        regions.push(BBRegion::new(cycles));
    }

    Ok(regions)
}

fn extract_cycles(graph: &mut BBGraph, cycles_out: &mut Vec<BBCycle>) -> BBResult<()> {
    while graph.node_len() > 0 {
        println!("Handling closed walk");
        graph.remove_filaments();

        let Some(left_most) = graph.get_left_most_anchor_index() else {
            break;
        };

        let (outer_edge, closed_walk) = perform_closed_walk_from_node(graph, left_most)?;
        let (parent_cycle, nested_walks) = extract_nested_from_closed_walk(graph, &closed_walk)?;
        let mut parent_cycle = BBCycle::new(parent_cycle);

        graph.delete_edge(outer_edge); // Needed to cleanup the cycle

        for walk in nested_walks {
            println!("Handling nested closed walk");
            let mut nested_graph = BBGraph::new_from_other_edges(graph, &walk);
            extract_cycles(&mut nested_graph, &mut parent_cycle.children)?;
        }

        cycles_out.push(parent_cycle);
    }

    todo!();
}

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
    let first_link_idx = graph.get_cw_edge_of_node(node_idx, glam::Vec2::new(0., 1.), None)?;
    let mut closed_walk = vec![first_link_idx];

    let first_link = graph.edge(first_link_idx)?.directed_from(node_idx);

    #[cfg(feature = "debug_draw")]
    comfy::draw_arrow(
        graph.node(first_link.start_idx())?.position() + 0.2,
        graph.node(first_link.end_idx())?.position() + 0.2,
        0.08,
        comfy::GRAY,
        100,
    );

    let mut edge_idx_curr = first_link_idx;
    let mut edge_curr = first_link;
    let mut node_curr = first_link.end_idx();
    let mut dir_curr = first_link.calc_end_tangent(graph)?.mul(-1.);

    let mut iterations = 0;

    while iterations < 1000 {
        iterations += 1;

        if iterations >= 3 && edge_curr.contains_node_idx(node_idx) {
            break;
        }

        let edge_idx_next = graph.get_ccw_edge_of_node(node_curr, dir_curr, Some(edge_idx_curr))?;

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
        dir_curr = edge_next.calc_end_tangent(graph)?.mul(-1.);
    }

    let length = closed_walk.len();
    if length >= MIN_EDGES_FOR_CYCLE {
        Ok((first_link_idx, closed_walk))
    } else {
        Err(BBError::ClosedWalkTooSmall(length))
    }
}

pub fn extract_nested_from_closed_walk(
    graph: &BBGraph,
    closed_walk: &ClosedWalk,
) -> BBResult<(ClosedWalk, Vec<ClosedWalk>)> {
    // let mut result = vec![];
    // let cw_real: Vec<_> = closed_walk.iter().map(|edge_idx| graph.edge(*edge_idx)).collect();
    // dbg!(cw_real);
    //
    // let node_first = graph
    //     .edge(*closed_walk.first().unwrap())
    //     .unwrap()
    //     .start_idx();
    // let mut node_prev_idx = node_first;
    // let mut nodes = vec![node_first];
    // for edge_idx in closed_walk.iter() {
    //     let edge = graph.edge(*edge_idx).unwrap();
    //     let node_next = (*edge).other_node_idx(node_prev_idx);
    //     nodes.push(node_next);
    //     node_prev_idx = node_next;
    // }

    let mut closed_walk = graph.edges_from_closed_walk(closed_walk)?;
    let mut nested_closed_walk = vec![];
    for (i, (idx, edge)) in closed_walk.iter().enumerate() {
        println!("{i}: {idx} {edge}");
    }

    let mut next_i = 1;
    while next_i < closed_walk.len() {
        let i = next_i - 1;
        let (edge_idx, edge) = closed_walk[i];

        let mut nested_range = None;

        for (end_i, (other_edge_idx, other_edge)) in closed_walk[next_i..]
            .iter()
            .enumerate()
            .rev()
        {
            println!("{i}:{end_i} - {} vs {}", edge.start_idx(), other_edge.end_idx());

            if edge.start_idx() == other_edge.end_idx() {
                nested_range = Some(i..(end_i + next_i));
                break;
            }
            // let shares_node = if is_first {
            //     other_edge.contains_node_idx(edge.end_idx())
            // } else if is_last {
            //     edge.contains_node_idx(other_edge.start_idx())
            // } else {
            //     other_edge.shares_node_idx(&edge)
            // };
            //
            // if shares_node {
            //     break;
            // }
        }

        if let Some(nested_range) = nested_range {

            let nested_walk: Vec<_> = closed_walk.drain(nested_range).collect();

            println!("Found nested closed_walk:");
            for (i, (idx, edge)) in nested_walk.iter().enumerate() {
                println!("\t{i}: {idx} {edge}");
            }
            println!("Original Walk:");
            for (i, (idx, edge)) in closed_walk.iter().enumerate() {
                println!("\t{i}: {idx} {edge}");
            }

            nested_closed_walk.push(nested_walk);
        }

        next_i += 1;
    }

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
    Err(BBError::ClosedWalkDeadEnd)
}
