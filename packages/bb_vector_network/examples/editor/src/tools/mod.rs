mod select_tool;

use bb_vector_network::prelude::BBResult;
pub use select_tool::*;

use crate::GameState;

#[derive(Debug)]
pub enum Tool {
    Select,
}

pub enum ToolUpdateResult {
    Noop,           // Do nothing
    RegenerateMesh, // The shape of the graph has mutated, need to regenerate the mesh
    RegenerateAll,  // Both the shape of the graph and the nodes/edges in the graph have been
                    // mutated.  Need to rebuild the node representation + regenerate mesh.
}

pub trait ToolTrait {
    fn update(state: &mut GameState) -> BBResult<ToolUpdateResult>;
}
