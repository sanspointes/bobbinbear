mod utils;

use bevy::{
    math::{Vec3Swizzles, Vec4Swizzles},
    prelude::*,
};

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
        bbpath::{BBPath, BBPathEvent}
    },
    constants::Z_INDEX_BB_NODE,
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
    utils::coordinates::{LocalToScreen, ScreenToLocal},
};

use self::utils::make_path_of_bb_path_event;

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
                (
                    (
                        sys_check_needs_update.pipe(sys_update_bb_nodes),
                        sys_check_needs_update.pipe(sys_update_bb_path_event),
                    ),
                    // sys_handle_bb_node_moved,
                )
                    .chain()
                    .run_if(in_state(InspectState::InspectVector))
                    .after(sys_msg_handler),
            );
    }
}

#[derive(Component, Reflect, Debug, Default)]
/// Tag that marks an entity as a segment of a BBObject::Vector, used with BBIndex to lookup a
/// BBPathEvent.
struct BBVectorSegmentTag;

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
            BBPathEvent::from(seg),
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

fn sys_check_needs_update(
    q_inspected_vector: Query<
        Entity,
        (
            With<BBObject>,
            With<InspectingTag>,
            Or<(Changed<GlobalTransform>, Changed<BBPath>)>,
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
        (Entity, &BBId, &BBPath, &GlobalTransform),
        (With<BBObject>, With<InspectingTag>),
    >,
    q_ss_root: Query<&ScreenSpaceRoot>,
    mut q_bb_node: Query<(&BBNode, &BBIndex, &mut Transform)>,
) {
    if !needs_update {
        return;
    }

    let ss_root = q_ss_root.single();
    let Ok((_entity, _bbid, bb_path, global_transform)) = q_inspected_vector.get_single() else {
        return;
    };

    let global_matrix = global_transform.compute_matrix();

    for (bb_node, bb_index, mut transform) in &mut q_bb_node {
        let Some(segment) = bb_path.get(bb_index.0) else {
            warn!(
                "sys_handle_changed: Attempted to get segment at index {:?} but none found.",
                bb_index.0
            );
            continue;
        };

        match (bb_node, segment) {
            (BBNode::From, seg) => {
                let local_pos = seg.from_position();
                transform.translation = local_pos
                    .local_to_screen(&global_matrix, ss_root)
                    .extend(Z_INDEX_BB_NODE);
            }
            (BBNode::Ctrl1, BBPathEvent::Cubic { ctrl1, .. } | BBPathEvent::Quadratic { ctrl1, .. }) => {
                let local_pos = ctrl1;
                transform.translation = local_pos
                    .local_to_screen(&global_matrix, ss_root)
                    .extend(Z_INDEX_BB_NODE);
            }
            (BBNode::Ctrl2, BBPathEvent::Cubic { ctrl2, .. }) => {
                let local_pos = ctrl2;
                transform.translation = local_pos
                    .local_to_screen(&global_matrix, ss_root)
                    .extend(Z_INDEX_BB_NODE);
            }
            (BBNode::To, seg) => {
                let local_pos: Vec2 = seg.to_position();
                transform.translation = local_pos
                    .local_to_screen(&global_matrix, ss_root)
                    .extend(Z_INDEX_BB_NODE);
            }
            (bb_node, seg) => {
                panic!("sys_update_bb_nodes: Unhandled BBNode/PathEvent combination: \n BBNode: {bb_node:?}\n PathEvent: {seg:?}.")
            }
        }
    }
}

fn sys_update_bb_path_event(
    In(needs_update): In<bool>,
    mut q_inspected_vector: Query<
        (Entity, &BBId, &mut BBPath, &GlobalTransform),
        (With<BBObject>, With<InspectingTag>),
    >,
    q_ss_root: Query<&ScreenSpaceRoot>,
    mut q_bb_path_event: Query<(&BBIndex, &mut Path), (With<BBVectorSegmentTag>, Without<InspectingTag>)>,
) {
    if !needs_update {
        return;
    }

    let ss_root = q_ss_root.single();
    let Ok((_entity, _bbid, mut bb_path, global_transform)) = q_inspected_vector.get_single_mut() else {
        return;
    };

    let global_matrix = global_transform.compute_matrix();

    for (bb_index, mut path) in q_bb_path_event.iter_mut() {
        let Some(segment) = bb_path.get(bb_index.0) else {
            warn!(
                "sys_handle_changed: Attempted to get segment at index {:?} but none found.",
                bb_index.0
            );
            continue;
        };

        let seg_path = make_path_of_bb_path_event(&segment, ss_root, &global_matrix);
        *path = Path(seg_path);
    }
}

// /// When a BBNode moves, updates the cooresponding BBPathEvent entity
// ///
// /// * `q_ss_root`:
// /// * `q_inspected_vector`:
// /// * `param_set_bb_node`:
// fn sys_handle_bb_node_moved(
//     q_ss_root: Query<&ScreenSpaceRoot>,
//     q_inspected_vector: Query<
//         (Entity, &BBId, &mut BBPath, &GlobalTransform),
//         (With<BBObject>, With<InspectingTag>),
//     >,
//     mut param_set_bb_node: ParamSet<(
//         // Query for if a bbnode changed
//         Query<
//             (&BBNode, &BBIndex, &Transform, &InspectArtifact),
//             (Changed<Transform>, With<InspectArtifact>),
//         >,
//         // Query for the BB Path Event objects.
//         Query<(&BBVectorSegmentTag, &BBIndex), Without<InspectingTag>>,
//     )>,
// ) {
//     let Ok((_entity, inspected_bbid, path, global_transform)) = q_inspected_vector.get_single()
//     else {
//         return;
//     };
//
//     let ss_root = q_ss_root.single();
//     let inverse_global_matrix = global_transform.compute_matrix().inverse();
//
//     let mut changed_bb_nodes: Vec<(BBNode, BBIndex, Vec3)> = param_set_bb_node
//         .p0()
//         .iter()
//         .filter(|(_, _, _, inspect_artifact)| inspect_artifact.0.eq(inspected_bbid))
//         .map(|(_1, _2, transform, _3)| (*_1, *_2, transform.translation))
//         .collect();
//
//     for (mut bb_path_event, bb_index) in &mut param_set_bb_node.p1() {
//         let bb_node = changed_bb_nodes.iter().find(|(_, i, _)| bb_index.eq(i));
//
//         let Some((bb_node, _, screen_pos)) = bb_node else {
//             continue;
//         };
//
//         let local_pos = screen_pos
//             .screen_to_local(&inverse_global_matrix, ss_root)
//             .xy();
//
//         bb_path_event.update_from_bb_node(*bb_node, local_pos);
//     }
// }
//
// /// When a BBPathEvent is updated, updated the inspected entity path
// ///
// /// * `q_ss_root`: 
// /// * `q_inspected_vector`: 
// /// * `param_set_bb_path_event`: 
// fn sys_handle_bb_path_event_updated(
//     q_ss_root: Query<&ScreenSpaceRoot>,
//     q_inspected_vector: Query<
//         (Entity, &BBId, &mut Path, &GlobalTransform),
//         (With<BBObject>, With<InspectingTag>),
//     >,
//     mut param_set_bb_path_event: ParamSet<(
//         // Query for changed bb_path_event
//         Query<Entity, With<InspectArtifact>>,
//         Query<(&BBVectorSegmentTag, &BBIndex, &InspectArtifact)>,
//     )>,
// ) {
//     let some_changed = param_set_bb_path_event.p0().iter().next().is_some();
//     if !some_changed {
//         return;
//     }
//     let Ok((_entity, inspected_bbid, mut inspected_path, global_transform)) = q_inspected_vector.get_single()
//     else {
//         return;
//     };
//
//     let ss_root = q_ss_root.single();
//
//     let mut ordered_bb_path_events: Vec<(BBPathEvent, BBIndex)> = param_set_bb_path_event
//         .p1()
//         .iter()
//         .filter(|(_, _, inspect_artifact)| inspect_artifact.0.eq(inspected_bbid))
//         .map(|(bb_path_event, bb_index, _)| (*bb_path_event, *bb_index))
//         .collect();
//     ordered_bb_path_events.sort_by(|a, b| a.1.cmp(&b.1))
//
//
// }
