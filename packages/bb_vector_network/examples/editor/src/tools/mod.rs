mod input;
mod pen_tool;
mod select_tool;

use bb_vector_network::prelude::BBResult;
pub use input::*;
pub use pen_tool::*;
pub use select_tool::*;

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

impl ToolUpdateResult {
    pub fn should_update_mesh(&self) -> bool {
        match self {
            ToolUpdateResult::RegenerateMesh | ToolUpdateResult::RegenerateAll => true,
            _ => false,
        }
    }

    pub fn should_update_entities(&self) -> bool {
        match self {
            ToolUpdateResult::RegenerateAll => true,
            _ => false,
        }
    }
}

pub trait ToolTrait {
    fn update(state: &mut GameState, mouse_events: &Vec<InputEvent>) -> BBResult<ToolUpdateResult>;
    /// Resets the tool state (for when you're switching tools)
    fn reset(state: &mut GameState);
}
