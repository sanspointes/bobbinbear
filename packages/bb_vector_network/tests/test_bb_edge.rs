use bb_vector_network::prelude::*;

#[test]
fn directed_from_unchanged() {
    let edge = BBEdge::Line { start: BBNodeIndex(0), end: BBNodeIndex(1) };

    let result = edge.directed_from(BBNodeIndex(0));
    assert_eq!(edge.start_idx(), result.start_idx());
    assert_eq!(edge.end_idx(), result.end_idx());
}

#[test]
fn directed_from_needs_flip() {
    let edge = BBEdge::Line { start: BBNodeIndex(0), end: BBNodeIndex(1) };

    let result = edge.directed_from(BBNodeIndex(1));
    assert_eq!(edge.start_idx(), result.end_idx());
    assert_eq!(edge.end_idx(), result.start_idx());
}

