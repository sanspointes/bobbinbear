use std::ops::{Div, Sub, Mul};

use bevy::prelude::*;

/// Converts from screen coordinates to world coordinates
///
/// * `screen_pos`: Screen coordinates in pixels
/// * `window_size`: Window size in pixels
/// * `proj_rect`: Projection bounds in world space
#[inline]
pub fn screen_to_world(screen_pos: &Vec2, window_size: &Vec2, proj_rect: &Rect) -> Vec2 {
    let norm_pos = screen_pos.div(*window_size);
    normalized_screen_to_world(&norm_pos, proj_rect)
}

/// Converts from normalized screen coordinates to world coordinates
///
/// * `norm_pos`: Normalized screen coordinates [0-1]
/// * `proj_rect`: Projection bounds in world space
#[inline]
pub fn normalized_screen_to_world(norm_pos: &Vec2, proj_rect: &Rect) -> Vec2 {
    norm_pos.mul_add(proj_rect.size(), proj_rect.min)
}

/// Converts from world coordinates to screen coordinates
///
/// * `world_pos`: Worldspace coordinates
/// * `window_size`: Window size in pixels
/// * `proj_rect`: Projection bounds in world space
pub fn world_to_screen(world_pos: &Vec2, window_size: &Vec2, proj_rect: &Rect) -> Vec2 {
    let norm_pos = world_pos.sub(proj_rect.min).div(proj_rect.size());
    normalized_proj_to_screen(&norm_pos, window_size)
}

pub fn normalized_proj_to_screen(norm_pos: &Vec2, window_size: &Vec2) -> Vec2 {
    norm_pos.mul(*window_size)
}
