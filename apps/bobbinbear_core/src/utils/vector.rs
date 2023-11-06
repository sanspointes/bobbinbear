use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_prototype_lyon::{
    prelude::{
        tess::{geom::Point, path::Path as TessPath},
        Path, Geometry, GeometryBuilder,
    },
    render::ShapeMaterial,
};

use crate::components::bbpath::BBPath;

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

#[derive(Bundle)]
pub struct BBObjectVectorBundle {
    pub path: Path,
    pub bb_path: BBPath,
    pub mesh: Mesh2dHandle,
    pub material: Handle<ShapeMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
impl Default for BBObjectVectorBundle {
    fn default() -> Self {
        Self {
            path: Path(TessPath::new()),
            bb_path: BBPath::default(),
            mesh: Mesh2dHandle::default(),
            material: Handle::<ShapeMaterial>::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
        }
    }
}
#[allow(dead_code)]
impl BBObjectVectorBundle {
    pub fn from_shape(shape: &impl Geometry) -> Self {
        let tess_path = GeometryBuilder::build_as(shape).0;
        BBObjectVectorBundle::from_tess_path(tess_path)
    }
    pub fn from_tess_path(tess_path: TessPath) -> Self {
        Self {
            bb_path: (&tess_path).into(),
            path: Path(tess_path),
            ..Default::default()
        }
    }
    pub fn from_bb_path(bb_path: BBPath) -> Self {
        Self {
            path: Path((&bb_path).into()),
            bb_path,
            ..Default::default()
        }
    }
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}
