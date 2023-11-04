use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::prelude::{
    tess::{
        geom::Point,
        path::{Event, Path as TessPath},
    },
    Fill, ShapeBundle, Stroke, *,
};

use crate::{
    components::{
        bbid::BBId,
        scene::{BBIndex, BBNode, BBPathEvent},
    },
    plugins::{
        inspect_plugin::InspectArtifact, screen_space_root_plugin::ScreenSpaceRoot,
        selection_plugin::SelectableBundle,
    },
    prelude::*,
    utils::{
        coordinates::LocalToScreen,
        vector::{FromPoint2, FromVec2},
    },
};

use super::VectorResource;

pub fn spawn_bbpathevent_of_segment(
    commands: &mut Commands,
    inspecting_target: BBId,
    segment: Event<Point<f32>, Point<f32>>,
    segment_index: usize,
    screen_space_entity: Entity,
    ss_root: &ScreenSpaceRoot,
    world_transform: &Mat4,
) -> (Entity, BBId) {
    let mut pb = TessPath::builder();

    #[allow(unused_assignments)]
    let mut name: Option<Name> = None;

    match segment {
        Event::Line { from, to } => {
            name = Some(Name::from("BBPathEvent::Line"));
            pb.begin(from.local_to_screen(world_transform, ss_root));
            pb.line_to(to.local_to_screen(world_transform, ss_root));
            pb.end(false);
        }
        Event::Quadratic { from, ctrl, to } => {
            name = Some(Name::from("BBPathEvent::Quadratic"));
            pb.begin(from.local_to_screen(world_transform, ss_root));
            pb.quadratic_bezier_to(
                ctrl.local_to_screen(world_transform, ss_root),
                to.local_to_screen(world_transform, ss_root),
            );
            pb.end(false);
        }
        Event::Cubic {
            from,
            ctrl1,
            ctrl2,
            to,
        } => {
            name = Some(Name::from("BBPathEvent::Cubic"));

            pb.begin(from.local_to_screen(world_transform, ss_root));
            pb.cubic_bezier_to(
                ctrl1.local_to_screen(world_transform, ss_root),
                ctrl2.local_to_screen(world_transform, ss_root),
                to.local_to_screen(world_transform, ss_root),
            );
            pb.end(false);
        }
        Event::Begin { at } => {
            name = Some(Name::from("BBPathEvent::Begin"));
        }
        Event::End { last, first, close } => {
            name = Some(Name::from("BBPathEvent::Close"));
            if close {
                pb.begin(last.local_to_screen(world_transform, ss_root));
                pb.line_to(first.local_to_screen(world_transform, ss_root));
                pb.end(false);
            }
        }
    }

    let name =
        name.expect("sys_handle_enter_inspect_vector: Name is None, this should never happen.");
    let seg_path = pb.build();

    let path_seg_bbid = BBId::default();

    let e = commands
        .spawn((
            name,
            BBPathEvent::from(segment),
            Stroke::new(Color::BLACK, 2.),
            InspectArtifact(inspecting_target),
            BBIndex(segment_index),
            path_seg_bbid,
            SelectableBundle::default(),
            ShapeBundle {
                path: Path(seg_path),
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .set_parent(screen_space_entity)
        .id();

    (e, path_seg_bbid)
}

/// Spawns all of the vector nodes of a path segment in screenspace.
#[allow(clippy::too_many_arguments)]
pub fn spawn_bbnodes_of_segment(
    commands: &mut Commands,
    res: &Res<VectorResource>,
    inspecting_target: BBId,
    parent_matrix: &Mat4,
    segment: Event<Point<f32>, Point<f32>>,
    segment_index: usize,
    screen_space_entity: Entity,
    ss_root: &ScreenSpaceRoot,
) -> Vec<(Entity, BBId)> {
    let mut temp_vec = Vec2::default();
    // Spawns a single BBNode
    let mut spawn_bbnode =
        |res: &Res<VectorResource>, bb_node: BBNode, local_p: &Point<f32>| -> (Entity, BBId) {
            let bbid = BBId::default();

            let vec3 = temp_vec.copy_from_p2(*local_p).extend(0.);
            let vec3 = parent_matrix.transform_vector3(vec3);
            let ss_position = ss_root.world_to_screen(vec3.xy());

            let bundle = (
                BBId::default(),
                bb_node,
                BBIndex(segment_index),
                InspectArtifact(inspecting_target),
                SelectableBundle::default(),
                Stroke::new(Color::BLACK, 2.),
                Fill::color(Color::WHITE),
                Name::from(match bb_node {
                    BBNode::From => "BBNode::From",
                    BBNode::Ctrl1 => "BBNode::Ctrl1",
                    BBNode::Ctrl2 => "BBNode::Ctrl2",
                    BBNode::To => "BBNode::To",
                }),
                ShapeBundle {
                    path: Path(match bb_node {
                        BBNode::From => res.cached_paths.endpoint_node.clone(),
                        BBNode::Ctrl1 => res.cached_paths.control_node.clone(),
                        BBNode::Ctrl2 => res.cached_paths.control_node.clone(),
                        BBNode::To => res.cached_paths.endpoint_node.clone(),
                    }),
                    transform: Transform {
                        translation: ss_position.extend(0.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            );

            let e = commands.spawn(bundle).set_parent(screen_space_entity).id();
            (e, bbid)
        };

    let mut spawned: Vec<_> = Vec::with_capacity(3);
    match segment {
        Event::Begin { at } => {
            spawned.push(spawn_bbnode(res, BBNode::From, &at));
        }
        Event::Line { from: _, to } => {
            spawned.push(spawn_bbnode(res, BBNode::To, &to));
        }
        Event::Quadratic { from: _, ctrl, to } => {
            spawned.push(spawn_bbnode(res, BBNode::Ctrl1, &ctrl));
            spawned.push(spawn_bbnode(res, BBNode::To, &to));
        }
        Event::Cubic {
            from: _,
            ctrl1,
            ctrl2,
            to,
        } => {
            spawned.push(spawn_bbnode(res, BBNode::Ctrl1, &ctrl1));
            spawned.push(spawn_bbnode(res, BBNode::Ctrl2, &ctrl2));
            spawned.push(spawn_bbnode(res, BBNode::To, &to));
        }
        Event::End {
            last: _,
            first: _,
            close: _,
        } => {}
    }

    spawned
}
