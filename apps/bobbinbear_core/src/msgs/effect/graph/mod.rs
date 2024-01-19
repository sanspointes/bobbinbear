//! Contains effects relating to the `VectorGraph` and `BBGraph` structs.

mod inspect;
mod moved;
mod utils;

use bevy::prelude::*;

pub use inspect::{handle_graph_inspected, handle_graph_uninspected};
pub use moved::handle_bb_node_moved;

use crate::shared::CachedMeshes;

