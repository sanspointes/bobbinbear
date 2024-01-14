use bevy::prelude::*;

use bb_vector_network::{bb_edge::BBEdge, prelude::BBNodeIndex};
use bevy::sprite::MaterialMesh2dBundle;

use crate::{
    components::{
        bbid::{BBId, BBIdUtils},
        scene::{BBIndex, BBNode, BBObject},
        utility::OnMoveCommand,
    },
    msgs::cmds::inspect_cmd::InspectingTag,
    plugins::{
        inspect_plugin::{InspectArtifact, inspect_vector_plugin::sys_update::{handle_endpoint_node_moved, handle_ctrl1_node_moved, handle_ctrl2_node_moved}},
        screen_space_root_plugin::{ScreenSpaceRoot, WorldToScreen},
        selection_plugin::SelectableBundle,
        vector_graph_plugin::VectorGraph,
    },
    utils::coordinates::ScreenToLocal,
};

use super::VectorResource;

/// Generates all of the entities require to inspect the currently inspecting BBVector entity.
pub(super) fn sys_handle_enter_inspect_vector(
    mut commands: Commands,
    res: Res<VectorResource>,
    q_inspected_vector: Query<
        (Entity, &BBId, &VectorGraph, &GlobalTransform),
        (With<BBObject>, With<InspectingTag>),
    >,
    q_ss_root: Query<Entity, With<ScreenSpaceRoot>>,
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

    let ss_root_entity = q_ss_root.single();

    let make_default_node_bundle = |node: BBNode, index: usize| {
        (
            BBId::default(),
            BBNode::Endpoint,
            BBIndex(index),
            InspectArtifact(*bbid),
            SelectableBundle::default(),
            MaterialMesh2dBundle {
                mesh: match node {
                    BBNode::Endpoint => res.cached_meshes.endpoint_node.as_ref().unwrap().clone(),
                    BBNode::Ctrl1 | BBNode::Ctrl2 => {
                        res.cached_meshes.control_node.as_ref().unwrap().clone()
                    }
                },
                material: res.cached_meshes.material.as_ref().unwrap().clone(),
                transform: Transform {
                    translation: Vec3::ZERO,
                    ..Default::default()
                },
                ..Default::default()
            },
            WorldToScreen(global_transform.translation()),
        )
    };

    for (idx, node) in &graph.0.nodes {
        let mut e = commands.spawn(make_default_node_bundle(BBNode::Endpoint, idx.0));
        e.insert(Name::from(format!("{}", idx).to_string()));
        e.insert(OnMoveCommand::new(handle_endpoint_node_moved));
        e.set_parent(ss_root_entity);
    }

    for (idx, edge) in &graph.0.edges {
        let (ctrl1, ctrl2) = match edge {
            BBEdge::Line { .. } => (None, None),
            BBEdge::Quadratic { ctrl1, .. } => (Some(ctrl1), None),
            BBEdge::Cubic { ctrl1, ctrl2, .. } => (Some(ctrl1), Some(ctrl2)),
        };

        if let Some(ctrl1) = ctrl1 {
            let mut e = commands.spawn(make_default_node_bundle(BBNode::Ctrl1, idx.0));
            e.insert(OnMoveCommand::new(handle_ctrl1_node_moved));
            e.set_parent(ss_root_entity);
        }

        if let Some(ctrl2) = ctrl2 {
            let mut e = commands.spawn(make_default_node_bundle(BBNode::Ctrl2, idx.0));
            e.insert(OnMoveCommand::new(handle_ctrl2_node_moved));
            e.set_parent(ss_root_entity);
        }
    }
}

pub(super) fn sys_handle_exit_inspect_vector(
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
