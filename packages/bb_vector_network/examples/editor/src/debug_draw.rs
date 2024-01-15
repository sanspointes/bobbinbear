//! Contains logic for modifying the `debug_draw` feature of the package.  Used to visualise the
//! underlying processes behind the package.
//!

use std::fmt::{Display, Write};

use comfy::*;

use crate::{GameState, utils::{TEXT_PARAMS, screen_bottom_left_world}};

#[derive(Default)]
pub struct DebugDrawConfig {
    enabled: bool,
    show_graph: bool,
    show_ccw: bool,
    show_traversal: bool,
}

fn bool_to_check(value: bool) -> &'static str {
    if value {
        "x"
    } else {
        "  "
    }
}

impl Display for DebugDrawConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Debug Mode:")?;
        writeln!(f, "[{}]: Graph (g)", bool_to_check(self.show_graph))?;
        writeln!(f, "[{}]: CW/CCW (c)", bool_to_check(self.show_ccw))?;
        writeln!(f, "[{}]: Traversal (t)", bool_to_check(self.show_traversal))
    }
}

pub fn handle_debug_controls(state: &mut GameState) {
    use bb_vector_network::debug_draw::*;

    if is_key_released(KeyCode::D) {
        state.debug_draw_config.enabled = !state.debug_draw_config.enabled;
    }
    let enabled = state.debug_draw_config.enabled;

    if enabled && is_key_released(KeyCode::G) {
        state.debug_draw_config.show_graph = !state.debug_draw_config.show_graph;
    }

    if enabled && is_key_released(KeyCode::C) {
        let mut v = DEBUG_DRAW_CCW.borrow_mut();
        *v = !*v;
        state.debug_draw_config.show_ccw = *v;
    }

    if enabled && is_key_released(KeyCode::T) {
        let mut v = DEBUG_DRAW_TRAVERSAL.borrow_mut();
        *v = !*v;
        state.debug_draw_config.show_traversal = *v;
    }
}

pub fn draw_debug_controls(state: &GameState) {
    if state.debug_draw_config.enabled {
        draw_text_ex(
            &format!("{}", state.debug_draw_config),
            screen_bottom_left_world(),
            TextAlign::BottomLeft,
            TEXT_PARAMS.clone(),
        );
    }

    if state.debug_draw_config.show_graph {
        for node in state.nodes.iter() {
            draw_text_ex(
                &format!("{} ({:.2},{:.2})", node.node_idx, node.position.x, node.position.y),
                node.position + vec2(0.1, 0.1),
                TextAlign::BottomLeft,
                TEXT_PARAMS.clone(),
            );
        }

        for (idx, edge) in state.graph.edges.iter() {
            draw_arrow(
                edge.start_pos(&state.graph),
                edge.end_pos(&state.graph),
                0.1,
                BLUE,
                100,
            );
            draw_text_ex(
                &format!("{}", idx),
                edge.t_point(&state.graph, 0.5) + vec2(0.1, 0.1),
                TextAlign::BottomLeft,
                TEXT_PARAMS.clone(),
            );
        }
    }
}
