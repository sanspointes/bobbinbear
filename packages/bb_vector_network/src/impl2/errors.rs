use thiserror::Error;

use crate::{BBNodeIndex, BBEdgeIndex};

#[derive(Error, Debug)]
pub enum BBError {
    #[error("Missing referenced node {0:?}.")]
    MissingNode(BBNodeIndex),
    #[error("Missing referenced edge {0:?}.")]
    MissingEdge(BBEdgeIndex),
    #[error("Found a dead end while traversing closed walk.")]
    ClosedWalkDeadEnd,
    #[error("Closed walk found too few links to be a valid cycle. Expected")]
    ClosedWalkTooSmall(usize),
}

pub type BBResult<T> = Result<T, BBError>;
