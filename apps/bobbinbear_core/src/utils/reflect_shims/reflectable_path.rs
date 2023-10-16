use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{*, tess::{path::{Path as TessPath, Event}, math::Point}};

#[derive(Reflect, Clone, Copy)]
/// ReflectableEvent is a copy of the lyon Event struct and must match the memory layout exactly.
/// Do not re-order.
pub enum ReflectableEvent {
    Begin { at: Vec2 },
    Line  { from: Vec2, to: Vec2 },
    Quadratic { from: Vec2, ctrl: Vec2, to: Vec2 },
    Cubic { from: Vec2, ctrl1: Vec2, ctrl2: Vec2, to: Vec2 },
    End {
        last: Vec2,
        first: Vec2,
        close: bool,
    },
}
impl From<Event<Point, Point>> for ReflectableEvent {
    fn from(value: Event<Point, Point>) -> Self {
        unsafe {
            std::mem::transmute(value)
        }
    }
}
impl From<ReflectableEvent> for Event<Point, Point> {
    fn from(value: ReflectableEvent) -> Self {
        unsafe {
            std::mem::transmute(value)
        }
    }
}


#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct ReflectablePath(pub Vec<ReflectableEvent>);
impl From<TessPath> for ReflectablePath {
    fn from(value: TessPath) -> Self {
        let mut result: Vec<ReflectableEvent> = Vec::new();
        for verb in value.into_iter() {
            result.push(verb.into());
        }
        Self(result)
    }
}
impl From<ReflectablePath> for Path {
    fn from(value: ReflectablePath) -> Self {
        let mut builder = TessPath::builder();

        for event in value.0.into_iter() {
            builder.path_event(event.into());
        }

        Path(builder.build())
    }
}


