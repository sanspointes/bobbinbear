use bb_vector_network::prelude::*;
use comfy::*;

use crate::{
    tools::ToolUpdateResult,
    utils::{screen_top_left_world, TEXT_PARAMS},
};

use super::{InputEvent, ToolTrait};

#[derive(Default, Debug, Clone, Copy)]
pub enum PenToolState {
    #[default]
    Default,
    // Indicates that the user has clicked on a node and is trying to create a new edge from that
    // node.
    EdgeFromNode {
        node: BBNodeIndex,
    },
}

#[derive(Default)]
pub struct PenTool {
    pub state: PenToolState,
}

impl ToolTrait for PenTool {
    fn update(
        state: &mut crate::GameState,
        mouse_events: &Vec<super::InputEvent>,
    ) -> BBResult<ToolUpdateResult> {
        draw_text_ex(
            "Pen: Click on node to add new edge from that node. Click (and optionally drag) \nin new location to extend the shape.",
            screen_top_left_world() - vec2(-0.1, 1.),
            comfy::TextAlign::TopLeft,
            TEXT_PARAMS.clone(),
        );

        let mut update_result = ToolUpdateResult::Noop;

        for ev in mouse_events {
            match (state.pen_tool.state, ev) {
                (PenToolState::Default, InputEvent::MouseClick { position }) => {
                    let n = state.intersect_nodes(*position);
                    if let Some(n) = n {
                        state.pen_tool.state = PenToolState::EdgeFromNode { node: n.node_idx };
                    } else {
                        state.pen_tool.state = PenToolState::Default;
                    }
                }

                (PenToolState::EdgeFromNode { node }, InputEvent::MouseClick { position }) => {
                    let n = state.intersect_nodes(*position);
                    if let Some(n) = n {
                        if node == n.node_idx {
                            state.pen_tool.state = PenToolState::Default;
                        } else {
                            let (_, edge) = state.graph.line_from_to(node, n.node_idx);
                            state.pen_tool.state = PenToolState::EdgeFromNode {
                                node: edge.end_idx(),
                            }
                        }
                    } else {
                        let (_, edge) = state.graph.line_from(node, mouse_world());
                        state.pen_tool.state = PenToolState::EdgeFromNode {
                            node: edge.end_idx(),
                        }
                    }
                    update_result = ToolUpdateResult::RegenerateAll;
                }

                (PenToolState::EdgeFromNode { node }, InputEvent::MouseMove { position }) => {
                    let source_pos = state.graph.node(node)?.position();
                    draw_line(source_pos, *position, 0.2, WHITE, 5);
                }

                _ => (),
            }
        }

        match (
            state.pen_tool.state,
            is_key_released(KeyCode::Escape),
            is_mouse_button_released(MouseButton::Right),
        ) {
            // Exit edge from node mode when 
            (PenToolState::EdgeFromNode { .. }, true, _)
            | (PenToolState::EdgeFromNode { .. }, _, true) => {
                state.pen_tool.state = PenToolState::Default
            }
            _ => (),
        }

        Ok(update_result)
    }

    fn reset(state: &mut crate::GameState) {
        state.pen_tool.state = PenToolState::Default;
    }
}
