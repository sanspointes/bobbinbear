//! Lifecycle methods for handling when edges/endpoints are spawned/despawned.

use bevy::{
    ecs::{entity::EntityHashSet, query::QueryEntityError, system::QueryLens},
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
    utils::{hashbrown::HashSet, HashMap},
};

use lyon_tessellation::{
    path::Path, BuffersBuilder, FillTessellator, FillVertexConstructor, StrokeTessellator,
    StrokeVertexConstructor, VertexBuffers,
};

use crate::{
    components::{Edge, Endpoint, VectorGraphic, VectorGraphicPathStorage},
    lyon_components::{FillOptions, StrokeOptions},
    prelude::EdgeVariant,
    utils::ToPoint,
    SptsFillTessellator, SptsStrokeTessellator,
};

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
        *path_storage = VectorGraphicPathStorage::NeedsRecalculate;
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
        *path_storage = VectorGraphicPathStorage::NeedsRecalculate;
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
        *path_storage = VectorGraphicPathStorage::NeedsRecalculate;
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
        *path_storage = VectorGraphicPathStorage::NeedsRecalculate;
    }
}

#[allow(clippy::type_complexity)]
pub fn sys_check_vector_graphic_children_changed(
    q_changed_endpoint: Query<
        &Parent,
        (Or<(Changed<Endpoint>, Changed<Transform>)>, Without<Edge>),
    >,
    q_changed_edge: Query<&Parent, (Or<(Changed<Edge>, Changed<EdgeVariant>)>, Without<Endpoint>)>,
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
        let (_, mut path_storage) = q_vector_graphic.get_mut(vector_grapic_entity).unwrap();
        *path_storage = VectorGraphicPathStorage::NeedsRecalculate;
    }
}

/// Start at endpoint e3,
/// iter forward e4, e5, e6, end
/// iter back e2, e1, reverse it so it's e1, e2
/// Add forward to back so it's e1 ... e6

fn traverse_endpoints(
    start_endpoint: Entity,
    q_endpoints: &mut QueryLens<&Endpoint>,
    q_edges: &mut QueryLens<&Edge>,
) -> Result<Vec<Entity>, QueryEntityError> {
    println!("Start endpoint: {start_endpoint:?}.");
    let first = *q_endpoints.query().get(start_endpoint)?;

    let mut needs_reverse = true;
    let mut endpoints = vec![];

    if first.next_edge_entity().is_some() {
        let mut curr = first;
        endpoints.push(start_endpoint);
        loop {
            let Some(edge) = curr.next_edge(q_edges) else {
                break;
            };
            let edge = edge?;
            let endpoint_entity = edge.next_endpoint_entity();
            curr = *q_endpoints.query().get(endpoint_entity)?;
            endpoints.push(endpoint_entity);
            if endpoint_entity == start_endpoint {
                needs_reverse = false; // Loop complete
                break;
            }
        }
    }
    if first.prev_edge_entity().is_some() && needs_reverse {
        let mut curr = first;
        let mut reverse_endpoints = vec![];
        loop {
            let Some(edge) = curr.prev_edge(q_edges) else {
                break;
            };
            let edge = edge?;
            let endpoint_entity = edge.prev_endpoint_entity();
            if endpoint_entity == start_endpoint {
                break;
            }
            curr = *q_endpoints.query().get(endpoint_entity)?;
            reverse_endpoints.push(endpoint_entity);
        }

        reverse_endpoints.reverse();
        reverse_endpoints.extend(endpoints.iter());

        endpoints = reverse_endpoints;
    }

    Ok(endpoints)
}

/// Builds Vec<Vec<Entity>> of all endpoint paths that need to be regenerated.
///
/// * `changed_vector_graphics`: The VectorGraphic entities that have changes.
/// * `q_endpoints`:
/// * `q_edges`:
pub fn sys_collect_vector_graph_path_endpoints(
    mut q_vector_graphic: Query<(Entity, &VectorGraphic, &mut VectorGraphicPathStorage)>,
    mut q_endpoints: Query<(Entity, &Endpoint, &Parent, &Transform)>,
    mut q_edges: Query<(Entity, &Edge, &EdgeVariant, &Parent)>,
) {
    let changed_vector_graphics: Vec<_> = q_vector_graphic
        .iter()
        .filter_map(|(e, _, path_storage)| {
            if matches!(path_storage, VectorGraphicPathStorage::NeedsRecalculate) {
                Some(e)
            } else {
                None
            }
        })
        .collect();

    // Hashset storing endpoint entity(0) and their parent(1)
    let mut unvisited: HashSet<_> = q_endpoints
        .iter()
        .filter_map(|(entity, _, parent, _)| {
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

        let result: Vec<_> = endpoints
            .iter()
            .map(|e| (e, q_endpoints.get(*e).unwrap().1))
            .collect();
        println!("Endpoints: {result:?}");

        unvisited.remove(&(entity, parent));
        for e in endpoints.iter() {
            unvisited.remove(&(*e, parent));
        }

        if endpoints.len() <= 1 {
            // panic!("sys_mark_vector_graph_path_starts: Endpoints path too short({endpoints:?}).");
            continue;
        }

        let entry = vector_graphic_path_endpoints.entry(parent);
        let paths = entry.or_insert(vec![]);
        paths.push(endpoints);
    }

    // Build the paths
    for vector_grapic_entity in changed_vector_graphics {
        let paths = vector_graphic_path_endpoints
            .get(&vector_grapic_entity)
            .unwrap();
        println!("Paths: {paths:?}");

        let Ok((_, _, mut path_storage)) = q_vector_graphic.get_mut(vector_grapic_entity) else {
            warn!("sys_mark_vector_graph_path_starts: Tried to get VectorGraphicPathStorage for changed path but entity or component on entity does not exist.  Entity: {vector_grapic_entity:?}");
            continue;
        };

        let mut pb = Path::builder();

        for path in paths {
            if path.len() < 2 {
                continue;
            }

            let mut path_iter = path.iter();

            let e_first = *path_iter.next().unwrap(); // Safety `path.len() < 2` above
            let (_, endpoint, _, transform) = q_endpoints
                .get(e_first)
                .expect("Could not get endpoint.");

            pb.begin(transform.translation.xy().to_point());

            let mut curr_endpoint = *endpoint;
            let mut e_last = e_first;

            for e_endpoint in path_iter {
                let (edge_entity, edge, edge_variant, _) =
                    q_edges.get(curr_endpoint.next_edge_entity().unwrap()).unwrap();
                let (e_next_endpoint, next_endpoint, _, transform) =
                    q_endpoints.get(edge.next_endpoint_entity()).unwrap();

                let to_point = transform.translation.xy().to_point();

                match edge_variant {
                    EdgeVariant::Line => {
                        println!("-> line_to {e_next_endpoint:?} via edge {edge_entity:?} ({endpoint:?}, {to_point:?})");
                        pb.line_to(to_point);
                    }
                    EdgeVariant::Quadratic { ctrl1 } => {
                        println!("-> quadratic_bezier_to {e_next_endpoint:?} via edge {edge_entity:?} ({endpoint:?}, {to_point:?})");
                        pb.quadratic_bezier_to(ctrl1.to_point(), to_point);
                    }
                    EdgeVariant::Cubic { ctrl1, ctrl2 } => {
                        println!("-> cubic_bezier_to {e_next_endpoint:?} via edge {edge_entity:?} ({endpoint:?}, {to_point:?})");
                        pb.cubic_bezier_to(ctrl1.to_point(), ctrl2.to_point(), to_point);
                    }
                }

                curr_endpoint = *next_endpoint;
                e_last = *e_endpoint;
            }

            let is_closed = e_last == e_first;
            pb.end(is_closed);
        }

        let path = pb.build();
        println!("Built path for {vector_grapic_entity:?} {path:?}");
        *path_storage = VectorGraphicPathStorage::Calculated(path);
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
        let VectorGraphicPathStorage::Calculated(path) = path_storage else {
            continue;
        };
        let mut geometry = VertexBuffers::new();

        if let Some(stroke_options) = maybe_stroke_options {
            println!("Tessellating {stroke_options:?} on path {path:?}.");
            if let Err(reason) = stroke_tesellator.tessellate_path(
                path,
                &(*stroke_options).into(),
                &mut BuffersBuilder::new(&mut geometry, RemeshVertexConstructor),
            ) {
                warn!("sys_remesh_vector_graphic: Failed to tessellate stroke {reason:?}.");
                continue;
            }
        }

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

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );

        let VertexBuffers { vertices, indices } = geometry;
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

        println!("Mesh generated {mesh:?}");
        let handle = Mesh2dHandle::from(meshes.add(mesh));
        commands.entity(entity).insert(handle);
    }
}
