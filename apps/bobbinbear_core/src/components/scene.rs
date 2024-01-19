
use bevy::prelude::*;
// use crate::utils::vector::{FromPoint2, FromVec2};

#[derive(Component, Reflect, Default, serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
/// Represents a scene object that would show in the editor, i.e. a Vector shape, some text.
pub enum BBObject {
    // Scene Object type for a vector element
    #[default]
    Vector,
}

#[derive(Copy, Clone, Default, Component)]
pub enum VectorGraphDirty {
    #[default]
    Default,
    Dirty,
}

#[derive(Component, Reflect, Default, Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
/// Component represents something that has an index associated with it.
pub struct BBIndex(pub usize);

// #[derive(Component, Reflect, Debug, Copy, Clone)]
// #[reflect(Component)]
// /// Stores the position of the segment in worldspace coordinates
// pub enum BBPathEvent {
//     Begin {
//         at: Vec2,
//     },
//     Line {
//         from: Vec2,
//         to: Vec2,
//     },
//     Quadratic {
//         from: Vec2,
//         ctrl: Vec2,
//         to: Vec2,
//     },
//     Cubic {
//         from: Vec2,
//         ctrl1: Vec2,
//         ctrl2: Vec2,
//         to: Vec2,
//     },
//     End {
//         first: Vec2,
//         last: Vec2,
//         close: bool,
//     },
// }
//
// impl BBPathEvent {
//     pub fn from_pos(&self) -> Vec2 {
//         match self {
//             BBPathEvent::Begin { at } => *at,
//             BBPathEvent::Line { from, .. } => *from,
//             BBPathEvent::Quadratic { from, .. } => *from,
//             BBPathEvent::Cubic { from, .. } => *from,
//             BBPathEvent::End { last, .. } => *last,
//         }
//     }
//
//     pub fn to_pos(&self) -> Vec2 {
//         match self {
//             BBPathEvent::Begin { at } => *at,
//             BBPathEvent::Line { to, .. } => *to,
//             BBPathEvent::Quadratic { to, .. } => *to,
//             BBPathEvent::Cubic { to, .. } => *to,
//             BBPathEvent::End { first, .. } => *first,
//         }
//     }
//
//     /// Updates this path event from a BBNode (generalised enum representing which field to
//     /// populate) and position.
//     ///
//     /// * `bb_node`:
//     /// * `pos`:
//     pub fn update_from_bb_node(&mut self, bb_node: BBNode, pos: Vec2) {
//         match (bb_node, self) {
//             (
//                 BBNode::From,
//                 BBPathEvent::Begin { at: ref mut from, .. }
//                 | BBPathEvent::Line { ref mut from, .. }
//                 | BBPathEvent::Quadratic { ref mut from, .. }
//                 | BBPathEvent::Cubic { ref mut from, .. },
//             ) => {
//                 *from = pos;
//             }
//             (
//                 BBNode::To,
//                 BBPathEvent::Line { ref mut to, .. }
//                 | BBPathEvent::Quadratic { ref mut to, .. }
//                 | BBPathEvent::Cubic { ref mut to, .. },
//             ) => {
//                 *to = pos;
//             }
//             (
//                 BBNode::Ctrl1,
//                 BBPathEvent::Quadratic { ref mut ctrl, .. }
//                 | BBPathEvent::Cubic { ctrl1: ref mut ctrl, .. },
//             ) => {
//                 *ctrl = pos;
//             }
//             (
//                 BBNode::Ctrl2,
//                 | BBPathEvent::Cubic { ctrl2: ref mut ctrl, .. },
//             ) => {
//                 *ctrl = pos;
//             }
//             (_, BBPathEvent::End { .. }) => (),
//             (_, bb_path_event) => panic!("BBPathEvent::update_from_bb_node() Unhandled node/path_event combination. {bb_path_event:?} {bb_node:?}."),
//         }
//     }
// }
//
// impl Default for BBPathEvent {
//     fn default() -> Self {
//         Self::Begin { at: Vec2::ZERO }
//     }
// }
//
// impl From<Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>> for BBPathEvent {
//     fn from(value: Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>>) -> Self {
//         match value {
//             Event::Begin { at } => BBPathEvent::Begin { at: at.into_vec2() },
//             Event::Line { from, to } => BBPathEvent::Line {
//                 from: from.into_vec2(),
//                 to: to.into_vec2(),
//             },
//             Event::Quadratic { from, ctrl, to } => BBPathEvent::Quadratic {
//                 from: from.into_vec2(),
//                 ctrl: ctrl.into_vec2(),
//                 to: to.into_vec2(),
//             },
//             Event::Cubic {
//                 from,
//                 ctrl1,
//                 ctrl2,
//                 to,
//             } => BBPathEvent::Cubic {
//                 from: from.into_vec2(),
//                 ctrl1: ctrl1.into_vec2(),
//                 ctrl2: ctrl2.into_vec2(),
//                 to: to.into_vec2(),
//             },
//             Event::End { last, first, close } => BBPathEvent::End {
//                 first: first.into_vec2(),
//                 last: last.into_vec2(),
//                 close,
//             },
//         }
//     }
// }
// impl From<BBPathEvent> for Event<Point2D<f32, UnknownUnit>, Point2D<f32, UnknownUnit>> {
//     fn from(value: BBPathEvent) -> Self {
//         match value {
//             BBPathEvent::Begin { at } => Event::Begin { at: at.into_p2() },
//             BBPathEvent::Line { from, to } => Event::Line {
//                 from: from.into_p2(),
//                 to: to.into_p2(),
//             },
//             BBPathEvent::Quadratic { from, ctrl, to } => Event::Quadratic {
//                 from: from.into_p2(),
//                 ctrl: ctrl.into_p2(),
//                 to: to.into_p2(),
//             },
//             BBPathEvent::Cubic {
//                 from,
//                 ctrl1,
//                 ctrl2,
//                 to,
//             } => Event::Cubic {
//                 from: from.into_p2(),
//                 ctrl1: ctrl1.into_p2(),
//                 ctrl2: ctrl2.into_p2(),
//                 to: to.into_p2(),
//             },
//             BBPathEvent::End { last, first, close } => Event::End {
//                 first: first.into_p2(),
//                 last: last.into_p2(),
//                 close,
//             },
//         }
//     }
// }
//
// #[derive(Component, Reflect, Debug, Clone, Default)]
// #[reflect(Component)]
// pub struct BBPath(pub Vec<BBPathEvent>);
//
// impl From<&TessPath> for BBPath {
//     fn from(value: &TessPath) -> Self {
//         let events: Vec<BBPathEvent> = value.iter().map(|event| event.into()).collect();
//         Self(events)
//     }
// }
//
// impl From<&BBPath> for TessPath {
//     fn from(value: &BBPath) -> Self {
//         let mut pb = PathBuilder::new();
//         for event in &value.0 {
//             match event {
//                 BBPathEvent::Begin { at } => {
//                     pb.move_to(*at);
//                 }
//                 BBPathEvent::Line { to, .. } => {
//                     pb.line_to(*to);
//                 }
//                 BBPathEvent::Quadratic { ctrl, to, .. } => {
//                     pb.quadratic_bezier_to(*ctrl, *to);
//                 }
//                 BBPathEvent::Cubic {
//                     ctrl1, ctrl2, to, ..
//                 } => {
//                     pb.cubic_bezier_to(*ctrl1, *ctrl2, *to);
//                 }
//                 BBPathEvent::End { close, .. } => {
//                     if *close {
//                         pb.close()
//                     }
//                 }
//             }
//         }
//         pb.build().0
//     }
// }

#[derive(Component, Reflect, Default, Debug, Copy, Clone)]
#[reflect(Component)]
pub enum BBNode {
    #[default]
    Endpoint,
    Ctrl1,
    Ctrl2,
}
