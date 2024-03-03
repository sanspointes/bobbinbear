//! Lifecycle methods for handling when edges/endpoints are spawned/despawned.

use bevy_ecs::{
    entity::{Entity, EntityHashSet},
    query::{Added, Changed, QueryEntityError, Without},
    removal_detection::RemovedComponents,
    system::{In, Query, QueryLens},
};
use bevy_hierarchy::Parent;
use bevy_transform::components::Transform;
use bevy_utils::HashSet;

use crate::components::{Edge, Endpoint, EndpointPosition, VectorGraphic, VectorGraphicPathStorage};

/// Adds any added endpoints to the hashset within the parent VectorGraphic
pub fn sys_add_spawned_endpoints_to_vector_graphic(
    q_endpoints: Query<(Entity, &Parent), Added<Endpoint>>,
    mut q_vector_graphic: Query<&mut VectorGraphic, Without<Endpoint>>,
) {
    for (entity, parent) in &q_endpoints {
        let Ok(mut vg) = q_vector_graphic.get_mut(parent.get()) else {
            continue;
        };
        vg.endpoints.insert(entity);
    }
}

/// Adds any added edges to the hashset within the parent VectorGraphic
pub fn sys_add_spawned_edges_to_vector_graphic(
    q_edges: Query<(Entity, &Parent), Added<Edge>>,
    mut q_vector_graphic: Query<&mut VectorGraphic, Without<Edge>>,
) {
    for (entity, parent) in &q_edges {
        let Ok(mut vg) = q_vector_graphic.get_mut(parent.get()) else {
            continue;
        };
        vg.edges.insert(entity);
    }
}

/// removes any despawned endpoints from the hashset within the parent VectorGraphic
pub fn sys_remove_despawned_endpoints_from_vector_graphic(
    mut e_endpoints_removed: RemovedComponents<Endpoint>,
    q_endpoints: Query<&Parent>,
    mut q_vector_graphic: Query<&mut VectorGraphic, Without<Endpoint>>,
) {
    for entity in e_endpoints_removed.read() {
        let Ok(parent) = q_endpoints.get(entity) else {
            continue;
        };
        let Ok(mut vg) = q_vector_graphic.get_mut(parent.get()) else {
            continue;
        };
        vg.endpoints.remove(&entity);
    }
}

/// removes any despawned edges from the hashset within the parent VectorGraphic
pub fn sys_remove_despawned_edges_from_vector_graphic(
    mut e_edges_removed: RemovedComponents<Edge>,
    q_edges: Query<&Parent>,
    mut q_vector_graphic: Query<&mut VectorGraphic, Without<Edge>>,
) {
    for entity in e_edges_removed.read() {
        let Ok(parent) = q_edges.get(entity) else {
            continue;
        };
        let Ok(mut vg) = q_vector_graphic.get_mut(parent.get()) else {
            continue;
        };
        vg.edges.remove(&entity);
    }
}

pub fn sys_check_vector_graphic_children_changed(
    q_changed_endpoint: Query<&Parent, Changed<Endpoint>>,
    q_changed_edge: Query<&Parent, Changed<Edge>>,
) -> Vec<Entity> {
    let mut changed = EntityHashSet::default();
    for parent in &q_changed_endpoint {
        changed.insert(parent.get());
    }
    for parent in &q_changed_edge {
        changed.insert(parent.get());
    }
    changed.iter().copied().collect()
}

fn traverse_endpoints(
    start_endpoint: Entity,
    q_endpoints: &mut QueryLens<&Endpoint>,
    q_edges: &mut QueryLens<&Edge>,
) -> Result<Vec<Entity>, QueryEntityError> {
    let mut endpoint_entity = start_endpoint;
    let mut endpoint = Some(*q_endpoints.query().get(start_endpoint)?);
    let mut forward_endpoints = vec![start_endpoint];

    // Traverse forward
    while endpoint.is_some() && endpoint_entity != start_endpoint {
        let Some(ep) = endpoint else {
            break;
        };
        let Some(edge) = ep.next_edge(q_edges) else {
            break;
        };
        let edge = edge?;
        let ep = edge.next_endpoint(q_endpoints)?;
        endpoint_entity = edge.next_endpoint_entity();
        forward_endpoints.push(edge.next_endpoint_entity());
        endpoint = Some(*ep);
    }

    let mut back_endpoints = vec![];
    let mut endpoint_entity = start_endpoint;
    let mut endpoint = Some(*q_endpoints.query().get(start_endpoint)?);

    // Traverse Backwards
    while endpoint.is_some() && endpoint_entity != start_endpoint {
        let Some(ep) = endpoint else {
            break;
        };
        let Some(edge) = ep.prev_edge(q_edges) else {
            break;
        };
        let edge = edge?;
        let ep = edge.prev_endpoint(q_endpoints)?;
        endpoint_entity = edge.prev_endpoint_entity();
        forward_endpoints.push(edge.prev_endpoint_entity());
        endpoint = Some(*ep);
    }
    // Reverse backwards endpoints so it's facing forward again
    back_endpoints.reverse();
    back_endpoints.extend(forward_endpoints);

    Ok(back_endpoints)
}

pub fn sys_rebuild_vector_graphic_paths(
    changed_vector_graphics: In<Vec<Entity>>,
    mut q_vector_graphic: Query<(&VectorGraphic, &mut VectorGraphicPathStorage)>,
    mut q_endpoints: Query<(Entity, &Endpoint, &EndpointPosition, &Transform, &Parent)>,
    mut q_edges: Query<(Entity, &Edge, &Parent)>,
) {
    for e_vg in changed_vector_graphics.iter() {
        let Ok((vg, mut vg_storage)) = q_vector_graphic.get_mut(*e_vg) else {
            panic!("Received {e_vg:?} as a changed vector graphic but it doesn't exist or doesn't have a `VectorGraphicPathStorage` component.")
        };



    }
}
