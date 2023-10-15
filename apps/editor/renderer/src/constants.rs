use bevy::{render::view::Layer, prelude::Color};

pub const BB_LAYER_SCENE: Layer = 0;
pub const BB_LAYER_UI: Layer = 1;

// pub const BG_HIT_Z_INDEX: f32 = -999.;
// pub const CANVAS_Z_INDEX: f32 = -900.;
// pub const DOC_ELEMENTS_Z_INDEX: f32 = -500.;
// pub const ANCHOR_POINT_Z_INDEX: f32 = -400.;
// pub const FOCUS_RING_Z_INDEX: f32 = -100.;
//
// pub const HOVER_COLOR: Color = Color::rgb(0.038, 0.6, 0.962);
pub const SELECT_COLOR: Color = Color::rgb(0.033, 0.527, 0.869);
pub const SELECTION_BOUNDS_STROKE_WIDTH: f32 = 2.;
//
