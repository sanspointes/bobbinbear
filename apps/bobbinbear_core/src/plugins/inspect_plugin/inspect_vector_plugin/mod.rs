use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_prototype_lyon::{
    prelude::{
        tess::{
            math::Point,
            path::{Event, Path as TessPath},
        },
        Fill, GeometryBuilder, Path, ShapeBundle, Stroke,
    },
    shapes,
};

use crate::{
    components::{
        bbid::BBId,
        scene::{p2_2_v2, BBIndex, BBNode, BBObject, BBPathEvent},
    },
    msgs::cmds::inspect_cmd::InspectingTag,
    plugins::{inspect_plugin::InspectArtifact, selection_plugin::SelectableBundle},
    utils::coordinates,
};

use super::InspectState;

pub struct InspectVectorPlugin;

impl Plugin for InspectVectorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VectorResource::new())
            .add_systems(
                OnEnter(InspectState::InspectVector),
                sys_handle_enter_inspect_vector,
            )
            .add_systems(
                OnExit(InspectState::InspectVector),
                sys_handle_exit_inspect_vector,
            );
    }
}

type TessEvent = Event<Point, Point>;

///
/// Vector Entity Resource
///

// Caches paths so they don't need to be re-calculated
#[derive(Resource)]
pub struct VectorCachedPaths {
    pub control_node: TessPath,
    pub endpoint_node: TessPath,
}
#[derive(Resource)]
pub struct VectorResource {
    pub cached_paths: VectorCachedPaths,
}

impl VectorResource {
    pub fn new() -> Self {
        let control_shape = shapes::Polygon {
            points: vec![
                Vec2::new(-3., -3.),
                Vec2::new(3., -3.),
                Vec2::new(3., 3.),
                Vec2::new(-3., 3.),
            ],
            closed: true,
        };
        let control_node = GeometryBuilder::build_as(&control_shape).0;
        let endpoint_shape = shapes::Circle {
            radius: 5.,
            center: Vec2::ZERO,
        };
        let endpoint_node = GeometryBuilder::build_as(&endpoint_shape).0;

        Self {
            cached_paths: VectorCachedPaths {
                control_node,
                endpoint_node,
            },
        }
    }
}

fn make_vector_node_entity(
    builder: &mut EntityCommands<'_, '_, '_>,
    res: &Res<VectorResource>,
    node_type: BBNode,
    position: Vec2,
    parent_segment: BBId,
) {
    let path = match node_type {
        BBNode::Control => &res.cached_paths.control_node,
        BBNode::Endpoint => &res.cached_paths.endpoint_node,
    };
    builder.insert((
        node_type,
        BBId::default(),
        InspectArtifact(parent_segment),
        SelectableBundle::default(),
        Stroke::new(Color::BLACK, 2.),
        Fill::color(Color::WHITE),
        ShapeBundle {
            path: Path(path.clone()),
            transform: Transform {
                translation: position.extend(2.),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

/// Generates all of the entities require to inspect the currently inspecting BBVector entity.
fn sys_handle_enter_inspect_vector(
    mut commands: Commands,
    res: Res<VectorResource>,
    q_inspected_vector: Query<(Entity, &BBId, &Path), (With<BBObject>, With<InspectingTag>)>,
) {
    let (entity, bbid, path) = q_inspected_vector
        .get_single()
        .expect("sys_handle_enter_inspect_vector: None or more than 1 entity inspecting.");
    info!("sys_handle_exit_inspect_vector: Inspecting {:?}", bbid);

    commands.entity(entity).with_children(|builder| {
        for (i, seg) in path.0.iter().enumerate() {
            let path_seg_bbid = BBId::default();

            if i == 0 {
                let pos = p2_2_v2(seg.from());
                make_vector_node_entity(
                    &mut builder.spawn_empty(),
                    &res,
                    BBNode::Endpoint,
                    pos,
                    path_seg_bbid,
                );
            }

            let mut pb = TessPath::builder();
            let mut name: Option<Name> = None;

            // TODO: Add nodes
            match seg {
                Event::Line { from, to } => {
                    name = Some(Name::from("Line"));
                    pb.begin(from);
                    pb.line_to(to);
                    pb.end(false);

                    make_vector_node_entity(
                        &mut builder.spawn_empty(),
                        &res,
                        BBNode::Endpoint,
                        p2_2_v2(to),
                        path_seg_bbid,
                    );
                }
                Event::Quadratic { from, ctrl, to } => {
                    name = Some(Name::from("Quadratic"));
                    pb.begin(from);
                    pb.quadratic_bezier_to(ctrl, to);
                    pb.end(false);

                    make_vector_node_entity(
                        &mut builder.spawn_empty(),
                        &res,
                        BBNode::Control,
                        p2_2_v2(ctrl),
                        path_seg_bbid,
                    );
                    make_vector_node_entity(
                        &mut builder.spawn_empty(),
                        &res,
                        BBNode::Endpoint,
                        p2_2_v2(to),
                        path_seg_bbid,
                    );
                }
                Event::Cubic {
                    from,
                    ctrl1,
                    ctrl2,
                    to,
                } => {
                    name = Some(Name::from("Cubic"));
                    pb.begin(from);
                    pb.cubic_bezier_to(ctrl1, ctrl2, to);
                    pb.end(false);

                    make_vector_node_entity(
                        &mut builder.spawn_empty(),
                        &res,
                        BBNode::Control,
                        p2_2_v2(ctrl1),
                        path_seg_bbid,
                    );
                    make_vector_node_entity(
                        &mut builder.spawn_empty(),
                        &res,
                        BBNode::Control,
                        p2_2_v2(ctrl2),
                        path_seg_bbid,
                    );
                    make_vector_node_entity(
                        &mut builder.spawn_empty(),
                        &res,
                        BBNode::Endpoint,
                        p2_2_v2(to),
                        path_seg_bbid,
                    );
                }
                Event::Begin { at } => {
                    name = Some(Name::from("Begin"));

                    make_vector_node_entity(
                        &mut builder.spawn_empty(),
                        &res,
                        BBNode::Endpoint,
                        p2_2_v2(at),
                        path_seg_bbid,
                    );
                }
                Event::End { last, first, close } => {
                    name = Some(Name::from("Close"));
                    if close {
                        pb.begin(last);
                        pb.line_to(first);
                        pb.end(false);
                    }
                }
            }

            let name = name
                .expect("sys_handle_enter_inspect_vector: Name is None, this should never happen.");
            let seg_path = pb.build();

            builder.spawn((
                name,
                BBPathEvent::from(seg),
                Stroke::new(Color::BLACK, 2.),
                InspectArtifact(*bbid),
                BBIndex(i),
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
            ));

            println!("Inspecting entity with {seg:?}");
        }
    });
}
fn sys_handle_exit_inspect_vector(
    mut commands: Commands,
    q_inspected_vector: Query<(Entity, &BBId, &Path), (With<BBObject>, With<InspectingTag>)>,
    q_inspect_artifacts: Query<(Entity, &InspectArtifact)>,
) {
    let (_entity, bbid, _path) = q_inspected_vector
        .get_single()
        .expect("sys_handle_enter_inspect_vector: None or more than 1 entity inspecting.");
    info!("sys_handle_exit_inspect_vector: Uinspecting {:?}", bbid);

    let to_remove = q_inspect_artifacts
        .iter()
        .filter_map(|(entity, inspect_artifact)| {
            if inspect_artifact.0 == *bbid {
                Some(entity)
            } else {
                None
            }
        });

    for e in to_remove {
        if let Some(e) = commands.get_entity(e) {
            e.despawn_recursive();
        } else {
            warn!("sys_handle_exit_inspect_vector: Attempted to despawn {e:?} but no entity found.")
        }
    }
}

// fn sys_handle_path_change(
//     q_changed_path:
// ) {
//
// }
