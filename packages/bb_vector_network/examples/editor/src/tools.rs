use bb_vector_network::{prelude::BBResult, BBNodeIndex};
use comfy::*;

use crate::{utils::screen_top_left_world, GameState};

static TEXT_PARAMS: Lazy<TextParams> = Lazy::new(|| TextParams {
    color: WHITE,
    font: egui::FontId::new(16.0, egui::FontFamily::Name("comfy-font".into())),
    ..Default::default()
});

pub enum Tool {
    Select,
}

pub struct SelectedNodeModel {
    original_pos: Vec2,
}

#[derive(Default)]
pub struct SelectTool {
    pub hovered_node: Option<BBNodeIndex>,
    pub selected_nodes: HashMap<BBNodeIndex, SelectedNodeModel>,
    pub mouse_start_pos: Option<Vec2>,
    pub is_dragging: bool,
}

impl SelectTool {
    fn select_node(state: &mut GameState, node_idx: BBNodeIndex) {
        let original_pos = state.node(node_idx).unwrap().position;
        state
            .select_tool
            .selected_nodes
            .insert(node_idx, SelectedNodeModel { original_pos });
    }

    pub fn update(state: &mut GameState) -> BBResult<()> {
        state.select_tool.hovered_node = state.intersect_nodes(mouse_world()).map(|n| n.node_idx);

        let lm_start_pos = state.select_tool.mouse_start_pos;
        let lm_down = is_mouse_button_down(MouseButton::Left);
        let is_dragging = state.select_tool.is_dragging;

        let top_left_pos = screen_top_left_world();

        draw_text_ex(
            "Select: click to select, hold [ctrl/cmd] to select multiple, right click to delete.",
            top_left_pos - vec2(0., 0.6),
            comfy::TextAlign::TopLeft,
            TEXT_PARAMS.clone(),
        );

        let cmd_pressed = is_key_down(KeyCode::LCtrl) || is_key_down(KeyCode::LGui);

        match (lm_start_pos, lm_down, is_dragging) {
            // Mouse press
            (None, true, false) => {
                state.select_tool.mouse_start_pos = Some(mouse_world());
            }

            // Mouse release while not dragging (click)
            (Some(p), false, false) => {
                state.select_tool.mouse_start_pos = None;
                if let Some(node_idx) = state.intersect_nodes(p).map(|n| n.node_idx) {
                    if !cmd_pressed {
                        state.select_tool.selected_nodes.clear();
                    }
                    SelectTool::select_node(state, node_idx);
                } else {
                    state.select_tool.selected_nodes.clear();
                }
            }

            // While moving but not dragging (check to start dragging)
            (Some(p), true, false) => {
                if mouse_world().distance(p) > 0.01 {
                    state.select_tool.is_dragging = true;
                    if !cmd_pressed {
                        state.select_tool.selected_nodes.clear();
                    }

                    if let Some(node_idx) = state.intersect_nodes(p).map(|n| n.node_idx) {
                        SelectTool::select_node(state, node_idx);
                    }
                }
            }

            // Mouse release while dragging, reset the original positions
            (Some(_), false, true) => {
                state.select_tool.mouse_start_pos = None;
                state.select_tool.is_dragging = false;
                let selected: Vec<_> = state.select_tool.selected_nodes.keys().cloned().collect();
                for node in selected {
                    SelectTool::select_node(state, node);
                }
            }

            // While moving and dragging
            (Some(p), true, true) => {
                let diff = mouse_world() - p;

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
                }
            }

            // Default state, do nothing
            (None, false, false) => (),
            state => panic!("SelectTool mouse event unhandled state. ({state:?})"),
        }

        if let Some(hovered) = state.select_tool.hovered_node {
            if !lm_down && is_mouse_button_down(MouseButton::Right) {
                state.graph.delete_node(hovered)?;
                state.select_tool.selected_nodes.remove(&hovered);
                state.select_tool.hovered_node = None;
                state.rebuild_game_nodes();
            }
        }

        Ok(())
    }
}
