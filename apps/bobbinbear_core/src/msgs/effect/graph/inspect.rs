use std::hash::Hash;

use bb_vector_network::prelude::BBError;
use bevy::utils::HashMap;
use bevy::{prelude::*, utils::HashSet};

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
    },
    plugins::{
        bounds_2d_plugin::GlobalBounds2D,
        inspect_plugin::InspectArtifact,
        screen_space_root_plugin::{ScreenSpaceRoot, WorldToScreen},
        selection_plugin::SelectableBundle,
        vector_graph_plugin::VectorGraph,
    },
    shared::CachedMeshes,
    utils::coordinates::{LocalToWorld, ScreenToLocal},
};

// /// Helper method to get the local (BBGraph) position of a BBNode.  Useful applying entity
// /// position changes to the BBGraph.
// pub(super) fn get_node_local_pos(world: &mut World, e: Entity, inspecting_e: Entity) -> Vec3 {
//     let Some(screen_pos) = world.get::<Transform>(e).map(|t| t.translation) else {
//         panic!("Could not get transform of BBNode on move.");
//     };
//     let inspected_world_transform = world.get::<GlobalTransform>(inspecting_e).unwrap();
//     let world_matrix = inspected_world_transform.compute_matrix().inverse();
//
//     let ss_root = world.query::<&ScreenSpaceRoot>().single(world);
//     screen_pos.screen_to_local(&world_matrix, ss_root)
// }
//
// /// Moved callback for when a BBNode::Endpoint is moved.
// pub(super) fn handle_endpoint_node_moved(world: &mut World, entity: Entity) {
//     let inspecting_entity = world.bbid(world.get::<InspectArtifact>(entity).unwrap().0);
//     let new_position = get_node_local_pos(world, entity, inspecting_entity);
//
//     let node_index = BBNodeIndex(world.get::<BBIndex>(entity).unwrap().0);
//     let mut graph = world.get_mut::<VectorGraph>(inspecting_entity).unwrap();
//     graph
//         .0
//         .node_mut(node_index)
//         .unwrap()
//         .set_position(new_position.xy());
//
//     GlobalBounds2D::reset_on_entity(world, inspecting_entity);
// }
//
// /// Moved callback for when a BBNode::Ctrl1 is moved.
// pub(super) fn handle_ctrl1_node_moved(world: &mut World, entity: Entity) {
//     let inspecting_entity = world.bbid(world.get::<InspectArtifact>(entity).unwrap().0);
//     let new_position = get_node_local_pos(world, entity, inspecting_entity);
//
//     let edge_idx = BBEdgeIndex(world.get::<BBIndex>(entity).unwrap().0);
//     let mut graph = world.get_mut::<VectorGraph>(inspecting_entity).unwrap();
//     match graph.0.edge_mut(edge_idx).unwrap() {
//         BBEdge::Line { start, end } => panic!("No line."),
//         BBEdge::Quadratic { ref mut ctrl1, .. } | BBEdge::Cubic { ref mut ctrl1, .. } => {
//             *ctrl1 = new_position.xy()
//         }
//     }
//
//     GlobalBounds2D::reset_on_entity(world, inspecting_entity);
// }
//
// /// Moved callback for when a BBNode::Ctrl2 is moved.
// pub(super) fn handle_ctrl2_node_moved(world: &mut World, entity: Entity) {
//     let inspecting_entity = world.bbid(world.get::<InspectArtifact>(entity).unwrap().0);
//     let new_position = get_node_local_pos(world, entity, inspecting_entity);
//
//     let edge_idx = BBEdgeIndex(world.get::<BBIndex>(entity).unwrap().0);
//     let mut graph = world.get_mut::<VectorGraph>(inspecting_entity).unwrap();
//     match graph.0.edge_mut(edge_idx).unwrap() {
//         BBEdge::Line { start, end } => panic!("No line."),
//         BBEdge::Quadratic { ref mut ctrl1, .. } => panic!("No Quadratic."),
//         BBEdge::Cubic { ref mut ctrl1, .. } => *ctrl1 = new_position.xy(),
//     }
//     GlobalBounds2D::reset_on_entity(world, inspecting_entity);
// }

fn create_node_bundle(
    target: BBId,
    node: BBNode,
    index: usize,
    meshes: &CachedMeshes,
) -> impl Bundle {
    (
        BBId::default(),
        BBNode::Endpoint,
        BBIndex(index),
        InspectArtifact(target),
        SelectableBundle::default(),
        GlobalBounds2D::default(),
        MaterialMesh2dBundle {
            mesh: match node {
                BBNode::Endpoint => meshes.endpoint_node.as_ref().unwrap().clone(),
                BBNode::Ctrl1 | BBNode::Ctrl2 => meshes.control_node.as_ref().unwrap().clone(),
            },
            material: meshes.material.as_ref().unwrap().clone(),
            transform: Transform {
                translation: Vec3::ZERO,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}

/// Sets up inspection entities for the targetted BBVector
// fn setup_for_target(world: &mut World, target: BBId) {
//     println!("effect::graph::inspect::setup_for_target(target: {target})");
//     let target_e = world.bbid(target);
//     let global_matrix = world
//         .get::<GlobalTransform>(target_e)
//         .unwrap()
//         .compute_matrix();
//     let ss_root_entity = ScreenSpaceRoot::get_entity_from_world(world);
//     let cached_meshes = world.resource::<CachedMeshes>().clone();
//
//     let mut ss = SystemState::<(Commands, Query<&mut VectorGraph>)>::new(world);
//     let (mut commands, mut q_vector_graph) = ss.get_mut(world);
//     let graph = q_vector_graph.get_mut(target_e).unwrap();
//
//     for (idx, node) in &graph.0.nodes {
//         let mut e = commands.spawn(create_node_bundle(
//             target,
//             BBNode::Endpoint,
//             idx.0,
//             &cached_meshes,
//         ));
//         e.insert(Name::from(format!("{}", idx).to_string()));
//         e.insert(OnMoveCommand::new(handle_endpoint_node_moved));
//         e.insert(WorldToScreen(
//             node.position().local_to_world(&global_matrix).extend(0.),
//         ));
//         e.set_parent(ss_root_entity);
//     }
//
//     for (idx, edge) in &graph.0.edges {
//         let (ctrl1, ctrl2) = match edge {
//             BBEdge::Line { .. } => (None, None),
//             BBEdge::Quadratic { ctrl1, .. } => (Some(ctrl1), None),
//             BBEdge::Cubic { ctrl1, ctrl2, .. } => (Some(ctrl1), Some(ctrl2)),
//         };
//
//         if let Some(ctrl1) = ctrl1 {
//             let mut e = commands.spawn(create_node_bundle(
//                 target,
//                 BBNode::Ctrl1,
//                 idx.0,
//                 &cached_meshes,
//             ));
//             e.insert(OnMoveCommand::new(handle_ctrl1_node_moved));
//             e.insert(WorldToScreen(
//                 ctrl1.local_to_world(&global_matrix).extend(0.),
//             ));
//             e.set_parent(ss_root_entity);
//         }
//
//         if let Some(ctrl2) = ctrl2 {
//             let mut e = commands.spawn(create_node_bundle(
//                 target,
//                 BBNode::Ctrl2,
//                 idx.0,
//                 &cached_meshes,
//             ));
//             e.insert(OnMoveCommand::new(handle_ctrl2_node_moved));
//             e.insert(WorldToScreen(
//                 ctrl2.local_to_world(&global_matrix).extend(0.),
//             ));
//             e.set_parent(ss_root_entity);
//         }
//     }
//
//     ss.apply(world);
// }
//
// fn cleanup_for_target(world: &mut World, target: BBId) {
//     println!("effect::graph::inspect::cleanup_for_target(target: {target})");
//     let mut q_inspect_nodes = world.query_filtered::<(Entity, &InspectArtifact), With<BBNode>>();
//     let to_remove: Vec<_> = q_inspect_nodes
//         .iter(world)
//         .filter_map(|(e, inspect_artifact)| {
//             if inspect_artifact.0 == target {
//                 Some(e)
//             } else {
//                 None
//             }
//         })
//         .collect();
//     for e in to_remove {
//         world.despawn(e);
//     }
// }

pub fn spawn_ctrl_graph_nodes(world: &mut World, target: BBId, to_spawn: &[BBEdgeIndex]) {
    let ss_root_entity = ScreenSpaceRoot::get_entity_from_world(world);
    let cached_meshes = world.resource::<CachedMeshes>().clone();
    let world_matrix = world.bbid_get::<GlobalTransform>(target).compute_matrix();

    let graph = &world.bbid_get::<VectorGraph>(target).0;
    let to_spawn: Vec<_> = to_spawn
        .iter()
        .flat_map(|idx| {
            let edge = graph.edge(*idx).unwrap();
            let mut nodes = vec![];
            match edge {
                BBEdge::Line { .. } => {}
                BBEdge::Quadratic { ctrl1, .. } => {
                    nodes.push((
                        BBEdgeIndex(idx.0),
                        BBNode::Ctrl1,
                        ctrl1.local_to_world(&world_matrix),
                    ));
                }
                BBEdge::Cubic { ctrl1, ctrl2, .. } => {
                    nodes.push((
                        BBEdgeIndex(idx.0),
                        BBNode::Ctrl1,
                        ctrl1.local_to_world(&world_matrix),
                    ));
                    nodes.push((
                        BBEdgeIndex(idx.0),
                        BBNode::Ctrl2,
                        ctrl2.local_to_world(&world_matrix),
                    ));
                }
            }
            nodes
        })
        .collect();

    for (idx, bbnode, world_pos) in to_spawn {
        let mut v = world.spawn(create_node_bundle(target, bbnode, idx.0, &cached_meshes));
        v.insert(Name::from("Endpoint"));
        v.insert(WorldToScreen(world_pos.extend(0.)));
        v.set_parent(ss_root_entity);
    }
}

fn update_ctrl_graph_nodes(world: &mut World, target: BBId, to_update: &[BBEdgeIndex]) {
    let target_entity = world.bbid(target);

    let mut ss_state = SystemState::<(
        Query<(&BBIndex, &BBNode, &mut WorldToScreen, &InspectArtifact)>,
        Query<(&VectorGraph, &GlobalTransform)>,
    )>::new(world);

    let (mut q_to_update, mut q_target) = ss_state.get_mut(world);
    let (graph, world_matrix) = q_target.get_mut(target_entity).unwrap();
    let graph = &graph.0;
    let world_matrix = world_matrix.compute_matrix();

    let position_lookup: HashMap<(BBEdgeIndex, BBNode), Vec2> = to_update
        .iter()
        .flat_map(|idx| {
            let edge = graph.edge(*idx).unwrap();
            let mut nodes = vec![];

            match edge {
                BBEdge::Line { .. } => {}
                BBEdge::Quadratic { ctrl1, .. } => {
                    let key = (BBEdgeIndex(idx.0), BBNode::Ctrl1);
                    nodes.push((key, ctrl1.local_to_world(&world_matrix)));
                }
                BBEdge::Cubic { ctrl1, ctrl2, .. } => {
                    let key = (BBEdgeIndex(idx.0), BBNode::Ctrl1);
                    nodes.push((key, ctrl1.local_to_world(&world_matrix)));
                    let key = (BBEdgeIndex(idx.0), BBNode::Ctrl2);
                    nodes.push((key, ctrl2.local_to_world(&world_matrix)));
                }
            }
            nodes
        })
        .collect();

    let to_update = q_to_update
        .iter_mut()
        .filter(|(_, _, _, artifact)| artifact.0 == target);

    for (idx, bbnode, mut world_to_screen, _) in to_update {
        let idx = BBEdgeIndex(idx.0);
        if let Some(world_pos) = position_lookup.get(&(idx, *bbnode)) {
            world_to_screen.0 = world_pos.extend(0.);
        }
    }
}

pub fn despawn_ctrl_graph_nodes(world: &mut World, to_despawn: &[BBEdgeIndex], ctrl_lookup: &HashMap<(BBEdgeIndex, BBNode), Entity>) {
    for edge_idx in to_despawn.iter() {
        if let Some(e) = ctrl_lookup.get(&(*edge_idx, BBNode::Ctrl1)) {
            world.despawn(*e);
        }
        if let Some(e) = ctrl_lookup.get(&(*edge_idx, BBNode::Ctrl2)) {
            world.despawn(*e);
        }
    }
}

/// Get a hashmap that maps from BBNodeIndex -> World entity
fn get_endpoint_bbnode_lookup<T: Hash + Eq>(
    world: &mut World,
    target: BBId,
    filter_map_predicate: impl Fn((Entity, &BBIndex, &InspectArtifact, &BBNode)) -> Option<(T, Entity)>,
) -> HashMap<T, Entity> {
    world
        .query::<(Entity, &BBIndex, &InspectArtifact, &BBNode)>()
        .iter(world)
        .filter_map(filter_map_predicate)
        .collect()
}

/// Spawns the entities representing the endpoints of a BBGraph
pub fn spawn_endpoint_graph_nodes(world: &mut World, target: BBId, to_add: &[BBNodeIndex]) {
    let ss_root_entity = ScreenSpaceRoot::get_entity_from_world(world);
    let cached_meshes = world.resource::<CachedMeshes>().clone();
    let world_matrix = world.bbid_get::<GlobalTransform>(target).compute_matrix();

    let graph = &world.bbid_get::<VectorGraph>(target).0;
    let to_add: Vec<_> = to_add
        .iter()
        .map(|idx| {
            let node = graph.node(*idx).unwrap();
            let world_pos = node.position().local_to_world(&world_matrix);

            (*idx, world_pos)
        })
        .collect();

    for (idx, world_pos) in to_add {
        let mut v = world.spawn(create_node_bundle(
            target,
            BBNode::Endpoint,
            idx.0,
            &cached_meshes,
        ));
        v.insert(Name::from("Endpoint"));
        v.insert(WorldToScreen(world_pos.extend(0.)));
        v.set_parent(ss_root_entity);
    }
}

/// Updates the positions of the entities representing the endpoints of a BBGraph
fn update_endpoint_graph_nodes(world: &mut World, target: BBId, to_update: &[BBNodeIndex]) {
    let target_entity = world.bbid(target);

    let mut ss_state = SystemState::<(
        Query<(&BBIndex, &mut WorldToScreen), With<BBNode>>,
        Query<(&VectorGraph, &GlobalTransform)>,
    )>::new(world);

    let (mut q_to_update, mut q_target) = ss_state.get_mut(world);
    let (graph, world_matrix) = q_target.get_mut(target_entity).unwrap();
    let graph = &graph.0;
    let world_matrix = world_matrix.compute_matrix();

    let to_update: Result<HashMap<BBNodeIndex, Vec2>, BBError> = to_update
        .iter()
        .map(|idx| {
            let local_pos = graph.node(*idx)?.position();
            let position = local_pos.local_to_world(&world_matrix);
            Ok((*idx, position))
        })
        .collect();
    let to_update = to_update.unwrap();

    for (idx, mut world_to_screen) in q_to_update.iter_mut() {
        let idx = BBNodeIndex(idx.0);
        if let Some(world_pos) = to_update.get(&idx) {
            world_to_screen.0 = world_pos.extend(0.);
        }
    }
}

/// Despawns the entities representing the endpoints of a BBGraph
fn despawn_endpoint_graph_nodes(
    world: &mut World,
    to_despawn: &[BBNodeIndex],
    lookup: &HashMap<BBNodeIndex, Entity>,
) {
    println!("{to_despawn:?} {lookup:?}");
    for node_idx in to_despawn.iter() {
        if let Some(e) = lookup.get(node_idx) {
            world.despawn(*e);
        }
    }
}

pub fn handle_graph_inspected(world: &mut World, target: BBId) {
    let graph = &world.bbid_get::<VectorGraph>(target).0;
    let node_ids: Vec<_> = graph.nodes.keys().copied().collect();
    let edge_ids: Vec<_> = graph.edges.keys().copied().collect();

    spawn_endpoint_graph_nodes(world, target, &node_ids);
    spawn_ctrl_graph_nodes(world, target, &edge_ids);
}

pub fn handle_inspected_graph_changed(world: &mut World, target: BBId) {
    let endpoint_lookup =
        get_endpoint_bbnode_lookup(world, target, |(e, idx, artifact, bbnode)| {
            let same_target = artifact.0 == target;
            if same_target && bbnode.is_endpoint() {
                Some((BBNodeIndex(idx.0), e))
            } else {
                None
            }
        });

    let ctrl_lookup = get_endpoint_bbnode_lookup(world, target, |(e, idx, artifact, bbnode)| {
        let same_target = artifact.0 == target;
        if same_target && bbnode.is_control() {
            Some(((BBEdgeIndex(idx.0), *bbnode), e))
        } else {
            None
        }
    });

    let graph = &world.bbid_get::<VectorGraph>(target).0;
    // Collect endpoint nodes
    let graph_endpoint_nodes: HashSet<BBNodeIndex> = graph.nodes.keys().copied().collect();
    let scene_endpoint_nodes: HashSet<BBNodeIndex> = endpoint_lookup.keys().copied().collect();

    // Collect edge/ctrl nodes
    let graph_edges: HashSet<BBEdgeIndex> = graph.edges.keys().copied().collect();
    let scene_edges: HashSet<BBEdgeIndex> = ctrl_lookup.keys().map(|(idx, bbnode)| *idx).collect();

    // Sync endpoint nodes

    let to_despawn: Vec<_> = scene_endpoint_nodes
        .difference(&graph_endpoint_nodes)
        .copied()
        .collect();
    despawn_endpoint_graph_nodes(world, &to_despawn, &endpoint_lookup);

    let to_spawn: Vec<_> = graph_endpoint_nodes
        .difference(&scene_endpoint_nodes)
        .copied()
        .collect();
    spawn_endpoint_graph_nodes(world, target, &to_spawn);

    let to_update: Vec<_> = graph_endpoint_nodes
        .intersection(&scene_endpoint_nodes)
        .copied()
        .collect();
    update_endpoint_graph_nodes(world, target, &to_update);

    // Sync edge/ctrl nodes

    let to_despawn: Vec<_> = scene_edges.difference(&graph_edges).copied().collect();
    despawn_ctrl_graph_nodes(world, &to_despawn, &ctrl_lookup);

    let to_spawn: Vec<_> = graph_edges.difference(&scene_edges).copied().collect();
    spawn_ctrl_graph_nodes(world, target, &to_spawn);

    let to_update: Vec<_> = graph_edges.intersection(&scene_edges).copied().collect();
    update_ctrl_graph_nodes(world, target, &to_update);
}

pub fn handle_graph_uninspected(world: &mut World, target: BBId) {
    let graph = &world.bbid_get::<VectorGraph>(target).0;
    let node_ids: Vec<_> = graph.nodes.keys().copied().collect();
    let edge_ids: Vec<_> = graph.edges.keys().copied().collect();
    let endpoint_lookup =
        get_endpoint_bbnode_lookup(world, target, |(e, idx, artifact, bbnode)| {
            let same_target = artifact.0 == target;
            if same_target && bbnode.is_endpoint() {
                Some((BBNodeIndex(idx.0), e))
            } else {
                None
            }
        });
    let ctrl_lookup = get_endpoint_bbnode_lookup(world, target, |(e, idx, artifact, bbnode)| {
        let same_target = artifact.0 == target;
        if same_target && bbnode.is_control() {
            Some(((BBEdgeIndex(idx.0), *bbnode), e))
        } else {
            None
        }
    });

    despawn_endpoint_graph_nodes(world, &node_ids, &endpoint_lookup);
    despawn_ctrl_graph_nodes(world, &edge_ids, &ctrl_lookup);
}
