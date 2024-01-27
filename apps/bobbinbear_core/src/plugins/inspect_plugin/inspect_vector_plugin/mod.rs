mod systems;
mod utils;

use bevy::math::vec3;
use bevy::{math::vec2, prelude::*, sprite::Mesh2dHandle};

use crate::{
    components::{
        bbid::BBId,
        scene::{BBIndex, BBObject},
    },
    msgs::cmds::inspect_cmd::InspectingTag,
    plugins::{screen_space_root_plugin::ScreenSpaceRoot, vector_graph_plugin::VectorGraph},
    utils::mesh::{add_vertex_colors_mesh, combine_meshes},
};

// use self::utils::make_path_of_bb_path_event;

use self::systems::sys_handle_exit_inspect_vector;

pub use systems::sys_handle_enter_inspect_vector;

use super::InspectState;

pub struct InspectVectorPlugin;

impl Plugin for InspectVectorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VectorResource::default())
            .add_systems(Startup, sys_setup_cached_meshes)
            .add_systems(
                OnEnter(InspectState::InspectVector),
                sys_handle_enter_inspect_vector,
            )
            .add_systems(
                PostUpdate,
                systems::sys_handle_inspected_graph_changed
                    .run_if(in_state(InspectState::InspectVector)),
            )
            .add_systems(
                OnExit(InspectState::InspectVector),
                sys_handle_exit_inspect_vector,
            );
        // .add_systems(
        //     Update,
        //     (
        //         (
        //             sys_check_needs_update.pipe(sys_update_bb_nodes),
        //             sys_check_needs_update.pipe(sys_update_bb_path_event),
        //         ),
        //         // sys_handle_bb_node_moved,
        //     )
        //         .chain()
        //         .run_if(in_state(InspectState::InspectVector))
        //         .after(sys_msg_handler),
        // );
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
#[derive(Resource, Default, Clone)]
pub struct InspectCachedMeshes {
    pub material: Option<Handle<ColorMaterial>>,
    pub control_node: Option<Mesh2dHandle>,
    pub endpoint_node: Option<Mesh2dHandle>,
}
#[derive(Resource, Default, Clone)]
pub struct VectorResource {
    pub cached_meshes: InspectCachedMeshes,
}

pub fn sys_setup_cached_meshes(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut resource: ResMut<VectorResource>,
) {
    resource.cached_meshes.material = Some(materials.add(ColorMaterial::default()));
    // Builds the control node mesh (square)
    {
        let mut control_node_m1 = Mesh::from(shape::Quad::new(vec2(3., 3.)));
        add_vertex_colors_mesh(&mut control_node_m1, Color::WHITE);
        let mut control_node_m2 = Mesh::from(shape::Quad::new(vec2(5., 5.)));
        add_vertex_colors_mesh(&mut control_node_m2, Color::BLUE);

        let to_combine = [control_node_m1, control_node_m2];
        let transforms = [
            Transform::default(),
            Transform {
                translation: vec3(0., 0., 1.),
                ..Default::default()
            },
        ];

        let combined = combine_meshes(&to_combine, &transforms, true, false, true, true);
        println!("Generating control node mesh {combined:?}");
        let handle = meshes.add(combined);
        resource.cached_meshes.control_node = Some(handle.into());
    }

    // Builds the control node mesh (square)
    {
        let mut control_node_m1 = Mesh::from(shape::Circle::new(3.));
        add_vertex_colors_mesh(&mut control_node_m1, Color::BLUE);
        let mut control_node_m2 = Mesh::from(shape::Circle::new(5.));
        add_vertex_colors_mesh(&mut control_node_m2, Color::WHITE);

        let to_combine = [control_node_m1, control_node_m2];
        let transforms = [
            Transform::default(),
            Transform {
                translation: vec3(0., 0., 1.),
                ..Default::default()
            },
        ];

        let handle = meshes.add(combine_meshes(
            &to_combine,
            &transforms,
            true,
            false,
            true,
            true,
        ));
        resource.cached_meshes.endpoint_node = Some(handle.into());
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
