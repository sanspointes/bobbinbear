use bb_vector_network::prelude::{BBEdge, BBEdgeIndex, BBNodeIndex};
use bevy::prelude::*;

use crate::{
    components::{
        bbid::{BBId, BBIdUtils},
        scene::{BBIndex, BBNode, BBObject},
    },
    constants::Z_INDEX_BB_NODE,
    msgs::cmds::inspect_cmd::InspectingTag,
    plugins::{
        bounds_2d_plugin::GlobalBounds2D,
        inspect_plugin::InspectArtifact,
        screen_space_root_plugin::{ScreenSpaceRoot, WorldToScreen},
        vector_graph_plugin::VectorGraph,
    },
    utils::coordinates::{LocalToWorld, ScreenToLocal},
};

pub(super) fn sys_check_needs_update(
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

pub(super) fn sys_update_bb_nodes(
    In(needs_update): In<bool>,
    q_inspected_vector: Query<
        (Entity, &BBId, &VectorGraph, &GlobalTransform),
        (With<BBObject>, With<InspectingTag>),
    >,
    mut q_bb_node: Query<(&BBNode, &BBIndex, &mut WorldToScreen)>,
) {
    if !needs_update {
        return;
    }

    let Ok((_entity, _bbid, vector_graph, global_transform)) = q_inspected_vector.get_single()
    else {
        return;
    };

    let global_matrix = global_transform.compute_matrix();

    for (bb_node, bb_index, mut world_to_screen) in &mut q_bb_node {
        let pos = match bb_node {
            BBNode::Endpoint => vector_graph
                .0
                .node(BBNodeIndex(bb_index.0))
                .unwrap()
                .position(),
            BBNode::Ctrl1 => {
                let edge = vector_graph.0.edge(BBEdgeIndex(bb_index.0)).unwrap();
                match edge {
                    BBEdge::Quadratic { ctrl1, .. } | BBEdge::Cubic { ctrl1, .. } => {
                        *ctrl1
                    }
                    edge => panic!("sys_update_bb_nodes: Trying to update BBNode::Ctrl1 node but it references a {edge:?}."),
                }
            }
            BBNode::Ctrl2 => {
                let edge = vector_graph.0.edge(BBEdgeIndex(bb_index.0)).unwrap();
                match edge {
                    BBEdge::Cubic { ctrl2, .. } => {
                        *ctrl2
                    }
                    edge => panic!("sys_update_bb_nodes: Trying to update BBNode::Ctrl2 node but it references a {edge:?}.")
                }
            }
        };

        world_to_screen.0 = pos.local_to_world(&global_matrix).extend(Z_INDEX_BB_NODE);
    }
}

mod handle_moved_utils {
    use bevy::prelude::*;

    use crate::{
        components::bbid::BBIdUtils,
        plugins::{inspect_plugin::InspectArtifact, screen_space_root_plugin::ScreenSpaceRoot},
        utils::coordinates::ScreenToLocal,
    };

}
