mod input;
mod select_tool;
mod pen_tool;

use bb_vector_network::prelude::BBResult;
pub use select_tool::*;
pub use pen_tool::*;
pub use input::*;

use crate::GameState;

#[derive(Debug)]
pub enum Tool {
    Select,
    Pen,
}

pub enum ToolUpdateResult {
    Noop,           // Do nothing
    RegenerateMesh, // The shape of the graph has mutated, need to regenerate the mesh
    RegenerateAll,  // Both the shape of the graph and the nodes/edges in the graph have been
                    // mutated.  Need to rebuild the node representation + regenerate mesh.
}

pub trait ToolTrait {
    fn update(state: &mut GameState, mouse_events: &Vec<InputEvent>) -> BBResult<ToolUpdateResult>;
    /// Resets the tool state (for when you're switching tools)
    fn reset(state: &mut GameState);
}
