use std::thread;

use bb_vector_network::prelude::*;
use glam::Vec2;

fn test_bb_graph_mcb() {
    let mut g = BBGraph::new();

    // Left Cycle
    let (_, first_edge) = g.line(Vec2::new(-6., 0.), Vec2::new(0., 0.));
    let (_, middle_edge) = g.line_from(first_edge.end_idx(), Vec2::new(0., -5.));
    let (_, edge) = g.line_from(middle_edge.end_idx(), Vec2::new(-5., -5.));
    g.line_from_to(edge.end_idx(), first_edge.start_idx());

    // Right Cycle 
    let (_, edge) = g.line_from(middle_edge.start_idx(), Vec2::new(5., 0.));
    let (_, edge) = g.line_from(edge.end_idx(), Vec2::new(5., -5.));
    let (_, _) = g.line_from_to(edge.end_idx(), middle_edge.end_idx());

    println!("{g}");

    match mcb::mcb(&g) {
        Ok(result) => {
            println!("SUCCESS: {result:#?}");
        }
        Err(reason) => {
            println!("FOUND ERROR: {reason:?}");
        }
    };

}

fn main() {
    let thread_handle = thread::spawn(|| {
        println!("\n\n Attempt 1");
        test_bb_graph_mcb();
    });
    thread_handle.join().unwrap();
    let thread_handle = thread::spawn(|| {
        println!("\n\n Attempt 2");
        test_bb_graph_mcb();
    });
    thread_handle.join().unwrap();
    let thread_handle = thread::spawn(|| {
        println!("\n\n Attempt 3");
        test_bb_graph_mcb();
    });
    thread_handle.join().unwrap();
}
