use comfy::*;

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
