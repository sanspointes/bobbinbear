use std::ops::{Div, Sub, Mul};

use bevy::{prelude::*, math::{Vec4Swizzles, Vec3Swizzles}};

use crate::plugins::screen_space_root_plugin::ScreenSpaceRoot;

use super::W;

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
    let norm_pos = world_pos.sub(proj_rect.min).div(proj_rect.size());
    normalized_proj_to_screen(norm_pos, window_size)
}

pub fn normalized_proj_to_screen(norm_pos: Vec2, window_size: Vec2) -> Vec2 {
    norm_pos.mul(window_size)
}

/// To world trait 
pub trait LocalToScreen {
    fn local_to_screen(&self, world_matrix: &Mat4, ss_root: &ScreenSpaceRoot) -> Self;
}

impl LocalToScreen for Vec2 {
    fn local_to_screen(&self, world_matrix: &Mat4, ss_root: &ScreenSpaceRoot) -> Self {
        let world_pos = self.local_to_world(world_matrix);
        ss_root.world_to_screen(world_pos)
    }
}
impl LocalToScreen for Vec3 {
    fn local_to_screen(&self, world_matrix: &Mat4, ss_root: &ScreenSpaceRoot) -> Self {
        let world_pos = self.local_to_world(world_matrix);
        ss_root.world_to_screen(world_pos.xy()).extend(0.)
    }
}
impl LocalToScreen for Vec4 {
    fn local_to_screen(&self, world_matrix: &Mat4, ss_root: &ScreenSpaceRoot) -> Self {
        let world_pos = self.local_to_world(world_matrix);
        let sp = ss_root.world_to_screen(world_pos.xy());
        Vec4::new(sp.x, sp.y, 0., 1.)
    }
}

pub trait LocalToWorld {
    fn local_to_world(&self, world_matrix: &Mat4) -> Self;
}

impl LocalToWorld for Vec2 {
    fn local_to_world(&self, world_matrix: &Mat4) -> Self {
        world_matrix.mul_vec4(Vec4::new(self.x, self.y, 0., 1.)).xy()
    }
}
impl LocalToWorld for Vec3 {
    fn local_to_world(&self, world_matrix: &Mat4) -> Self {
        world_matrix.mul_vec4(Vec4::new(self.x, self.y, self.z, 1.)).xyz()
    }
}
impl LocalToWorld for Vec4 {
    fn local_to_world(&self, world_matrix: &Mat4) -> Self {
        world_matrix.mul_vec4(*self)
    }
}

pub trait WorldToLocal {
    fn world_to_local(&self, world_matrix: &Mat4) -> Self;
}

impl WorldToLocal for Vec2 {
    fn world_to_local(&self, world_matrix: &Mat4) -> Self {
        world_matrix.inverse().mul_vec4(Vec4::new(self.x, self.y, 0., 1.)).xy()
    }
}
impl WorldToLocal for Vec3 {
    fn world_to_local(&self, world_matrix: &Mat4) -> Self {
        world_matrix.inverse().mul_vec4(Vec4::new(self.x, self.y, self.z, 1.)).xyz()
    }
}
impl WorldToLocal for Vec4 {
    fn world_to_local(&self, world_matrix: &Mat4) -> Self {
        world_matrix.inverse().mul_vec4(*self)
    }
}
pub trait WorldToScreen {
    fn world_to_screen(&self, ss_root: &ScreenSpaceRoot) -> Vec2;
}

impl WorldToScreen for Vec2 {
    fn world_to_screen(&self, ss_root: &ScreenSpaceRoot) -> Vec2 {
        world_to_screen(self.xy(), ss_root.window_size(), ss_root.projection_area())
    }
}
impl WorldToScreen for Vec3 {
    fn world_to_screen(&self, ss_root: &ScreenSpaceRoot) -> Vec2 {
        world_to_screen(self.xy(), ss_root.window_size(), ss_root.projection_area())
    }
}
impl WorldToScreen for Vec4 {
    fn world_to_screen(&self, ss_root: &ScreenSpaceRoot) -> Vec2 {
        world_to_screen(self.xy(), ss_root.window_size(), ss_root.projection_area())
    }
}

pub trait WorldToEntityLocal {
    fn world_to_entity_local(&self, world: &World, entity: Entity) -> Self;
}
impl WorldToEntityLocal for Vec3 {
    fn world_to_entity_local(&self, world: &World, entity: Entity) -> Self {
        let global_matrix = world.get::<GlobalTransform>(entity).unwrap().compute_matrix();
        self.world_to_local(&global_matrix)
    }
}
impl WorldToEntityLocal for Vec4 {
    fn world_to_entity_local(&self, world: &World, entity: Entity) -> Self {
        let global_matrix = world.get::<GlobalTransform>(entity).unwrap().compute_matrix();
        self.world_to_local(&global_matrix)
    }
}

pub trait ScreenToWorld {
    fn screen_to_world(&self, ss_root: &ScreenSpaceRoot) -> Self;
}

impl ScreenToWorld for Vec2 {
    fn screen_to_world(&self, ss_root: &ScreenSpaceRoot) -> Self {
        ss_root.screen_to_world(*self)
    }
}
impl ScreenToWorld for Vec3 {
    fn screen_to_world(&self, ss_root: &ScreenSpaceRoot) -> Self {
        ss_root.screen_to_world(self.xy()).extend(0.)
    }
}
impl ScreenToWorld for Vec4 {
    fn screen_to_world(&self, ss_root: &ScreenSpaceRoot) -> Self {
        let v = ss_root.screen_to_world(self.xy());
        Vec4::new(v.x, v.y, 0., 1.)
    }
}

pub trait ScreenToLocal {
    fn screen_to_local(&self, inverse_world_matrix: &Mat4, ss_root: &ScreenSpaceRoot) -> Self;
}

impl ScreenToLocal for Vec2 {
    fn screen_to_local(&self, inverse_world_matrix: &Mat4, ss_root: &ScreenSpaceRoot) -> Self {
        let wp = self.screen_to_world(ss_root);
        inverse_world_matrix.mul_vec4(Vec4::new(wp.x, wp.y, 0., 1.)).xy()
    }
}
impl ScreenToLocal for Vec3 {
    fn screen_to_local(&self, inverse_world_matrix: &Mat4, ss_root: &ScreenSpaceRoot) -> Self {
        let wp = self.screen_to_world(ss_root);
        inverse_world_matrix.mul_vec4(Vec4::new(wp.x, wp.y, wp.z, 1.)).xyz()
    }
}
impl ScreenToLocal for Vec4 {
    fn screen_to_local(&self, inverse_world_matrix: &Mat4, ss_root: &ScreenSpaceRoot) -> Self {
        let wp = self.screen_to_world(ss_root);
        inverse_world_matrix.mul_vec4(wp)
    }
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
