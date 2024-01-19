use bb_vector_network::prelude::BBNodeIndex;
use bevy::prelude::*;

use crate::{
    components::{bbid::BBIdUtils, scene::BBIndex},
    msgs::{effect::{ObjectMovedEffect, EffectMsg}, MsgQue},
    plugins::{inspect_plugin::InspectArtifact, vector_graph_plugin::VectorGraph},
    utils::coordinates::WorldToEntityLocal,
};

pub fn handle_bb_node_moved(world: &mut World, effect: &ObjectMovedEffect, responder: &mut MsgQue) {
    let ObjectMovedEffect {
        target,
        world_position,
    } = effect;

    let inspected_bbid = world.bbid_get::<InspectArtifact>(*target).0;
    let inspected_entity = world.bbid(inspected_bbid);

    let graph_position = world_position
        .world_to_entity_local(world, inspected_entity)
        .xy();

    let node_idx = BBNodeIndex(world.bbid_get::<BBIndex>(*target).0);
    let mut graph = world.get_mut::<VectorGraph>(inspected_entity).unwrap();
    graph
        .0
        .node_mut(node_idx)
        .unwrap()
        .set_position(graph_position);

    responder.push_internal(EffectMsg::GraphNeedsRemesh { bbid: inspected_bbid });
}
