use bb_vector_network::prelude::*;
use comfy::*;

use crate::{GameState, utils::ColorUtils};

pub struct Node {
    pub node_idx: BBNodeIndex,
    pub radius: f32,
    pub position: Vec2,
}

impl Node {
    pub fn new(node_idx: BBNodeIndex, radius: f32) -> Self {
        Self {
            node_idx,
            radius,
            position: Vec2::ZERO,
        }
    }

    pub fn update_from_graph(&mut self, graph: &BBGraph) -> BBResult<()> {
        let n = graph.node(self.node_idx)?;

        self.position = n.position();

        Ok(())
    }

    pub fn draw(&self, state: &GameState, z_index: i32) {
        let is_selected = state.select_tool.selected_nodes.keys().contains(&self.node_idx);
        let is_hovered = match state.select_tool.hovered_node {
            Some(hovered_node) => hovered_node == self.node_idx,
            None => false,
        };
        let inner_color = match (is_hovered, is_selected) {
            (true, true) => Color::gray(1.),
            (true, false) => Color::gray(0.8),
            (false, true) => Color::gray(0.9),
            (false, false) => Color::gray(0.7),
        };
        let outer_color = match (is_hovered, is_selected) {
            (true, true) => BLUE.add_scalar(0.1),
            (true, false) => BLUE,
            (false, true) => BLUE.add_scalar(-0.1),
            (false, false) => BLUE.add_scalar(-0.2),
        };

        draw_circle(self.position, self.radius * 0.6, inner_color, z_index);
        draw_circle(self.position, self.radius, outer_color, z_index - 1);
    }

    pub fn intersects(&self, p: &Vec2) -> bool {
        p.distance(self.position) < self.radius
    }
}
