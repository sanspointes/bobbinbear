//! Lifecycle methods for handling when edges/endpoints are spawned/despawned.

use bevy_ecs::{
    entity::{Entity, EntityHashSet},
    query::{Added, Changed, Or, QueryEntityError, With, Without},
    removal_detection::RemovedComponents,
    system::{In, Query, QueryLens},
};
use bevy_hierarchy::Parent;
use bevy_transform::components::Transform;
use bevy_utils::{tracing::warn, HashMap, HashSet};

use crate::{components::{
    Edge, Endpoint, VectorGraphic, VectorGraphicPathStorage,
}, prelude::EdgeVariant};

/// Adds any added endpoints to the hashset within the parent VectorGraphic
/// Marks VectorGraphic as needing a remesh
pub fn sys_add_spawned_endpoints_to_vector_graphic(
    q_endpoints: Query<(Entity, &Parent), Added<Endpoint>>,
    mut q_vector_graphic: Query<&mut VectorGraphic, Without<Endpoint>>,
) {
    for (entity, parent) in &q_endpoints {
        let Ok(mut vg) = q_vector_graphic.get_mut(parent.get()) else {
            continue;
        };
        vg.endpoints.insert(entity);
        vg.needs_redraw = true;
    }
}

/// Adds any added edges to the hashset within the parent VectorGraphic
/// Marks VectorGraphic as needing a remesh
pub fn sys_add_spawned_edges_to_vector_graphic(
    q_edges: Query<(Entity, &Parent), Added<Edge>>,
    mut q_vector_graphic: Query<&mut VectorGraphic, Without<Edge>>,
) {
    for (entity, parent) in &q_edges {
        let Ok(mut vg) = q_vector_graphic.get_mut(parent.get()) else {
            continue;
        };
        vg.edges.insert(entity);
        vg.needs_redraw = true;
    }
}

/// removes any despawned endpoints from the hashset within the parent VectorGraphic
/// Marks VectorGraphic as needing a remesh
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
        vg.edges.insert(entity);
    }
}

/// removes any despawned edges from the hashset within the parent VectorGraphic
/// Marks VectorGraphic as needing a remesh
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
        vg.needs_redraw = true;
    }
}

#[allow(clippy::type_complexity)]
pub fn sys_check_vector_graphic_children_changed(
    q_changed_endpoint: Query<&Parent, (Or<(Changed<Endpoint>, Changed<Transform>)>, Without<Edge>)>,
    q_changed_edge: Query<&Parent, (Or<(Changed<Edge>, Changed<EdgeVariant>)>, Without<Endpoint>)>,
    mut q_vector_graphic: Query<&mut VectorGraphic, Without<Edge>>,
) {
    let mut changed = EntityHashSet::default();
    for parent in &q_changed_endpoint {
        changed.insert(parent.get());
    }
    for parent in &q_changed_edge {
        changed.insert(parent.get());
    }
    for vector_grapic_entity in changed {
        q_vector_graphic.get_mut(vector_grapic_entity).unwrap().needs_redraw = true;
    }
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
        endpoint = Some(ep);
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
        endpoint = Some(ep);
    }
    // Reverse backwards endpoints so it's facing forward again
    back_endpoints.reverse();
    back_endpoints.extend(forward_endpoints);

    Ok(back_endpoints)
}

/// Builds Vec<Vec<Entity>> of all endpoint paths that need to be regenerated.
///
/// * `changed_vector_graphics`: The VectorGraphic entities that have changes.
/// * `q_endpoints`:
/// * `q_edges`:
pub fn sys_collect_vector_graph_path_endpoints(
    mut q_vector_graphic: Query<(Entity, &VectorGraphic, &mut VectorGraphicPathStorage)>,
    mut q_endpoints: Query<(Entity, &Endpoint, &Parent)>,
    mut q_edges: Query<(Entity, &Edge, &Parent)>,
) -> HashMap<Entity, Vec<Vec<Entity>>> {
    let changed_vector_graphics: Vec<_> = q_vector_graphic.iter().filter_map(|(e, vg, _)| {
        if vg.needs_redraw {
            Some(e)
        } else {
            None
        }
    }).collect();
    // Clear the path storage for regeneration
    for entity in changed_vector_graphics.iter() {
        let Ok((_, _, mut path_storage)) = q_vector_graphic.get_mut(*entity) else {
            warn!("sys_mark_vector_graph_path_starts: Tried to get VectorGraphicPathStorage for changed path but entity or component on entity does not exist.  Entity: {entity:?}");
            continue;
        };
        path_storage.clear();
    }

    // Hashset storing endpoint entity(0) and their parent(1)
    let mut unvisited: HashSet<_> = q_endpoints
        .iter()
        .filter_map(|(entity, _, parent)| {
            if changed_vector_graphics.contains(&parent.get()) {
                Some((entity, parent.get()))
            } else {
                None
            }
        })
        .collect();

    let mut vector_graphic_path_endpoints = HashMap::new();

    while let Some((entity, parent)) = unvisited.iter().copied().next() {
        // Collect endpoints in path
        let endpoints = traverse_endpoints(
            entity,
            &mut q_endpoints.transmute_lens::<&Endpoint>(),
            &mut q_edges.transmute_lens::<&Edge>(),
        );
        let Ok(endpoints) = endpoints else {
            warn!("sys_mark_vector_graph_path_starts: Could not get endpoints of group.  Reason {endpoints:?}");
            continue;
        };

        if endpoints.len() <= 1 {
            warn!("sys_mark_vector_graph_path_starts: Endpoints path too short({endpoints:?}).");
            continue;
        }

        for e in endpoints.iter() {
            unvisited.remove(&(*e, parent));
        }

        let entry = vector_graphic_path_endpoints.entry(parent);
        let paths = entry.or_insert(vec![]);
        paths.push(endpoints);
    }

    vector_graphic_path_endpoints
}

pub fn sys_build_vector_graph_paths(
    In(vector_graphic_path_endpoints): In<HashMap<Entity, Vec<Vec<Endpoint>>>>,
    mut q_vector_graphic: Query<(&VectorGraphic, &mut VectorGraphicPathStorage)>,
    mut q_endpoints: Query<(Entity, &Endpoint, &Parent)>,
    mut q_edges: Query<(Entity, &Edge, &Parent)>,
) {

}
