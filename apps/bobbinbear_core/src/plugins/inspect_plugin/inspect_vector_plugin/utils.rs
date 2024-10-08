// use bevy::{math::Vec3Swizzles, prelude::*};
//
// use crate::{
//     components::{
//         bbid::BBId,
//         scene::{BBIndex, BBNode},
//     },
//     constants::{Z_INDEX_BB_NODE, Z_INDEX_BB_PATH_EVENT},
//     plugins::{
//         inspect_plugin::InspectArtifact, screen_space_root_plugin::ScreenSpaceRoot,
//         selection_plugin::SelectableBundle,
//     },
//     prelude::W,
//     utils::{coordinates::LocalToScreen, vector::FromVec2},
// };
//
// use super::{VectorResource, BBVectorSegmentTag};

// pub(super) fn make_path_of_bb_path_event(
//     segment: &BBPathEvent,
//     ss_root: &ScreenSpaceRoot,
//     world_transform: &Mat4,
// ) -> TessPath {
//     let mut pb = TessPath::builder();
//
//     match segment {
//         BBPathEvent::Line { from, to } => {
//             pb.begin(W(from.local_to_screen(world_transform, ss_root)).into());
//             pb.line_to(W(to.local_to_screen(world_transform, ss_root)).into());
//             pb.end(false);
//         }
//         BBPathEvent::Quadratic { from, ctrl1, to } => {
//             pb.begin(W(from.local_to_screen(world_transform, ss_root)).into());
//             pb.quadratic_bezier_to(
//                 W(ctrl1.local_to_screen(world_transform, ss_root)).into(),
//                 W(to.local_to_screen(world_transform, ss_root)).into(),
//             );
//             pb.end(false);
//         }
//         BBPathEvent::Cubic {
//             from,
//             ctrl1,
//             ctrl2,
//             to,
//         } => {
//             pb.begin(W(from.local_to_screen(world_transform, ss_root)).into());
//             pb.cubic_bezier_to(
//                 W(ctrl1.local_to_screen(world_transform, ss_root)).into(),
//                 W(ctrl2.local_to_screen(world_transform, ss_root)).into(),
//                 W(to.local_to_screen(world_transform, ss_root)).into(),
//             );
//             pb.end(false);
//         }
//         BBPathEvent::Begin { from } => {}
//         BBPathEvent::End { last, first, close } => {
//             if *close {
//                 pb.begin(W(last.local_to_screen(world_transform, ss_root)).into());
//                 pb.line_to(W(first.local_to_screen(world_transform, ss_root)).into());
//                 pb.end(false);
//             }
//         }
//     }
//
//     pb.build()
// }
//
// pub fn spawn_bbpathevent_of_segment(
//     commands: &mut Commands,
//     inspecting_target: BBId,
//     bb_path_event: BBPathEvent,
//     segment_index: usize,
//     screen_space_entity: Entity,
//     ss_root: &ScreenSpaceRoot,
//     world_transform: &Mat4,
// ) -> (Entity, BBId) {
//     #[allow(unused_assignments)]
//     let name = format!("{bb_path_event:?}");
//
//     let path_seg_bbid = BBId::default();
//     let seg_path = make_path_of_bb_path_event(&bb_path_event, ss_root, world_transform);
//
//     let e = commands
//         .spawn((
//             Name::from(name),
//             BBVectorSegmentTag,
//             Stroke::new(Color::BLACK, 2.),
//             InspectArtifact(inspecting_target),
//             BBIndex(segment_index),
//             path_seg_bbid,
//             SelectableBundle::default(),
//             ShapeBundle {
//                 path: Path(seg_path),
//                 transform: Transform {
//                     translation: Vec3::new(0., 0., Z_INDEX_BB_PATH_EVENT),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//         ))
//         .set_parent(screen_space_entity)
//         .id();
//
//     (e, path_seg_bbid)
// }
//
// /// Spawns all of the vector nodes of a path segment in screenspace.
// #[allow(clippy::too_many_arguments)]
// pub fn spawn_bbnodes_of_segment(
//     commands: &mut Commands,
//     res: &Res<VectorResource>,
//     inspecting_target: BBId,
//     parent_matrix: &Mat4,
//     segment: Event<Point<f32>, Point<f32>>,
//     segment_index: usize,
//     screen_space_entity: Entity,
//     ss_root: &ScreenSpaceRoot,
// ) -> Vec<(Entity, BBId)> {
//     // Spawns a single BBNode
//     let mut spawn_bbnode =
//         |res: &Res<VectorResource>, bb_node: BBNode, local_p: &Point<f32>| -> (Entity, BBId) {
//             let bbid = BBId::default();
//
//             let screen_pos = local_p.local_to_screen(parent_matrix, ss_root).into_vec2();
//
//             let bundle = (
//                 BBId::default(),
//                 bb_node,
//                 BBIndex(segment_index),
//                 InspectArtifact(inspecting_target),
//                 SelectableBundle::default(),
//                 Stroke::new(Color::BLACK, 2.),
//                 Fill::color(Color::WHITE),
//                 Name::from(match bb_node {
//                     BBNode::From => "BBNode::From",
//                     BBNode::Ctrl1 => "BBNode::Ctrl1",
//                     BBNode::Ctrl2 => "BBNode::Ctrl2",
//                     BBNode::To => "BBNode::To",
//                 }),
//                 ShapeBundle {
//                     path: Path(match bb_node {
//                         BBNode::From => res.cached_paths.endpoint_node.clone(),
//                         BBNode::Ctrl1 => res.cached_paths.control_node.clone(),
//                         BBNode::Ctrl2 => res.cached_paths.control_node.clone(),
//                         BBNode::To => res.cached_paths.endpoint_node.clone(),
//                     }),
//                     transform: Transform {
//                         translation: screen_pos.extend(Z_INDEX_BB_NODE),
//                         ..Default::default()
//                     },
//                     ..Default::default()
//                 },
//             );
//
//             let e = commands.spawn(bundle).set_parent(screen_space_entity).id();
//             (e, bbid)
//         };
//
//     let mut spawned: Vec<_> = Vec::with_capacity(3);
//     match segment {
//         Event::Begin { at } => {
//             spawned.push(spawn_bbnode(res, BBNode::From, &at));
//         }
//         Event::Line { from: _, to } => {
//             spawned.push(spawn_bbnode(res, BBNode::To, &to));
//         }
//         Event::Quadratic { from: _, ctrl, to } => {
//             spawned.push(spawn_bbnode(res, BBNode::Ctrl1, &ctrl));
//             spawned.push(spawn_bbnode(res, BBNode::To, &to));
//         }
//         Event::Cubic {
//             from: _,
//             ctrl1,
//             ctrl2,
//             to,
//         } => {
//             spawned.push(spawn_bbnode(res, BBNode::Ctrl1, &ctrl1));
//             spawned.push(spawn_bbnode(res, BBNode::Ctrl2, &ctrl2));
//             spawned.push(spawn_bbnode(res, BBNode::To, &to));
//         }
//         Event::End {
//             last: _,
//             first: _,
//             close: _,
//         } => {}
//     }
//
//     spawned
// }
