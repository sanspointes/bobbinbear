use bb_vector_network::prelude::BBError;
use comfy::*;

use crate::GameState;

pub trait ColorUtils {
    fn add_scalar(&self, v: f32) -> Self;
    fn mul_scalar(&self, v: f32) -> Self;
}

impl ColorUtils for Color {
    fn add_scalar(&self, v: f32) -> Self {
        Self::rgb(self.r + v, self.g + v, self.b + v)
    }
    fn mul_scalar(&self, v: f32) -> Self {
        Self::rgb(self.r * v, self.g * v, self.b * v)
    }
}

pub fn screen_top_left_world() -> Vec2 {
    screen_to_world(vec2(0., 0.))
}
pub fn screen_bottom_left_world() -> Vec2 {
    screen_to_world(vec2(0., screen_height()))
}

pub static TEXT_PARAMS: Lazy<TextParams> = Lazy::new(|| TextParams {
    color: WHITE,
    font: egui::FontId::new(12.0, egui::FontFamily::Name("comfy-font".into())),
    ..Default::default()
});

pub static ERR_TEXT_PARAMS: Lazy<TextParams> = Lazy::new(|| TextParams {
    color: RED,
    font: egui::FontId::new(16.0, egui::FontFamily::Name("comfy-font".into())),
    ..Default::default()
});

pub fn draw_bb_error(tag: &str, state: &GameState, error: &BBError) {
    draw_text_ex(
        &format!("{tag} error: {error:?}"),
        screen_bottom_left_world() - vec2(-0.2, 0.),
        TextAlign::BottomLeft,
        ERR_TEXT_PARAMS.clone(),
    );


    #[allow(clippy::single_match)]
    match error {
        BBError::TraversalLimit(edges) => {
            let directed = state.graph.edges_directed(edges).unwrap();
            for (i, (_, edge)) in directed.iter().take(15).enumerate() {
                let alpha = (15 - i) as f32 / 15.;

                let color = if i == 0 {
                    RED
                } else {
                    ORANGE
                };
                draw_arrow(edge.start_pos(&state.graph), edge.end_pos(&state.graph), (1. - alpha) * 0.2 + 0.1, color.alpha(alpha), 200);
            }
        }
        _ => (),
    }
}
