use std::ops::{Div, Sub, Mul};

use bevy::prelude::*;

/// Converts from screen coordinates to world coordinates
///
/// * `screen_pos`: Screen coordinates in pixels
/// * `window_size`: Window size in pixels
/// * `proj_rect`: Projection bounds in world space
#[inline]
pub fn screen_to_world(screen_pos: Vec2, window_size: Vec2, proj_rect: Rect) -> Vec2 {
    let norm_pos = screen_pos.div(window_size);
    normalized_screen_to_world(norm_pos, proj_rect)
}

/// Converts from normalized screen coordinates to world coordinates
///
/// * `norm_pos`: Normalized screen coordinates [0-1]
/// * `proj_rect`: Projection bounds in world space
#[inline]
pub fn normalized_screen_to_world(norm_pos: Vec2, proj_rect: Rect) -> Vec2 {
    norm_pos.mul_add(proj_rect.size(), proj_rect.min)
}

/// Converts from world coordinates to screen coordinates
///
/// * `world_pos`: Worldspace coordinates
/// * `window_size`: Window size in pixels
/// * `proj_rect`: Projection bounds in world space
pub fn world_to_screen(world_pos: Vec2, window_size: Vec2, proj_rect: Rect) -> Vec2 {
    dbg!(world_pos, proj_rect);
    let norm_pos = world_pos.sub(proj_rect.min).div(proj_rect.size());
    normalized_proj_to_screen(norm_pos, window_size)
}

pub fn normalized_proj_to_screen(norm_pos: Vec2, window_size: Vec2) -> Vec2 {
    norm_pos.mul(window_size)
}

/// TESTS

#[test]
fn screen_to_world_works_in_positive_coord_space() {
    let screen_pos = Vec2::new(10., 10.);
    let window_size = Vec2::new(20., 20.);
    let proj_rect = Rect::new(0., 0., 50., 50.);

    let world_pos = screen_to_world(screen_pos, window_size, proj_rect);
    assert_eq!(world_pos, Vec2::new(25., 25.));

    let screen_pos = Vec2::new(0., 0.);
    let world_pos = screen_to_world(screen_pos, window_size, proj_rect);
    assert_eq!(world_pos, Vec2::new(0., 0.));

    let screen_pos = Vec2::new(20., 20.);
    let world_pos = screen_to_world(screen_pos, window_size, proj_rect);
    assert_eq!(world_pos, Vec2::new(50., 50.));
}

#[test]
fn screen_to_world_works_in_negative_coord_space() {
    let window_size = Vec2::new(20., 20.);
    let proj_rect = Rect::new(-25., -25., 25., 25.);

    let screen_pos = Vec2::new(10., 10.);
    let world_pos = screen_to_world(screen_pos, window_size, proj_rect);
    assert_eq!(world_pos, Vec2::new(0., 0.));

    let screen_pos = Vec2::new(0., 0.);
    let world_pos = screen_to_world(screen_pos, window_size, proj_rect);
    assert_eq!(world_pos, Vec2::new(-25., -25.));

    let screen_pos = Vec2::new(20., 20.);
    let world_pos = screen_to_world(screen_pos, window_size, proj_rect);
    assert_eq!(world_pos, Vec2::new(25., 25.));
}

#[test]
fn world_to_screen_works_in_pos_coord_space() {
    let window_size = Vec2::new(50., 30.);
    let proj_rect = Rect::new(0., 0., 25., 25.);

    let world_pos = Vec2::new(-10., 0.);
    let screen_pos = world_to_screen(world_pos, window_size, proj_rect);
    assert_eq!(screen_pos, Vec2::new(-20., 0.));

    let world_pos = Vec2::new(0., 0.);
    let screen_pos = world_to_screen(world_pos, window_size, proj_rect);
    assert_eq!(screen_pos, Vec2::new(0., 0.));

    let world_pos = Vec2::new(10., 0.);
    let screen_pos = world_to_screen(world_pos, window_size, proj_rect);
    assert_eq!(screen_pos, Vec2::new(20., 0.));

    let world_pos = Vec2::new(10., 10.);
    let screen_pos = world_to_screen(world_pos, window_size, proj_rect);
    assert_eq!(screen_pos, Vec2::new(20., 12.));

    let world_pos = Vec2::new(30., 30.);
    let screen_pos = world_to_screen(world_pos, window_size, proj_rect);
    assert_eq!(screen_pos, Vec2::new(60.000004, 36.));
}

#[test]
fn world_to_screen_works_in_neg_coord_space() {
    let window_size = Vec2::new(50., 30.);
    let proj_rect = Rect::new(-20., -20., 5., 5.);

    let world_pos = Vec2::new(-25., -20.); // x-5 from root
    let screen_pos = world_to_screen(world_pos, window_size, proj_rect);
    assert_eq!(screen_pos, Vec2::new(-10., 0.));

    let world_pos = Vec2::new(-20., -20.);
    let screen_pos = world_to_screen(world_pos, window_size, proj_rect);
    assert_eq!(screen_pos, Vec2::new(0., 0.));

    let world_pos = Vec2::new(0., 0.);
    let screen_pos = world_to_screen(world_pos, window_size, proj_rect);
    assert_eq!(screen_pos, Vec2::new(40., 24.));

    let world_pos = Vec2::new(10., 10.);
    let screen_pos = world_to_screen(world_pos, window_size, proj_rect);
    assert_eq!(screen_pos, Vec2::new(60.000004, 36.0));
}
