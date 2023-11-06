use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{tess::path::{Event, Path as TessPath}, PathBuilder};

use crate::utils::vector::FromVec2;

use super::{iter::BBPathIter, BBPathEvent};

#[derive(Reflect, Debug, Clone, Default)]
pub enum TessPathVerb {
    #[default]
    LineTo,
    QuadraticTo,
    CubicTo,
    Begin,
    Close,
    End,
}

#[derive(Component, Reflect, Debug, Clone, Default)]
#[reflect(Component)]
pub struct BBPath {
    points: Vec<Vec2>,
    verbs: Vec<TessPathVerb>,
}

impl BBPath {
    /// Creates a new, empty, BBPath.
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            verbs: Vec::new(),
        }
    }

    pub fn iter(&self) -> BBPathIter<'_> {
        BBPathIter::new(&self.points[..], &self.verbs[..])
    }

    /// Gets the n-th element from the BBPath as a BBPathEvent
    ///
    /// * `index`:
    pub fn get(&self, index: usize) -> Option<BBPathEvent> {
        let mut iter = self.iter();
        iter.nth(index)
    }
}

impl From<&TessPath> for BBPath {
    fn from(value: &TessPath) -> Self {
        let mut points: Vec<Vec2> = Vec::new();
        let mut verbs: Vec<TessPathVerb> = Vec::new();

        for event in value {
            match event {
                Event::Begin { at } => {
                    points.push(at.into_vec2());
                    verbs.push(TessPathVerb::Begin);
                }
                Event::Line { to, .. } => {
                    points.push(to.into_vec2());
                    verbs.push(TessPathVerb::LineTo);
                }
                Event::Quadratic { ctrl, to, .. } => {
                    points.push(ctrl.into_vec2());
                    points.push(to.into_vec2());
                    verbs.push(TessPathVerb::QuadraticTo);
                }
                Event::Cubic {
                    ctrl1, ctrl2, to, ..
                } => {
                    points.push(ctrl1.into_vec2());
                    points.push(ctrl2.into_vec2());
                    points.push(to.into_vec2());
                    verbs.push(TessPathVerb::QuadraticTo);
                }
                Event::End { last, first, close } => {
                    if close {
                        points.push(first.into_vec2());
                        verbs.push(TessPathVerb::Close);
                    } else {
                        verbs.push(TessPathVerb::End);
                    }
                }
            }
        }

        Self { points, verbs }
    }
}

impl From<&BBPath> for TessPath {
    fn from(value: &BBPath) -> Self {
        let mut pb = PathBuilder::new();

        for event in value.iter() {
            match event {
                BBPathEvent::Begin { from } => {
                    pb.move_to(from);
                },
                BBPathEvent::Line { to, .. } => {
                    pb.line_to(to);
                }
                BBPathEvent::Quadratic { ctrl1, to, .. } => {
                    pb.quadratic_bezier_to(ctrl1, to);
                }
                BBPathEvent::Cubic {
                    ctrl1, ctrl2, to, ..
                } => {
                    pb.cubic_bezier_to(ctrl1, ctrl2, to);
                }
                BBPathEvent::End { last, first, close } => {
                    if close {
                        pb.close();
                    }
                }
            }
        }

        pb.build().0
    }
}
