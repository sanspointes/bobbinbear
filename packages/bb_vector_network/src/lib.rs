// mod bbanchor;
// mod bbvectornetwork;
// mod bbvnlink;
// mod bbindex;
// mod bbvnregion;
mod traits;
#[cfg(feature = "debug_draw")]
mod debug_draw;

pub mod impl2;

// pub use bbvectornetwork::BBVectorNetwork;
// pub use bbvnlink::BBVNLink;
// pub use bbvnregion::{BBVNRegion, BBVNWindingRule};
pub use impl2::bb_graph::BBGraph;
pub use impl2::bb_edge::{BBEdge, BBEdgeIndex};
pub use impl2::bb_node::{BBNode, BBNodeIndex};
pub use traits::Determinate;
pub use impl2::prelude;
