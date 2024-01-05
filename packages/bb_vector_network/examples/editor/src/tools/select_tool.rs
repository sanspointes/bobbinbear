use bb_vector_network::{prelude::BBResult, BBNodeIndex};
use comfy::*;

use crate::{
    utils::{screen_top_left_world, TEXT_PARAMS},
    GameState,
};

use super::{InputEvent, ToolTrait, ToolUpdateResult};

pub struct SelectedNodeModel {
    original_pos: Vec2,
}

#[derive(Default)]
pub struct SelectTool {
    pub hovered_node: Option<BBNodeIndex>,
    pub selected_nodes: HashMap<BBNodeIndex, SelectedNodeModel>,
}

impl SelectTool {
    fn select_node(state: &mut GameState, node_idx: BBNodeIndex) {
        let original_pos = state.node(node_idx).unwrap().position;
        state
            .select_tool
            .selected_nodes
            .insert(node_idx, SelectedNodeModel { original_pos });
    }
}

impl ToolTrait for SelectTool {
    fn update(
        state: &mut GameState,
        mouse_events: &Vec<super::InputEvent>,
    ) -> BBResult<ToolUpdateResult> {
        let mut update_result = ToolUpdateResult::Noop;

        draw_text_ex(
            "Select: click to select, hold [ctrl/cmd] to select multiple, right click to delete.",
            screen_top_left_world() - vec2(-0.1, 1.),
            comfy::TextAlign::TopLeft,
            TEXT_PARAMS.clone(),
        );

        let cmd_pressed = is_key_down(KeyCode::LCtrl) || is_key_down(KeyCode::LGui);

        for ev in mouse_events {
            match ev {
                InputEvent::MouseMove { position } => {
                    state.select_tool.hovered_node =
                        state.intersect_nodes(*position).map(|n| n.node_idx);
                }
                InputEvent::MouseClick { position } => {
                    if let Some(node_idx) = state.intersect_nodes(*position).map(|n| n.node_idx) {
                        if !cmd_pressed {
                            state.select_tool.selected_nodes.clear();
                        }
                        SelectTool::select_node(state, node_idx);
                    } else {
                        state.select_tool.selected_nodes.clear();
                    }
                }
                InputEvent::DragStart { position } => {
                    println!("Drag start");
                    if let Some(node_idx) = state.intersect_nodes(*position).map(|n| n.node_idx) {
                        if !cmd_pressed {
                            state.select_tool.selected_nodes.clear();
                        }
                        SelectTool::select_node(state, node_idx);
                    } else {
                        println!("Deselect all");
                        state.select_tool.selected_nodes.clear();
                    }
                }
                ev @ InputEvent::DragMove {
                    start_position,
                    position,
                }
                | ev @ InputEvent::DragEnd {
                    start_position,
                    position,
                } => {
                    let diff = *position - *start_position;

                    let changeset: Vec<_> = state
                        .select_tool
                        .selected_nodes
                        .iter()
                        .map(|(node_idx, select_model)| {
                            let new_position = select_model.original_pos + diff;
                            (*node_idx, new_position)
                        })
                        .collect();

                    for (node_idx, new_position) in changeset {
                        state.graph.node_mut(node_idx)?.set_position(new_position);
                        state.node_mut(node_idx).unwrap().position = new_position;
                        update_result = ToolUpdateResult::RegenerateMesh;

                        if matches!(ev, InputEvent::DragEnd { .. }) {
                            SelectTool::select_node(state, node_idx);
                        }
                    }

                }
            }
        }

        if let Some(hovered) = state.select_tool.hovered_node {
            if !is_mouse_button_down(MouseButton::Left) && is_mouse_button_released(MouseButton::Right) {
                state.graph.delete_node(hovered)?;
                state.select_tool.selected_nodes.remove(&hovered);
                state.select_tool.hovered_node = None;
                update_result = ToolUpdateResult::RegenerateAll;
            }
        }

        Ok(update_result)
    }

    fn reset(_state: &mut crate::GameState) {

    }
}
