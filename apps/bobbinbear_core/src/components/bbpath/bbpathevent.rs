use bevy::prelude::*;
use bevy_prototype_lyon::prelude::tess::{path::Event, math::Point};

use crate::utils::vector::FromVec2;

#[derive(Debug, Copy, Clone)]
pub enum BBPathEvent {
    Begin {
        from: Vec2,
    },
    Line {
        from: Vec2,
        to: Vec2,
    },
    Quadratic {
        from: Vec2,
        ctrl1: Vec2,
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

impl BBPathEvent {
    pub fn from_position(&self) -> Vec2 {
        match self {
            Self::Begin { from } => *from,
            Self::End { last, .. } => *last,
            Self::Line { from, .. } => *from,
            Self::Quadratic { from, .. } => *from,
            Self::Cubic { from, .. } => *from,
        }
    }

    pub fn to_position(&self) -> Vec2 {
        match self {
            Self::Begin { from } => *from,
            Self::End { first, .. } => *first,
            Self::Line { to, .. } => *to,
            Self::Quadratic { to, .. } => *to,
            Self::Cubic { to, .. } => *to,
        }
    }
}


impl From<Event<Point, Point>> for BBPathEvent {
    fn from(value: Event<Point, Point>) -> Self {
        match value {
            Event::Begin { at } => BBPathEvent::Begin { from: at.into_vec2() },
            Event::Line { from, to } => BBPathEvent::Line {
                from: from.into_vec2(),
                to: to.into_vec2(),
            },
            Event::Quadratic { from, ctrl, to } => BBPathEvent::Quadratic {
                from: from.into_vec2(),
                ctrl1: ctrl.into_vec2(),
                to: to.into_vec2(),
            },
            Event::Cubic {
                from,
                ctrl1,
                ctrl2,
                to,
            } => BBPathEvent::Cubic {
                from: from.into_vec2(),
                ctrl1: ctrl1.into_vec2(),
                ctrl2: ctrl2.into_vec2(),
                to: to.into_vec2(),
            },
            Event::End { last, first, close } => BBPathEvent::End {
                first: first.into_vec2(),
                last: last.into_vec2(),
                close,
            },
        }
    }
}
