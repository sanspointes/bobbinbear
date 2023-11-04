use bevy::prelude::*;
use bevy_prototype_lyon::prelude::tess::geom::Point;

use super::W;

pub trait FromVec2 {
    fn into_vec2(self) -> Vec2;
    fn from_vec2(other: Vec2) -> Self;
    fn copy_from_vec2(&mut self, other: Vec2) -> &mut Self;
}

impl FromVec2 for Point<f32> {
    fn into_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    fn from_vec2(other: Vec2) -> Self {
        Point::new(other.x, other.y)
    }
    fn copy_from_vec2(&mut self, other: Vec2) -> &mut Self {
        self.x = other.x;
        self.y = other.y;
        self
    }
}

pub trait FromPoint2 {
    fn into_p2(self) -> Point<f32>;
    fn from_p2(other: Point<f32>) -> Self;
    fn copy_from_p2(&mut self, other: Point<f32>) -> &mut Self;
}

impl FromPoint2 for Vec2 {
    fn into_p2(self) -> Point<f32> {
        Point::new(self.x, self.y)
    }
    fn from_p2(other: Point<f32>) -> Self {
        Vec2::new(other.x, other.y)
    }
    fn copy_from_p2(&mut self, other: Point<f32>) -> &mut Self {
        self.x = other.x;
        self.y = other.y;
        self
    }
}

impl From<W<Point<f32>>> for Vec2 {
    fn from(value: W<Point<f32>>) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<W<Vec2>> for Point<f32> {
    fn from(value: W<Vec2>) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
