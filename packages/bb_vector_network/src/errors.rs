use thiserror::Error;

use crate::prelude::*;

#[derive(Error, Debug)]
pub enum BBError {
    #[error("The graph does not contain any nodes or edges.")]
    EmptyGraph,
    #[error("Missing referenced node {0:?}.")]
    MissingNode(BBNodeIndex),
    #[error("Missing referenced edge {0:?}.")]
    MissingEdge(BBEdgeIndex),
    #[error("Found a dead end while traversing closed walk.")]
    ClosedWalkDeadEnd,
    #[error("Closed walk found too few links to be a valid cycle. Expected")]
    ClosedWalkTooSmall(usize),
    #[error("Hit the limits on traversals, edges: {0:?}.")]
    TraversalLimit(Vec<BBEdgeIndex>),
}

impl BBError {
    /// Returns true if this error variant is of missing node / edge / region.
    pub fn is_missing_variant(&self) -> bool {
        matches!(self, BBError::MissingNode(_) | BBError::MissingEdge(_))
    }
}

pub type BBResult<T> = Result<T, BBError>;
