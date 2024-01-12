mod utils;

use bb_vector_network::{
    bb_edge::{BBEdge, BBEdgeIndex},
    bb_node::BBNodeIndex,
};
use bevy::{
    math::{vec2, vec3, Vec3Swizzles, Vec4Swizzles},
    prelude::*,
    render::mesh::MeshVertexAttribute,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    components::{
        bbid::BBId,
        scene::{BBIndex, BBNode, BBObject},
    },
    constants::Z_INDEX_BB_NODE,
    msgs::{cmds::inspect_cmd::InspectingTag, sys_msg_handler},
    plugins::{
        inspect_plugin::InspectArtifact, screen_space_root_plugin::ScreenSpaceRoot,
        selection_plugin::SelectableBundle, vector_graph_plugin::VectorGraph,
    },
    prelude::W,
    utils::{
        coordinates::{LocalToScreen, ScreenToLocal},
        mesh::{add_vertex_colors_mesh, combine_meshes, translate_mesh},
    },
};

// use self::utils::make_path_of_bb_path_event;

use super::InspectState;

pub struct InspectVectorPlugin;

impl Plugin for InspectVectorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VectorResource::default())
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
struct BBEdgeTag;

///
/// Vector Entity Resource
///

// Caches paths so they don't need to be re-calculated
#[derive(Resource, Default)]
pub struct InspectCachedMeshes {
    pub material: Option<Handle<ColorMaterial>>,
    pub control_node: Option<Mesh2dHandle>,
    pub endpoint_node: Option<Mesh2dHandle>,
}
#[derive(Resource, Default)]
pub struct VectorResource {
    pub cached_meshes: InspectCachedMeshes,
}

fn sys_setup_cached_meshes(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut resource: ResMut<VectorResource>,
) {
    resource.cached_meshes.material = Some(materials.add(ColorMaterial::from(Color::WHITE)));
    // Builds the control node mesh (square)
    {
        let mut control_node_m1 = Mesh::from(shape::Quad::new(vec2(3., 3.)));
        add_vertex_colors_mesh(&mut control_node_m1, Color::BLUE);
        let mut control_node_m2 = Mesh::from(shape::Quad::new(vec2(3., 3.)));
        add_vertex_colors_mesh(&mut control_node_m2, Color::WHITE);

        let to_combine = [control_node_m1, control_node_m2];
        let transforms = [
            Transform::default(),
            Transform {
                translation: vec3(0., 0., 0.1),
                ..Default::default()
            },
        ];

        let handle = meshes.add(combine_meshes(
            &to_combine,
            &transforms,
            true,
            true,
            true,
            true,
        ));
        resource.cached_meshes.control_node = Some(handle.into());
    }

    // Builds the control node mesh (square)
    {
        let mut control_node_m1 = Mesh::from(shape::Circle::new(3.));
        add_vertex_colors_mesh(&mut control_node_m1, Color::BLUE);
        let mut control_node_m2 = Mesh::from(shape::Circle::new(3.));
        add_vertex_colors_mesh(&mut control_node_m2, Color::WHITE);

        let to_combine = [control_node_m1, control_node_m2];
        let transforms = [
            Transform::default(),
            Transform {
                translation: vec3(0., 0., 0.1),
                ..Default::default()
            },
        ];

        let handle = meshes.add(combine_meshes(
            &to_combine,
            &transforms,
            true,
            true,
            true,
            true,
        ));
        resource.cached_meshes.endpoint_node = Some(handle.into());
    }
}

/// Generates all of the entities require to inspect the currently inspecting BBVector entity.
fn sys_handle_enter_inspect_vector(
    mut commands: Commands,
    res: Res<VectorResource>,
    q_inspected_vector: Query<
        (Entity, &BBId, &VectorGraph, &GlobalTransform),
        (With<BBObject>, With<InspectingTag>),
    >,
    q_ss_root: Query<(Entity, &ScreenSpaceRoot)>,
) {
    let (entity, bbid, graph, global_transform) = q_inspected_vector
        .get_single()
        .expect("sys_handle_enter_inspect_vector: None or more than 1 entity inspecting.");
    let global_matrix = global_transform.compute_matrix();
    let (_, _, parent_pos) = global_matrix.to_scale_rotation_translation();
    info!(
        "sys_handle_exit_inspect_vector: Inspecting {:?} with pos {parent_pos:?}",
        bbid
    );

    let (ss_root_entity, ss_root) = q_ss_root.single();

    let make_default_node_bundle = |node: BBNode, index: usize, screen_pos: Vec2| {
        (
            BBId::default(),
            BBNode::Endpoint,
            BBIndex(index),
            InspectArtifact(*bbid),
            SelectableBundle::default(),
            MaterialMesh2dBundle {
                mesh: match node {
                    BBNode::Endpoint => res.cached_meshes.endpoint_node.as_ref().unwrap().clone(),
                    BBNode::Ctrl1 | BBNode::Ctrl2 => res.cached_meshes.control_node.as_ref().unwrap().clone(),
                },
                material: res.cached_meshes.material.as_ref().unwrap().clone(),
                transform: Transform {
                    translation: screen_pos.extend(Z_INDEX_BB_NODE),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
    };

    for (idx, node) in &graph.0.nodes {
        let screen_pos = node.position().local_to_screen(&global_matrix, ss_root);
        let mut e = commands.spawn(make_default_node_bundle(
            BBNode::Endpoint,
            idx.0,
            screen_pos,
        ));
        e.insert(Name::from(format!("{}", idx).to_string()));
    }

    for (idx, edge) in &graph.0.edges {
        let (ctrl1, ctrl2) = match edge {
            BBEdge::Line { .. } => (None, None),
            BBEdge::Quadratic { ctrl1, .. } => (Some(ctrl1), None),
            BBEdge::Cubic { ctrl1, ctrl2, .. } => (Some(ctrl1), Some(ctrl2)),
        };

        if let Some(ctrl1) = ctrl1 {
            let screen_pos = ctrl1.local_to_screen(&global_matrix, ss_root);
            let e = commands.spawn(make_default_node_bundle(BBNode::Ctrl1, idx.0, screen_pos));
        }

        if let Some(ctrl2) = ctrl2 {
            let screen_pos = ctrl2.local_to_screen(&global_matrix, ss_root);
            let e = commands.spawn(make_default_node_bundle(BBNode::Ctrl2, idx.0, screen_pos));
        }
    }
}

fn sys_handle_exit_inspect_vector(
    mut commands: Commands,
    q_inspected_vector: Query<(Entity, &BBId, &VectorGraph), (With<BBObject>, With<InspectingTag>)>,
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
            Or<(Changed<GlobalTransform>, Changed<VectorGraph>)>,
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
        (Entity, &BBId, &VectorGraph, &GlobalTransform),
        (With<BBObject>, With<InspectingTag>),
    >,
    q_ss_root: Query<&ScreenSpaceRoot>,
    mut q_bb_node: Query<(&BBNode, &BBIndex, &mut Transform)>,
) {
    if !needs_update {
        return;
    }

    let ss_root = q_ss_root.single();
    let Ok((_entity, _bbid, vector_graph, global_transform)) = q_inspected_vector.get_single()
    else {
        return;
    };

    let global_matrix = global_transform.compute_matrix();

    for (bb_node, bb_index, mut transform) in &mut q_bb_node {
        match bb_node {
            BBNode::Endpoint => {
                let pos = vector_graph
                    .0
                    .node(BBNodeIndex(bb_index.0))
                    .unwrap()
                    .position();
                transform.translation = pos
                    .local_to_screen(&global_matrix, ss_root)
                    .extend(Z_INDEX_BB_NODE);
            }
            BBNode::Ctrl1 => {
                let edge = vector_graph.0.edge(BBEdgeIndex(bb_index.0)).unwrap();
                match edge {
                    edge => panic!("sys_update_bb_nodes: Trying to update BBNode::Ctrl1 node but it references a {edge:?}."),
                    BBEdge::Quadratic { ctrl1, .. } | BBEdge::Cubic { ctrl1, .. } => {
                        transform.translation = ctrl1.local_to_screen(&global_matrix, ss_root).extend(Z_INDEX_BB_NODE);
                    }
                }
            }
            BBNode::Ctrl2 => {
                let edge = vector_graph.0.edge(BBEdgeIndex(bb_index.0)).unwrap();
                match edge {
                    BBEdge::Cubic { ctrl2, .. } => {
                        transform.translation = ctrl2.local_to_screen(&global_matrix, ss_root).extend(Z_INDEX_BB_NODE);
                    }
                    edge => panic!("sys_update_bb_nodes: Trying to update BBNode::Ctrl2 node but it references a {edge:?}.")
                }
            }
        }
    }
}

fn sys_update_bb_path_event(
    In(needs_update): In<bool>,
    mut q_inspected_vector: Query<
        (Entity, &BBId, &mut VectorGraph, &GlobalTransform),
        (With<BBObject>, With<InspectingTag>),
    >,
    q_ss_root: Query<&ScreenSpaceRoot>,
    mut q_bb_path_event: Query<
        (&BBIndex, &mut VectorGraph),
        (With<BBEdgeTag>, Without<InspectingTag>),
    >,
) {
    if !needs_update {
        return;
    }

    let ss_root = q_ss_root.single();
    let Ok((_entity, _bbid, mut bb_path, global_transform)) = q_inspected_vector.get_single_mut()
    else {
        return;
    };

    let global_matrix = global_transform.compute_matrix();

    // for (bb_index, mut graph) in q_bb_path_event.iter_mut() {
    //     let Some(segment) = bb_path.get(bb_index.0) else {
    //         warn!(
    //             "sys_handle_changed: Attempted to get segment at index {:?} but none found.",
    //             bb_index.0
    //         );
    //         continue;
    //     };
    //
    //     let seg_path = make_path_of_bb_path_event(&segment, ss_root, &global_matrix);
    //     *graph = VectorGraph(seg_path);
    // }
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
