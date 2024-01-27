use bb_vector_network::prelude::{BBNodeIndex, BBEdgeIndex, BBEdge};
use bevy::{prelude::*, ecs::system::SystemState};

use crate::{
    components::{bbid::BBIdUtils, scene::{BBIndex, BBNode}},
    msgs::{effect::{ObjectMovedEffect, EffectMsg}, MsgQue},
    plugins::{inspect_plugin::InspectArtifact, vector_graph_plugin::VectorGraph, screen_space_root_plugin::WorldToScreen}, utils::coordinates::{WorldToEntityLocal, WorldToLocal},
};

pub fn handle_bb_node_moved(world: &mut World, effect: &ObjectMovedEffect, responder: &mut MsgQue) {
    let ObjectMovedEffect {
        target,
        world_position,
    } = effect;
    let inspected_bbid = world.bbid_get::<InspectArtifact>(*target).0;
    let inspected_entity = world.bbid(inspected_bbid);
    let node_entity = world.bbid(*target);

    let graph_position = world_position
        .world_to_entity_local(world, inspected_entity)
        .xy();
    println!("graph pos: {graph_position}");

    let mut ss_state = SystemState::<(
        Query<(&mut VectorGraph, &GlobalTransform)>,
        Query<(&BBIndex, &BBNode, &mut WorldToScreen)>,
    )>::new(world);

    let (mut q_vector_graph, mut q_nodes) = ss_state.get_mut(world);
    let (idx, bbnode, mut world_to_screen) = q_nodes.get_mut(node_entity).unwrap();
    world_to_screen.0 = *world_position;

    let (mut graph, global_transform) = q_vector_graph.get_mut(inspected_entity).unwrap();
    let world_matrix = global_transform.compute_matrix();
    let local_position = world_position.world_to_local(&world_matrix);
    println!("local pos: {local_position}");
    let edge_idx = BBEdgeIndex(idx.0);
    match (bbnode, graph.0.edge_mut(edge_idx)) {
        (BBNode::Endpoint, _) => {
            let idx = BBNodeIndex(idx.0);
            graph.0.node_mut(idx).unwrap().set_position(local_position.xy());
        }
        (BBNode::Ctrl1, Ok(BBEdge::Quadratic { ctrl1, .. })) => {
            *ctrl1 = local_position.xy();
        }
        (BBNode::Ctrl1, Ok(BBEdge::Cubic { ctrl1, .. })) => {
            *ctrl1 = local_position.xy();
        }
        (BBNode::Ctrl2, Ok(BBEdge::Cubic { ctrl2, .. })) => {
            *ctrl2 = local_position.xy();
        }
        (bbnode, edge) => {
            panic!("handle_bb_node_moved: Impossible combination.  Cannot update position of {bbnode:?} when it references edge {edge:?}");
        }
    }

    responder.push_internal(EffectMsg::GraphNeedsRemesh { bbid: inspected_bbid });
}
