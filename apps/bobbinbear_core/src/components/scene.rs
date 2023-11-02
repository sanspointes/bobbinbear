use bevy::prelude::*;
use bevy_prototype_lyon::prelude::tess::{
        geom::euclid::{Point2D, UnknownUnit},
        path::Event,
    };

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub enum BBObject {
    // Scene Object type for a vector element
    #[default]
    Vector,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
/// Component represents something that has an index associated with it.
pub struct BBIndex(pub usize);

fn p2_2_v2(p: Point2D<f32, UnknownUnit>) -> Vec2 {
    Vec2::new(p.x, p.y)
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum BBPathEvent {
    Begin {
        at: Vec2,
    },
    Line {
        from: Vec2,
        to: Vec2,
    },
    Quadratic {
        from: Vec2,
        ctrl: Vec2,
        to: Vec2,
    },
    Cubic {
        from: Vec2,
        ctrl1: Vec2,
        ctrl2: Vec2,
        to: Vec2,
    },
    End {
        first: Vec2,
        last: Vec2,
        close: bool,
    },
}
impl Default for BBPathEvent {
    fn default() -> Self {
        Self::Begin { at: Vec2::ZERO }
    }
}

impl From<Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>> for BBPathEvent {
    fn from(value: Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>) -> Self {
        match value {
            Event::Begin { at } => BBPathEvent::Begin { at: p2_2_v2(at) },
            Event::Line { from, to } => BBPathEvent::Line {
                from: p2_2_v2(from),
                to: p2_2_v2(to),
            },
            Event::Quadratic { from, ctrl, to } => BBPathEvent::Quadratic {
                from: p2_2_v2(from),
                ctrl: p2_2_v2(ctrl),
                to: p2_2_v2(to),
            },
            Event::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            } => BBPathEvent::Cubic {
                from: p2_2_v2(from),
                ctrl1: p2_2_v2(ctrl1),
                ctrl2: p2_2_v2(ctrl2),
                to: p2_2_v2(to),
            },
            Event::End { last, first, close } => BBPathEvent::End {
                first: p2_2_v2(first),
                last: p2_2_v2(last),
                close,
            },
        }
    }
}
