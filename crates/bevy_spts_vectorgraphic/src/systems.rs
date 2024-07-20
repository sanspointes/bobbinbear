//! Lifecycle methods for handling when edges/endpoints are spawned/despawned.

use core::panic;
use std::fmt::Debug;

// use bevy::{
//     ecs::{
//         entity::{EntityHashMap, EntityHashSet},
//         query::QueryEntityError,
//         system::QueryLens,
//     }, prelude::*, render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology}, sprite::Mesh2dHandle, utils::HashMap
// };

use bevy::{
    asset::Assets, ecs::{
        entity::{Entity, EntityHashMap, EntityHashSet}, query::{Added, Changed, Or, QueryEntityError, With, Without}, removal_detection::RemovedComponents, system::{Commands, Query, QueryLens, ResMut}
    }, hierarchy::Parent, log::warn, math::Vec3Swizzles, render::{mesh::{Indices, Mesh, PrimitiveTopology}, render_asset::RenderAssetUsages}, sprite::Mesh2dHandle, transform::components::Transform, utils::HashMap
};
use bevy_spts_uid::{Uid, UidRegistry, UidRegistryError};
use lyon_tessellation::{
    path::Path, BuffersBuilder, FillVertexConstructor, StrokeVertexConstructor, VertexBuffers,
};

use crate::{
    components::{Edge, Endpoint, VectorGraphic, VectorGraphicPathStorage},
    lyon_components::{FillOptions, StrokeOptions},
    prelude::{EdgeVariant, ATTRIBUTE_SHAPE_MIX},
    utils::ToPoint,
    SptsFillTessellator, SptsStrokeTessellator,
};

#[derive(Debug)]
pub enum VectorGraphicError {
    QueryEntity(QueryEntityError),
    UidRegistry(UidRegistryError),
}
impl From<QueryEntityError> for VectorGraphicError {
    fn from(value: QueryEntityError) -> Self {
        Self::QueryEntity(value)
    }
}
impl From<UidRegistryError> for VectorGraphicError {
    fn from(value: UidRegistryError) -> Self {
        Self::UidRegistry(value)
    }
}

/// Adds any added endpoints to the hashset within the parent VectorGraphic
/// Marks VectorGraphic as needing a remesh
pub fn sys_add_spawned_endpoints_to_vector_graphic(
    q_endpoints: Query<(Entity, &Parent), Added<Endpoint>>,
    mut q_vector_graphic: Query<
        (&mut VectorGraphic, &mut VectorGraphicPathStorage),
        Without<Endpoint>,
    >,
) {
    for (entity, parent) in &q_endpoints {
        let Ok((mut vg, mut path_storage)) = q_vector_graphic.get_mut(parent.get()) else {
            continue;
        };
        vg.edges.remove(&entity);
        path_storage.set_dirty();
    }
}

/// Adds any added edges to the hashset within the parent VectorGraphic
/// Marks VectorGraphic as needing a remesh
pub fn sys_add_spawned_edges_to_vector_graphic(
    q_edges: Query<(Entity, &Parent), Added<Edge>>,
    mut q_vector_graphic: Query<
        (&mut VectorGraphic, &mut VectorGraphicPathStorage),
        Without<Endpoint>,
    >,
) {
    for (entity, parent) in &q_edges {
        let Ok((mut vg, mut path_storage)) = q_vector_graphic.get_mut(parent.get()) else {
            continue;
        };
        vg.edges.remove(&entity);
        path_storage.set_dirty();
    }
}

/// removes any despawned endpoints from the hashset within the parent VectorGraphic
/// Marks VectorGraphic as needing a remesh
pub fn sys_remove_despawned_endpoints_from_vector_graphic(
    mut e_endpoints_removed: RemovedComponents<Endpoint>,
    q_endpoints: Query<&Parent>,
    mut q_vector_graphic: Query<
        (&mut VectorGraphic, &mut VectorGraphicPathStorage),
        Without<Endpoint>,
    >,
) {
    for entity in e_endpoints_removed.read() {
        let Ok(parent) = q_endpoints.get(entity) else {
            continue;
        };
        let Ok((mut vg, mut path_storage)) = q_vector_graphic.get_mut(parent.get()) else {
            continue;
        };
        vg.edges.remove(&entity);
        path_storage.set_dirty();
    }
}

/// removes any despawned edges from the hashset within the parent VectorGraphic
/// Marks VectorGraphic as needing a remesh
pub fn sys_remove_despawned_edges_from_vector_graphic(
    mut e_edges_removed: RemovedComponents<Edge>,
    q_edges: Query<&Parent>,
    mut q_vector_graphic: Query<
        (&mut VectorGraphic, &mut VectorGraphicPathStorage),
        Without<Endpoint>,
    >,
) {
    for entity in e_edges_removed.read() {
        let Ok(parent) = q_edges.get(entity) else {
            continue;
        };
        let Ok((mut vg, mut path_storage)) = q_vector_graphic.get_mut(parent.get()) else {
            continue;
        };
        vg.edges.remove(&entity);
        path_storage.set_dirty();
    }
}

#[allow(clippy::type_complexity)]
pub fn sys_check_vector_graphic_children_changed(
    q_changed_endpoint: Query<
        &Parent,
        (
            Or<(Changed<Endpoint>, Changed<Transform>)>,
            Without<Edge>,
            With<Endpoint>,
        ),
    >,
    q_changed_edge: Query<
        &Parent,
        (
            Or<(Changed<Edge>, Changed<EdgeVariant>)>,
            Without<Endpoint>,
            With<Edge>,
        ),
    >,
    mut q_vector_graphic: Query<
        (&mut VectorGraphic, &mut VectorGraphicPathStorage),
        Without<Endpoint>,
    >,
) {
    let mut changed = EntityHashSet::default();
    for parent in &q_changed_endpoint {
        changed.insert(parent.get());
    }
    for parent in &q_changed_edge {
        changed.insert(parent.get());
    }
    for vector_grapic_entity in changed {
        println!("Found Vector Graphic Entity {vector_grapic_entity:?}");
        let (_, mut path_storage) = q_vector_graphic.get_mut(vector_grapic_entity).unwrap();
        path_storage.set_dirty();
    }
}

pub enum TraverseEndpointsDirectedResult {
    Closed,
    DeadEnd,
}

/// Returns Vec<Entity> containing the endpoint/edge walk.
/// [Entity (Edge), Entity(Endpoint), ...]
/// WARN: Different from traverse_endpoints as it doesn't contain the start_endpoint in the output.
fn traverse_endpoints_directed(
    start_endpoint_uid: &Uid,
    first_edge_uid: &Uid,
    q_endpoints: &mut QueryLens<&Endpoint>,
    q_edges: &mut QueryLens<&Edge>,
    reg: &mut UidRegistry,
    out: &mut Vec<Entity>,
) -> Result<TraverseEndpointsDirectedResult, VectorGraphicError> {
    let q_endpoints = q_endpoints.query();
    let q_edges = q_edges.query();

    let start_endpoint_e = reg.get_entity(*start_endpoint_uid)?;
    let start_endpoint = *q_endpoints.get(start_endpoint_e)?;

    let mut behind_edge_uid = None;
    let mut curr_endpoint_uid = *start_endpoint_uid;
    let mut curr_endpoint = start_endpoint;
    loop {
        let maybe_next_edge_uid = match behind_edge_uid {
            Some(behind_edge_uid) => curr_endpoint.other_edge_uid(&behind_edge_uid),
            None => Some(first_edge_uid),
        };

        let Some(next_edge_uid) = maybe_next_edge_uid else {
            return Ok(TraverseEndpointsDirectedResult::DeadEnd);
        };
        let next_edge_e = reg.get_entity(*next_edge_uid)?;
        let next_edge = q_edges.get(next_edge_e)?;

        let Some(next_endpoint_uid) = next_edge.other_endpoint_uid(&curr_endpoint_uid) else {
            return Ok(TraverseEndpointsDirectedResult::DeadEnd);
        };
        let next_endpoint_e = reg.get_entity(next_endpoint_uid)?;
        let next_endpoint = *q_endpoints.get(next_endpoint_e)?;

        out.push(next_edge_e);
        out.push(next_endpoint_e);
        if next_endpoint_uid == *start_endpoint_uid {
            return Ok(TraverseEndpointsDirectedResult::Closed);
        }
        curr_endpoint_uid = next_endpoint_uid;
        behind_edge_uid = Some(*next_edge_uid);
        curr_endpoint = next_endpoint;
    }
}

/// Returns Vec<Entity> containing the endpoint/edge walk.
/// [ Entity (Endpoint),
///     (Entity (Edge), Entity (Endpoint))... Repeating
/// ]
/// WARN: Length is always >= 1 as start_endpoint is included in Vec.
fn traverse_endpoints(
    start_endpoint_uid: &Uid,
    q_endpoints: &mut QueryLens<&Endpoint>,
    q_edges: &mut QueryLens<&Edge>,
    reg: &mut UidRegistry,
) -> Result<Vec<Entity>, VectorGraphicError> {
    let start_endpoint_e = reg.get_entity(*start_endpoint_uid)?;
    let start_endpoint = *q_endpoints.query().get(start_endpoint_e)?;
    // warn!("traverse_endpoints: Starting at {first_e:?} ({start_endpoint})");

    let Some(forward_first_edge_uid) = start_endpoint
        .prev_edge_entity()
        .or(start_endpoint.next_edge_entity())
    else {
        return Ok(vec![]);
    };
    let maybe_backward_first_edge_uid = start_endpoint.other_edge_uid(&forward_first_edge_uid);

    let mut forward_walk = vec![start_endpoint_e];
    let result = traverse_endpoints_directed(
        start_endpoint_uid,
        &forward_first_edge_uid,
        q_endpoints,
        q_edges,
        reg,
        &mut forward_walk,
    )?;

    match (maybe_backward_first_edge_uid, result) {
        (Some(backward_back_edge), TraverseEndpointsDirectedResult::DeadEnd) => {
            let mut back_walk = vec![];
            traverse_endpoints_directed(
                start_endpoint_uid,
                backward_back_edge,
                q_endpoints,
                q_edges,
                reg,
                &mut back_walk,
            )?;

            back_walk.reverse();
            back_walk.extend(forward_walk);
            Ok(back_walk)
        }
        (Some(_), TraverseEndpointsDirectedResult::Closed) => Ok(forward_walk),
        (None, TraverseEndpointsDirectedResult::Closed) => {
            panic!("Impossible.  Can't have a closed loop without a back_edge on first endpoint.")
        }
        (None, TraverseEndpointsDirectedResult::DeadEnd) => Ok(forward_walk),
    }
}

/// Builds Vec<Vec<Entity>> of all endpoint paths that need to be regenerated.
///
/// * `changed_vector_graphics`: The VectorGraphic entities that have changes.
/// * `q_endpoints`:
/// * `q_edges`:
pub fn sys_collect_vector_graph_path_endpoints(
    mut q_vector_graphic: Query<(Entity, &VectorGraphic, &mut VectorGraphicPathStorage)>,
    mut q_endpoints: Query<(Entity, &Uid, &Endpoint, &Parent, &Transform)>,
    mut q_edges: Query<(Entity, &Edge, &EdgeVariant, &Parent)>,
    mut reg: ResMut<UidRegistry>,
) {
    let changed_vector_graphics: Vec<_> = q_vector_graphic
        .iter()
        .filter_map(|(e, _, path_storage)| {
            if path_storage.needs_recalculate() {
                Some(e)
            } else {
                None
            }
        })
        .collect();

    // Hashset storing endpoint entity(0) and their parent(1)
    let mut unvisited: EntityHashMap<_> = q_endpoints
        .iter()
        .filter_map(|(entity, uid, _, parent, _)| {
            if changed_vector_graphics.contains(&parent.get()) {
                Some((entity, (*uid, parent.get())))
            } else {
                None
            }
        })
        .collect();

    let mut vector_graphic_path_endpoints = HashMap::new();

    while let Some(entity) = unvisited.keys().copied().next() {
        let Some((uid, parent)) = unvisited.get(&entity).copied() else {
            continue;
        };
        // Collect endpoints in path
        let endpoints = traverse_endpoints(
            &uid,
            &mut q_endpoints.transmute_lens::<&Endpoint>(),
            &mut q_edges.transmute_lens::<&Edge>(),
            &mut reg,
        );

        let Ok(endpoints) = endpoints else {
            warn!("sys_collect_vector_graph_path_endpoints: Could not get endpoints of group.  Reason {endpoints:?}");
            break;
        };

        // let result: Vec<_> = endpoints
        //     .iter()
        //     .map(|e| (e, q_endpoints.get(*e).unwrap().1))
        //     .collect();

        unvisited.remove(&entity);
        for chunk in endpoints.as_slice().chunks(2) {
            let endpoint_e = unsafe { chunk.get_unchecked(0) };
            unvisited.remove(endpoint_e);
        }

        if endpoints.len() <= 1 {
            warn!("sys_collect_vector_graph_path_endpoints: Endpoints too short({endpoints:?}).");
            continue;
        }

        let entry = vector_graphic_path_endpoints.entry(parent);
        let paths = entry.or_insert(vec![]);
        paths.push(endpoints);
    }

    // Build the paths
    for vector_grapic_entity in changed_vector_graphics {
        let Some(paths_walks_2d) = vector_graphic_path_endpoints.get(&vector_grapic_entity) else {
            // warn!("sys_mark_vector_graph_path_starts: Tried to get endpoints of vector_grapic_entity({vector_grapic_entity:?}) but not in hashmap.");
            continue;
        };

        let Ok((_, _, mut path_storage)) = q_vector_graphic.get_mut(vector_grapic_entity) else {
            warn!("sys_collect_vector_graph_path_endpoints: Tried to get VectorGraphicPathStorage for changed path but entity or component on entity does not exist.  Entity: {vector_grapic_entity:?}");
            continue;
        };

        let mut pb = Path::builder();

        for path_walk in paths_walks_2d {
            if path_walk.len() <= 1 {
                warn!("sys_collect_vector_graph_path_endpoints: Endpoints path too short({path_walk:?}).");
                continue;
            }
            let first_endpoint_e = path_walk.first().unwrap();

            let (_, _, _, _, transform) = q_endpoints
                .get(*first_endpoint_e)
                .expect("Could not get endpoint.");
            pb.begin(transform.translation.xy().to_point());

            let remaining_walk_slice = &path_walk[1..];
            for chunk in remaining_walk_slice.chunks(2) {
                let Some(edge_e) = chunk.first() else {
                    panic!("sys_collect_vector_graph_path_endpoints: Couldn't get first endpoint, maybe the walk is malformed.");
                };
                let Some(endpoint_e) = chunk.get(1) else {
                    panic!("sys_collect_vector_graph_path_endpoints: Couldn't get first edge, maybe the walk is malformed.");
                };
                let (_, _, edge_variant, _) = q_edges.get(*edge_e).unwrap();
                let (_, _, _, _, transform) = q_endpoints.get(*endpoint_e).unwrap();

                let to_point = transform.translation.xy().to_point();
                match edge_variant {
                    EdgeVariant::Line => {
                        pb.line_to(to_point);
                    }
                    EdgeVariant::Quadratic { ctrl1 } => {
                        pb.quadratic_bezier_to(ctrl1.to_point(), to_point);
                    }
                    EdgeVariant::Cubic { ctrl1, ctrl2 } => {
                        pb.cubic_bezier_to(ctrl1.to_point(), ctrl2.to_point(), to_point);
                    }
                }
            }

            let is_closed = path_walk.last().unwrap() == first_endpoint_e;
            pb.end(is_closed);
        }

        let path = pb.build();
        path_storage.set_path(path);
    }
}

/** Stage 3 Remeshing **/

struct RemeshVertexConstructor;
impl FillVertexConstructor<RemeshVertex> for RemeshVertexConstructor {
    fn new_vertex(&mut self, vertex: lyon_tessellation::FillVertex) -> RemeshVertex {
        RemeshVertex {
            position: vertex.position().to_array(),
            normal: [0.0, 0.0],
        }
    }
}

impl StrokeVertexConstructor<RemeshVertex> for RemeshVertexConstructor {
    fn new_vertex(&mut self, vertex: lyon_tessellation::StrokeVertex) -> RemeshVertex {
        RemeshVertex {
            position: vertex.position().to_array(),
            normal: vertex.normal().to_array(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RemeshVertex {
    position: [f32; 2],
    normal: [f32; 2],
}

#[allow(clippy::type_complexity)]
pub fn sys_remesh_vector_graphic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    q_vector_graphic: Query<
        (
            Entity,
            &VectorGraphicPathStorage,
            Option<&StrokeOptions>,
            Option<&FillOptions>,
        ),
        Or<(
            Changed<VectorGraphicPathStorage>,
            Changed<StrokeOptions>,
            Changed<FillOptions>,
        )>,
    >,
    mut fill_tessellator: ResMut<SptsFillTessellator>,
    mut stroke_tesellator: ResMut<SptsStrokeTessellator>,
) {
    for (entity, path_storage, maybe_stroke_options, maybe_fill_options) in q_vector_graphic.iter()
    {
        let Some(path) = path_storage.path() else {
            continue;
        };
        let mut geometry = VertexBuffers::new();

        if let Some(fill_options) = maybe_fill_options {
            if let Err(reason) = fill_tessellator.tessellate_path(
                path,
                &(*fill_options).into(),
                &mut BuffersBuilder::new(&mut geometry, RemeshVertexConstructor),
            ) {
                warn!("sys_remesh_vector_graphic: Failed to tessellate fill {reason:?}.");
                continue;
            }
        }
        // Stores vertex attribtue of ATTRIBUTE_SHAPE_MIX
        let mut shape_mix_attr = vec![0.; geometry.vertices.len()];

        if let Some(stroke_options) = maybe_stroke_options {
            if let Err(reason) = stroke_tesellator.tessellate_path(
                path,
                &(*stroke_options).into(),
                &mut BuffersBuilder::new(&mut geometry, RemeshVertexConstructor),
            ) {
                warn!("sys_remesh_vector_graphic: Failed to tessellate stroke {reason:?}.");
                continue;
            }
        }

        shape_mix_attr.extend(vec![1.; geometry.vertices.len() - shape_mix_attr.len()]);

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );

        let VertexBuffers { vertices, indices } = geometry;

        mesh.insert_attribute(ATTRIBUTE_SHAPE_MIX, shape_mix_attr);

        mesh.insert_indices(Indices::U32(indices));
        let (positions, normals): (Vec<[f32; 3]>, Vec<[f32; 3]>) = vertices
            .into_iter()
            .map(|vert| {
                let position: [f32; 3] = [vert.position[0], vert.position[1], 0.0];
                let normal: [f32; 3] = [vert.normal[0], vert.normal[1], 0.0];
                (position, normal)
            })
            .unzip();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        let handle = Mesh2dHandle::from(meshes.add(mesh));
        commands.entity(entity).insert(handle);
    }
}
