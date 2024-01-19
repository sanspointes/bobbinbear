use bevy::prelude::*;

use bevy::ecs::system::SystemState;

use bb_vector_network::{
    bb_edge::BBEdge,
    prelude::{BBEdgeIndex, BBNodeIndex},
};
use bevy::sprite::MaterialMesh2dBundle;

use crate::{
    components::{
        bbid::{BBId, BBIdUtils},
        scene::{BBIndex, BBNode},
        utility::OnMoveCommand,
    },
    plugins::{
        bounds_2d_plugin::GlobalBounds2D,
        inspect_plugin::InspectArtifact,
        screen_space_root_plugin::{ScreenSpaceRoot, WorldToScreen},
        selection_plugin::SelectableBundle,
        vector_graph_plugin::VectorGraph,
    },
    utils::coordinates::{LocalToWorld, ScreenToLocal}, shared::CachedMeshes,
};

/// Helper method to get the local (BBGraph) position of a BBNode.  Useful applying entity
/// position changes to the BBGraph.
pub(super) fn get_node_local_pos(world: &mut World, e: Entity, inspecting_e: Entity) -> Vec3 {
    let Some(screen_pos) = world.get::<Transform>(e).map(|t| t.translation) else {
        panic!("Could not get transform of BBNode on move.");
    };
    let inspected_world_transform = world.get::<GlobalTransform>(inspecting_e).unwrap();
    let world_matrix = inspected_world_transform.compute_matrix().inverse();

    let ss_root = world.query::<&ScreenSpaceRoot>().single(world);
    screen_pos.screen_to_local(&world_matrix, ss_root)
}

/// Moved callback for when a BBNode::Endpoint is moved.
pub(super) fn handle_endpoint_node_moved(world: &mut World, entity: Entity) {
    let inspecting_entity = world.bbid(world.get::<InspectArtifact>(entity).unwrap().0);
    let new_position = get_node_local_pos(world, entity, inspecting_entity);

    let node_index = BBNodeIndex(world.get::<BBIndex>(entity).unwrap().0);
    let mut graph = world.get_mut::<VectorGraph>(inspecting_entity).unwrap();
    graph
        .0
        .node_mut(node_index)
        .unwrap()
        .set_position(new_position.xy());

    GlobalBounds2D::reset_on_entity(world, inspecting_entity);
}

/// Moved callback for when a BBNode::Ctrl1 is moved.
pub(super) fn handle_ctrl1_node_moved(world: &mut World, entity: Entity) {
    let inspecting_entity = world.bbid(world.get::<InspectArtifact>(entity).unwrap().0);
    let new_position = get_node_local_pos(world, entity, inspecting_entity);

    let edge_idx = BBEdgeIndex(world.get::<BBIndex>(entity).unwrap().0);
    let mut graph = world.get_mut::<VectorGraph>(inspecting_entity).unwrap();
    match graph.0.edge_mut(edge_idx).unwrap() {
        BBEdge::Line { start, end } => panic!("No line."),
        BBEdge::Quadratic { ref mut ctrl1, .. } | BBEdge::Cubic { ref mut ctrl1, .. } => {
            *ctrl1 = new_position.xy()
        }
    }

    GlobalBounds2D::reset_on_entity(world, inspecting_entity);
}

/// Moved callback for when a BBNode::Ctrl2 is moved.
pub(super) fn handle_ctrl2_node_moved(world: &mut World, entity: Entity) {
    let inspecting_entity = world.bbid(world.get::<InspectArtifact>(entity).unwrap().0);
    let new_position = get_node_local_pos(world, entity, inspecting_entity);

    let edge_idx = BBEdgeIndex(world.get::<BBIndex>(entity).unwrap().0);
    let mut graph = world.get_mut::<VectorGraph>(inspecting_entity).unwrap();
    match graph.0.edge_mut(edge_idx).unwrap() {
        BBEdge::Line { start, end } => panic!("No line."),
        BBEdge::Quadratic { ref mut ctrl1, .. } => panic!("No Quadratic."),
        BBEdge::Cubic { ref mut ctrl1, .. } => *ctrl1 = new_position.xy(),
    }
    GlobalBounds2D::reset_on_entity(world, inspecting_entity);
}

/// Sets up inspection entities for the targetted BBVector
fn setup_for_target(world: &mut World, target: BBId) {
    println!("effect::graph::inspect::setup_for_target(target: {target})");
    let target_e = world.bbid(target);
    let global_matrix = world
        .get::<GlobalTransform>(target_e)
        .unwrap()
        .compute_matrix();
    let ss_root_entity = ScreenSpaceRoot::get_entity_from_world(world);
    let cached_meshes = world.resource::<CachedMeshes>().clone();

    let mut ss = SystemState::<(Commands, Query<&mut VectorGraph>)>::new(world);
    let (mut commands, mut q_vector_graph) = ss.get_mut(world);
    let graph = q_vector_graph.get_mut(target_e).unwrap();

    let make_default_node_bundle = |node: BBNode, index: usize| {
        (
            BBId::default(),
            BBNode::Endpoint,
            BBIndex(index),
            InspectArtifact(target),
            SelectableBundle::default(),
            MaterialMesh2dBundle {
                mesh: match node {
                    BBNode::Endpoint => cached_meshes.endpoint_node.as_ref().unwrap().clone(),
                    BBNode::Ctrl1 | BBNode::Ctrl2 => {
                        cached_meshes.control_node.as_ref().unwrap().clone()
                    }
                },
                material: cached_meshes.material.as_ref().unwrap().clone(),
                transform: Transform {
                    translation: Vec3::ZERO,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
    };

    for (idx, node) in &graph.0.nodes {
        let mut e = commands.spawn(make_default_node_bundle(BBNode::Endpoint, idx.0));
        e.insert(Name::from(format!("{}", idx).to_string()));
        e.insert(OnMoveCommand::new(handle_endpoint_node_moved));
        e.insert(WorldToScreen(
            node.position().local_to_world(&global_matrix).extend(0.),
        ));
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
            e.insert(WorldToScreen(
                ctrl1.local_to_world(&global_matrix).extend(0.),
            ));
            e.set_parent(ss_root_entity);
        }

        if let Some(ctrl2) = ctrl2 {
            let mut e = commands.spawn(make_default_node_bundle(BBNode::Ctrl2, idx.0));
            e.insert(OnMoveCommand::new(handle_ctrl2_node_moved));
            e.insert(WorldToScreen(
                ctrl2.local_to_world(&global_matrix).extend(0.),
            ));
            e.set_parent(ss_root_entity);
        }
    }

    ss.apply(world);
}

fn cleanup_for_target(world: &mut World, target: BBId) {
    println!("effect::graph::inspect::cleanup_for_target(target: {target})");
    let mut q_inspect_nodes = world.query_filtered::<(Entity, &InspectArtifact), With<BBNode>>();
    let to_remove: Vec<_> = q_inspect_nodes
        .iter(world)
        .filter_map(|(e, inspect_artifact)| {
            if inspect_artifact.0 == target {
                Some(e)
            } else {
                None
            }
        })
        .collect();
    for e in to_remove {
        world.despawn(e);
    }
}

pub fn handle_graph_inspected(world: &mut World, target: BBId) {
    setup_for_target(world, target);
}

pub fn handle_graph_uninspected(world: &mut World, target: BBId) {
    cleanup_for_target(world, target);
}
