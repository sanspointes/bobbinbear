use std::ops::Mul;

use crate::{impl2::bb_region::BBCycle, BBEdgeIndex, BBGraph, BBNodeIndex};

use super::{
    bb_region::BBRegion,
    errors::{BBError, BBResult},
};

pub fn mcb(graph: &BBGraph) -> BBResult<Vec<BBRegion>> {
    let mut regions = vec![];

    for mut graph in graph.get_detached_graphs()? {
        println!("Handling detached graph");
        let mut cycles = vec![];
        extract_cycles(&mut graph, &mut cycles)?;
        regions.push(BBRegion::new(cycles));
    }

    Ok(regions)
}

fn extract_cycles(graph: &mut BBGraph, cycles_out: &mut Vec<BBCycle>) -> BBResult<()> {
    println!("START extract_cycles");
    while graph.node_len() > 0 {
        println!(
            "Trying to extract with {} remaining nodes.",
            graph.node_len()
        );
        graph.remove_filaments()?;
        println!("Filaments removed, {} remaining nodes.", graph.node_len());

        if graph.nodes.len() <= 2 || graph.edges.len() <= 2 {
            break;
        }

        let Some(left_most) = graph.get_left_most_anchor_index() else {
            break;
        };
        println!("Left most anchor: {left_most}.");

        let (outer_edge, closed_walk) = perform_closed_walk_from_node(graph, left_most)?;
        println!("Performed closed walk {closed_walk:?}.");
        let (parent_cycle, nested_walks) = extract_nested_from_closed_walk(graph, &closed_walk)?;

        let mut parent_cycle = BBCycle::new(parent_cycle);

        graph.delete_edge(outer_edge)?; // Needed to cleanup the cycle

        for walk in nested_walks {
            println!("Handling nested closed walk on {walk:?}");
            let mut nested_graph = BBGraph::try_new_from_other_edges(graph, &walk)?;
            let result = extract_cycles(&mut nested_graph, &mut parent_cycle.children);

            match result {
                Ok(_) => (),
                Err(reason) => match reason {
                    critical @ BBError::MissingNode(_) | critical @ BBError::MissingEdge(_) => {
                        return Err(critical)
                    }
                    _ => continue,
                },
            }
        }

        cycles_out.push(parent_cycle);
    }

    println!("END extract_cycles");
    Ok(())
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
    let first_edge_idx = graph.get_cw_edge_of_node(node_idx, glam::Vec2::new(0., 1.), None)?;
    let mut closed_walk = vec![first_edge_idx];

    let first_edge = graph.edge(first_edge_idx)?.directed_from(node_idx);

    #[cfg(feature = "debug_draw")]
    comfy::draw_arrow(
        graph.node(first_edge.start_idx())?.position() + 0.2,
        graph.node(first_edge.end_idx())?.position() + 0.2,
        0.08,
        comfy::GRAY,
        100,
    );

    let mut edge_idx_curr = first_edge_idx;
    let mut node_curr = first_edge.end_idx();
    let mut dir_curr = first_edge.calc_end_tangent(graph)?.mul(-1.);

    let mut iterations = 0;

    while iterations < 1000 {
        let edge_idx_next = graph.get_ccw_edge_of_node(node_curr, dir_curr, Some(edge_idx_curr))?;
        if iterations >= 1 && edge_idx_next == first_edge_idx {
            break;
        }

        edge_idx_curr = edge_idx_next;
        closed_walk.push(edge_idx_next);

        let edge_next = graph.edge(edge_idx_curr)?.directed_from(node_curr);

        #[cfg(feature = "debug_draw")]
        comfy::draw_arrow(
            graph.node(edge_next.start_idx())?.position() + (iterations as f32) * 0.1,
            graph.node(edge_next.end_idx())?.position() + (iterations as f32) * 0.1,
            0.05,
            comfy::GRAY,
            100,
        );

        node_curr = edge_next.other_node_idx(node_curr);
        dir_curr = edge_next.calc_end_tangent(graph)?.mul(-1.);
        println!("Traversed to {node_curr}");

        iterations += 1;
    }

    let length = closed_walk.len();
    if length >= MIN_EDGES_FOR_CYCLE {
        Ok((first_edge_idx, closed_walk))
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

    let mut next_i = 2;
    while next_i < closed_walk.len() {
        let i = next_i - 2;
        let (edge_idx, edge) = closed_walk[i];

        let mut nested_range = None;

        for (end_i, (other_edge_idx, other_edge)) in closed_walk[next_i..].iter().enumerate().rev()
        {
            if i != 0 && end_i != closed_walk.len() && edge.start_idx() == other_edge.end_idx() {
                nested_range = Some(i..(end_i + next_i + 1));
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
            let start = nested_range.start;
            let end = nested_range.end;
            println!("Found nested closed_walk: {start}..{end}");

            let nested_walk: Vec<_> = closed_walk.drain(nested_range).collect();
            for (i, (idx, edge)) in nested_walk.iter().enumerate() {
                println!("\t{i}: {idx} {edge}");
            }
            println!("Original Walk:");
            for (i, (idx, edge)) in closed_walk.iter().enumerate() {
                println!("\t{i}: {idx} {edge}");
            }

            nested_closed_walk.push(nested_walk.into_iter().map(|(idx, _)| idx).collect());
        }

        next_i += 1;
    }

    let parent_closed_walk: Vec<_> = closed_walk.into_iter().map(|(idx, _)| idx).collect();
    Ok((parent_closed_walk, nested_closed_walk))
}
