mod utils;

use bevy::{math::Vec4Swizzles, prelude::*};

use bevy_prototype_lyon::{
    prelude::{
        tess::{
            math::Point,
            path::{Event, Path as TessPath},
        },
        GeometryBuilder, Path,
    },
    shapes,
};

use crate::{
    components::{
        bbid::BBId,
        scene::{BBIndex, BBNode, BBObject},
    },
    msgs::{cmds::inspect_cmd::InspectingTag, sys_msg_handler},
    plugins::{
        inspect_plugin::{
            inspect_vector_plugin::utils::{
                spawn_bbnodes_of_segment, spawn_bbpathevent_of_segment,
            },
            InspectArtifact,
        },
        screen_space_root_plugin::ScreenSpaceRoot,
    },
    prelude::W,
    utils::coordinates::LocalToScreen,
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
            )
            .add_systems(
                Update,
                (sys_check_bb_node_needs_update.pipe(sys_update_bb_nodes))
                    .run_if(in_state(InspectState::InspectVector))
                    .after(sys_msg_handler),
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

/// Generates all of the entities require to inspect the currently inspecting BBVector entity.
fn sys_handle_enter_inspect_vector(
    mut commands: Commands,
    res: Res<VectorResource>,
    q_inspected_vector: Query<
        (Entity, &BBId, &Path, &GlobalTransform),
        (With<BBObject>, With<InspectingTag>),
    >,
    q_ss_root: Query<(Entity, &ScreenSpaceRoot)>,
) {
    let (entity, bbid, path, global_transform) = q_inspected_vector
        .get_single()
        .expect("sys_handle_enter_inspect_vector: None or more than 1 entity inspecting.");
    let parent_matrix = global_transform.compute_matrix();
    let (_, _, parent_pos) = parent_matrix.to_scale_rotation_translation();
    info!(
        "sys_handle_exit_inspect_vector: Inspecting {:?} with pos {parent_pos:?}",
        bbid
    );

    let (ss_root_entity, ss_root) = q_ss_root.single();

    for (i, seg) in path.0.iter().enumerate() {
        spawn_bbnodes_of_segment(
            &mut commands,
            &res,
            *bbid,
            &parent_matrix,
            seg,
            i,
            ss_root_entity,
            ss_root,
        );
        spawn_bbpathevent_of_segment(
            &mut commands,
            *bbid,
            seg,
            i,
            ss_root_entity,
            ss_root,
            &parent_matrix,
        );
    }
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

fn sys_check_bb_node_needs_update(
    q_inspected_vector: Query<
        Entity,
        (
            With<BBObject>,
            With<InspectingTag>,
            Or<(Changed<GlobalTransform>, Changed<Path>)>,
        ),
    >,
    q_ss_root: Query<Entity, Changed<ScreenSpaceRoot>>,
) -> bool {
    let inspected_vector_changed = q_inspected_vector.get_single().is_ok();
    let screenspace_root_changed = q_ss_root.get_single().is_ok();
    inspected_vector_changed || screenspace_root_changed
}

fn sys_update_bb_nodes(
    In(needs_update): In<bool>,
    q_inspected_vector: Query<
        (Entity, &BBId, &Path, &GlobalTransform),
        (With<BBObject>, With<InspectingTag>),
    >,
    q_ss_root: Query<&ScreenSpaceRoot>,
    mut q_bb_node: Query<(&BBNode, &BBIndex, &mut Transform)>,
) {
    if !needs_update {
        return;
    }

    let ss_root = q_ss_root.single();
    let Ok((_entity, _bbid, path, global_transform)) = q_inspected_vector.get_single() else {
        return;
    };

    let segments: Vec<_> = path.0.iter().collect();

    let global_matrix = global_transform.compute_matrix();

    for (bb_node, bb_index, mut transform) in &mut q_bb_node {
        let Some(segment) = segments.get(bb_index.0) else {
            warn!(
                "sys_handle_changed: Attempted to get segment at index {:?} but none found.",
                bb_index.0
            );
            continue;
        };

        match (bb_node, segment) {
            (BBNode::From, seg) => {
                let local_pos: Vec2 = W(seg.from()).into();
                transform.translation = local_pos
                    .local_to_screen(&global_matrix, ss_root)
                    .extend(0.);
            }
            (BBNode::Ctrl1, Event::Cubic { ctrl1: ctrl, .. } | Event::Quadratic { ctrl, .. }) => {
                let local_pos: Vec2 = W(*ctrl).into();
                transform.translation = local_pos
                    .local_to_screen(&global_matrix, ss_root)
                    .extend(0.);
            }
            (BBNode::Ctrl2, Event::Cubic { ctrl2, .. }) => {
                let local_pos: Vec2 = W(*ctrl2).into();
                transform.translation = local_pos
                    .local_to_screen(&global_matrix, ss_root)
                    .extend(0.);
            }
            (BBNode::To, seg) => {
                let local_pos: Vec2 = W(seg.to()).into();
                transform.translation = local_pos
                    .local_to_screen(&global_matrix, ss_root)
                    .extend(0.);
            }
            (bb_node, seg) => {
                panic!("sys_update_bb_nodes: Unhandled BBNode/PathEvent combination: \n BBNode: {bb_node:?}\n PathEvent: {seg:?}.")
            }
        }
    }
}
