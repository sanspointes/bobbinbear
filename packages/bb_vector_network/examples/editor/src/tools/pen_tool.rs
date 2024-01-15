use core::panic;

use bb_vector_network::prelude::*;
use comfy::*;

use crate::{
    tools::ToolUpdateResult,
    utils::{screen_top_left_world, TEXT_PARAMS},
    GameState,
};

use super::{InputEvent, ToolTrait};

#[derive(Debug, Clone, Copy)]
pub enum EdgeStart {
    Preexisting(BBNodeIndex),
    New(Vec2),
}

#[derive(Debug, Clone, Copy)]
pub enum EdgeType {
    Line,
    Quadratic(Vec2),
    Cubic(Vec2, Vec2),
}
impl EdgeType {
    pub fn mix(&self, start: Vec2, end: Vec2, alpha: f32) -> Vec2 {
        match self {
            Self::Line => start.lerp(end, alpha),
            Self::Quadratic(ctrl1) => {
                let v1 = start.lerp(*ctrl1, alpha);
                let v2 = ctrl1.lerp(end, alpha);
                v1.lerp(v2, alpha)
            }
            Self::Cubic(ctrl1, ctrl2) => {
                let v1 = start.lerp(*ctrl1, alpha);
                let v2 = ctrl1.lerp(*ctrl2, alpha);
                let v3 = ctrl2.lerp(end, alpha);

                let v1 = v1.lerp(v2, alpha);
                let v2 = v2.lerp(v3, alpha);
                v1.lerp(v2, alpha)
            }
        }
    }
}

pub fn commit_edge(
    state: &mut GameState,
    start: &EdgeStart,
    end: Option<BBNodeIndex>,
    edge_type: &EdgeType,
) -> BBResult<(BBEdgeIndex, BBEdge)> {
    let result = match (edge_type, start, end) {
        (EdgeType::Line, EdgeStart::New(start_pos), None) => {
            state.graph.line(*start_pos, mouse_world())
        }
        (EdgeType::Line, EdgeStart::Preexisting(start_idx), None) => {
            state.graph.line_from(*start_idx, mouse_world())
        }
        (EdgeType::Line, EdgeStart::New(start_pos), Some(node_idx)) => {
            state.graph.line_to(*start_pos, node_idx)
        }
        (EdgeType::Line, EdgeStart::Preexisting(start_pos), Some(node_idx)) => {
            state.graph.line_from_to(*start_pos, node_idx)
        }
        (EdgeType::Quadratic(ctrl1), EdgeStart::New(start_pos), None) => {
            state.graph.quadratic(*start_pos, *ctrl1, mouse_world())
        }
        (EdgeType::Quadratic(ctrl1), EdgeStart::Preexisting(start_idx), None) => state
            .graph
            .quadratic_from(*start_idx, *ctrl1, mouse_world()),
        (EdgeType::Quadratic(ctrl1), EdgeStart::New(start_pos), Some(node_idx)) => {
            state.graph.quadratic_to(*start_pos, *ctrl1, node_idx)
        }
        (EdgeType::Quadratic(ctrl1), EdgeStart::Preexisting(start_pos), Some(node_idx)) => state
            .graph
            .quadratic_from_to(*start_pos, *ctrl1, node_idx),
        (EdgeType::Cubic(ctrl1, ctrl2), EdgeStart::New(start_pos), None) => {
            state.graph.cubic(*start_pos, *ctrl1, *ctrl2, mouse_world())
        }
        (EdgeType::Cubic(ctrl1, ctrl2), EdgeStart::Preexisting(start_idx), None) => state
            .graph
            .cubic_from(*start_idx, *ctrl1, *ctrl2, mouse_world()),
        (EdgeType::Cubic(ctrl1, ctrl2), EdgeStart::New(start_pos), Some(node_idx)) => {
            state.graph.cubic_to(*start_pos, *ctrl1, *ctrl2, node_idx)
        }
        (EdgeType::Cubic(ctrl1, ctrl2), EdgeStart::Preexisting(start_pos), Some(node_idx)) => state
            .graph
            .cubic_from_to(*start_pos, *ctrl1, *ctrl2, node_idx),
    };

    Ok(result)
}

#[derive(Default, Debug, Clone, Copy)]
pub enum PenToolState {
    #[default]
    Default,
    // Indicates that the user has clicked on a node and is trying to create a new edge from that
    // node.
    EdgeFrom {
        start: EdgeStart,
        edge: EdgeType,
    },
}

impl PenToolState {
    pub fn draw(&self, state: &GameState) -> BBResult<()> {
        match self {
            PenToolState::EdgeFrom { start, edge } => {
                let end = state.intersect_nodes(mouse_world());
                if end.is_none() {
                    draw_circle(mouse_world(), 0.3 * 0.6, WHITE.alpha(0.5), 500);
                    draw_circle(mouse_world(), 0.3, BLUE.alpha(0.5), 500 - 1);
                }
                let end_pos = match end {
                    Some(n) => n.position,
                    None => mouse_world(),
                };
                let start_pos = match start {
                    EdgeStart::Preexisting(idx) => state.graph.node(*idx)?.position(),
                    EdgeStart::New(pos) => *pos,
                };
                for i in 1..16 {
                    let prev_i = i - 1;
                    let prev_a = prev_i as f32 / 16.;
                    let a = i as f32 / 16.;
                    let segment_start = edge.mix(start_pos, end_pos, prev_a);
                    let segment_end = edge.mix(start_pos, end_pos, a);
                    draw_line(segment_start, segment_end, 0.1, WHITE, 500);
                }

                match edge {
                    EdgeType::Line => (),
                    EdgeType::Quadratic(ctrl1) => {
                        draw_line(start_pos, *ctrl1, 0.04, WHITE, 500);
                        draw_line(end_pos, *ctrl1, 0.04, WHITE, 500);
                    }
                    EdgeType::Cubic(ctrl1, ctrl2) => {
                        draw_line(start_pos, *ctrl1, 0.04, WHITE, 500);
                        draw_line(end_pos, *ctrl2, 0.04, WHITE, 500);
                    }
                }
            }
            PenToolState::Default => (),
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct PenTool {
    pub state: PenToolState,
    pub is_hovering: bool,
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
                (PenToolState::Default, InputEvent::MouseMove { position }) => {
                    let n = state.intersect_nodes(*position);
                    state.pen_tool.is_hovering = n.is_none();
                }

                (PenToolState::Default, InputEvent::MouseClick { position }) => {
                    let n = state.intersect_nodes(*position);
                    if let Some(n) = n {
                        state.pen_tool.state = PenToolState::EdgeFrom {
                            start: EdgeStart::Preexisting(n.node_idx),
                            edge: EdgeType::Line,
                        };
                    } else {
                        state.pen_tool.state = PenToolState::EdgeFrom {
                            start: EdgeStart::New(mouse_world()),
                            edge: EdgeType::Line,
                        };
                    }
                }

                (
                    PenToolState::EdgeFrom { start, edge },
                    InputEvent::MouseClick { position },
                ) => {
                    let end_node = state.intersect_nodes(*position).map(|n| n.node_idx);

                    let (_, edge) = commit_edge(state, &start, end_node, &edge)?;

                    match edge {
                        BBEdge::Line { end, .. } => {
                            state.pen_tool.state = PenToolState::EdgeFrom {
                                start: EdgeStart::Preexisting(end),
                                edge: EdgeType::Line,
                            }
                        }
                        BBEdge::Quadratic { ctrl1, end, .. } => {
                            let end_pos = state.graph.node(end)?.position();
                            let diff = end_pos - ctrl1;
                            let next_ctrl_pos = (diff - end_pos) * -1. + diff;
                            state.pen_tool.state = PenToolState::EdgeFrom { start: EdgeStart::Preexisting(end), edge: EdgeType::Quadratic(next_ctrl_pos) }
                        }
                        BBEdge::Cubic { ctrl2, end, .. } => {
                            let end_pos = state.graph.node(end)?.position();
                            let diff = end_pos - ctrl2;
                            let next_ctrl_pos = (diff - end_pos) * -1. + diff;
                            state.pen_tool.state = PenToolState::EdgeFrom { start: EdgeStart::Preexisting(end), edge: EdgeType::Quadratic(next_ctrl_pos) }
                        }
                    }

                    update_result = ToolUpdateResult::RegenerateAll;
                }

                // When user starts creating a new edge via a mousedown -> drag
                (PenToolState::Default, InputEvent::DragStart { position }) => {
                    let n = state.intersect_nodes(*position);
                    let start = if let Some(n) = n {
                        EdgeStart::Preexisting(n.node_idx)
                    } else {
                        EdgeStart::New(*position)
                    };
                    state.pen_tool.state = PenToolState::EdgeFrom { start, edge: EdgeType::Quadratic(mouse_world()) }
                }

                // When a user has started creating an edge and then does a mousedown -> drag
                (PenToolState::EdgeFrom { ref mut edge, start }, InputEvent::DragStart { position }) => {
                    match edge {
                        EdgeType::Line => {
                            println!("DragStart while line, making Quadratic");
                            state.pen_tool.state = PenToolState::EdgeFrom { start, edge: EdgeType::Quadratic(*position) }
                        }
                        EdgeType::Quadratic(ctrl1) => {
                            state.pen_tool.state = PenToolState::EdgeFrom { start, edge: EdgeType::Cubic(*ctrl1, *position) }
                        }
                        EdgeType::Cubic(_, _) => panic!("Impossible state.  Can't initiate a click and drag when already in cubic.")
                    }
                }

                (PenToolState::EdgeFrom { start, ref mut edge }, InputEvent::DragMove { start_position, position }) => {
                    match edge {
                        EdgeType::Line => panic!("Impossible state. If a drag has already started the line should now be a quadratic curve."),
                        EdgeType::Quadratic(ctrl1) => *ctrl1 = mouse_world(),
                        EdgeType::Cubic(ctrl1, ctrl2) => (), 
                    }
                }

                (PenToolState::EdgeFrom { start, ref mut edge }, InputEvent::DragEnd { start_position, position }) => {
                    match edge {
                        EdgeType::Line => panic!("Impossible state. If a drag has already started the line should now be a quadratic curve."),
                        EdgeType::Quadratic(ctrl1) => *ctrl1 = mouse_world(),
                        EdgeType::Cubic(_, end) => { 
                            let _ = commit_edge(state, &start, None, edge);
                        }
                    }
                }

                _ => (),
            }
        }

        if matches!(state.pen_tool.state, PenToolState::Default) && state.pen_tool.is_hovering {
            draw_circle(mouse_world(), 0.3 * 0.6, WHITE.alpha(0.5), 500);
            draw_circle(mouse_world(), 0.3, BLUE.alpha(0.5), 500 - 1);
        }

        state.pen_tool.state.draw(state).unwrap();

        match (
            state.pen_tool.state,
            is_key_released(KeyCode::Escape),
            is_mouse_button_released(MouseButton::Right),
        ) {
            // Exit edge from node mode when
            (PenToolState::EdgeFrom { .. }, true, _) | (PenToolState::EdgeFrom { .. }, _, true) => {
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
