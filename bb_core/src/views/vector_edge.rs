use bevy::{
    asset::Assets,
    core::Name,
    ecs::{
        component::Component,
        entity::Entity,
        event::Events,
        query::{Changed, With},
        reflect::ReflectComponent,
        system::{Commands, Query, Res},
        world::World,
    },
    hierarchy::BuildWorldChildren,
    log::warn,
    reflect::Reflect,
    render::{
        mesh::{Indices, Mesh, MeshVertexAttribute},
        render_asset::RenderAssetUsages,
        render_resource::VertexFormat,
    },
    sprite::Mesh2dHandle,
    utils::hashbrown::HashSet,
};
use bevy_mod_raycast::markers::SimplifiedMesh;
use bevy_spts_uid::{Uid, UidRegistry};
use bevy_spts_vectorgraphic::{
    components::{EdgeVariant, Endpoint},
    lyon_path::builder::{Build, PathBuilder},
    lyon_tessellation::{
        BuffersBuilder, StrokeOptions, StrokeTessellator, StrokeVertex, StrokeVertexConstructor,
        VertexBuffers,
    },
    prelude::Edge,
};
use moonshine_core::{kind::Instance, object::Object};

use crate::{
    ecs::{InternalObject, ObjectBundle, ObjectType, Position, ProxiedObjectBundle},
    materials::{UiElementMaterialCache, ATTRIBUTE_THEME_BASE, ATTRIBUTE_THEME_BASE_OPACITY, ATTRIBUTE_THEME_MIX},
    plugins::{
        effect::Effect,
        model_view::{BuildView, Model, View, ViewBuilder},
        viewport::BobbinViewportResource,
    },
};

/// Attribute contains T value of edge (0-1) how far a vert is from start -> end.
pub const ATTRIBUTE_EDGE_T: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_EdgeT", 3340, VertexFormat::Float32);

#[repr(C)]
#[derive(Copy, Clone)]
struct RemeshVertex {
    position: [f32; 3],
    normal: [f32; 3],
    edge_t: f32,
}
struct RemeshVertexConstructor;
impl StrokeVertexConstructor<RemeshVertex> for RemeshVertexConstructor {
    fn new_vertex(&mut self, mut vertex: StrokeVertex) -> RemeshVertex {
        let pos = vertex.position();
        let norm = vertex.normal();
        let attrs = vertex.interpolated_attributes();
        RemeshVertex {
            position: [pos.x, pos.y, 0.],
            normal: [norm.x, norm.y, 0.],
            edge_t: attrs[0],
        }
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
/// View/Model marker struct for view_model plugin.  When this is present it will generates views
/// for the Object::VectorEndpoint entity.
pub struct VectorEdgeVM;

impl BuildView<VectorEdgeVM> for VectorEdgeVM {
    fn build(world: &World, object: Object<VectorEdgeVM>, view: &mut ViewBuilder<VectorEdgeVM>) {
        warn!("Spawning view for edge. {object:?}");
        // Build the proxied viewport object
        let material = world.resource::<UiElementMaterialCache>().default.clone();
        let endpoint_uid = world.resource::<UidRegistry>().uid(object.entity());
        let uid = Uid::default();
        view.insert((
            Name::from("VectorEdge (View)"),
            ObjectBundle::new(ObjectType::VectorEdge).with_z_position(9.),
            ProxiedObjectBundle::new(endpoint_uid),
            InternalObject,
            material,
            uid,
        ));
        let viewport_entity = world.resource::<BobbinViewportResource>().viewport_entity();
        let view_entity = view.entity();

        view.commands().commands().add(move |world: &mut World| {
            world.entity_mut(view_entity).set_parent(viewport_entity);
        });
        view.commands().commands().add(move |world: &mut World| {
            world
                .resource_mut::<Events<Effect>>()
                .send(Effect::EntitiesSpawned(vec![uid]));
        });
        view.commands().commands().add(move |world: &mut World| {
            world
                .resource_mut::<UidRegistry>()
                .register(uid, view_entity);
        });

        update_vector_edge_mesh(
            world,
            object.entity(),
            view_entity,
            &mut view.commands().commands(),
        );
    }

    fn on_before_destroy(
        world: &World,
        _model: Instance<VectorEdgeVM>,
        view: Instance<View<VectorEdgeVM>>,
        commands: &mut Commands,
    ) {
        let view_uid = *world.get::<Uid>(view.entity()).unwrap();
        commands.add(move |world: &mut World| {
            world
                .resource_mut::<Events<Effect>>()
                .send(Effect::EntitiesDespawned(vec![view_uid]));
        });
        commands.add(move |world: &mut World| {
            world.resource_mut::<UidRegistry>().unregister(view_uid);
        });
    }
}

fn update_vector_edge_mesh(
    world: &World,
    model_entity: Entity,
    view_entity: Entity,
    commands: &mut Commands,
) {
    // Collect data for building the mesh
    let edge = world.get::<Edge>(model_entity).unwrap();
    let edge_pos = world.get::<Position>(model_entity).unwrap();
    let edge_variant = world.get::<EdgeVariant>(model_entity).unwrap();
    let uid_registry = world.resource::<UidRegistry>();
    let e_prev_endpoint = uid_registry.entity(edge.prev_endpoint_uid());
    let e_next_endpoint = uid_registry.entity(edge.next_endpoint_uid());
    let prev_endpoint_pos = *world.get::<Position>(e_prev_endpoint).unwrap();
    let next_endpoint_pos = *world.get::<Position>(e_next_endpoint).unwrap();

    // Build the path for the edge
    let (mesh, aabb) = {
        let mut stroke_tesselator = StrokeTessellator::default();
        let mut geometry: VertexBuffers<RemeshVertex, u32> = VertexBuffers::new();

        let stroke_options = StrokeOptions::default().with_line_width(2.);
        let mut buffers_builder = BuffersBuilder::new(&mut geometry, RemeshVertexConstructor);
        let mut pb =
            stroke_tesselator.builder_with_attributes(1, &stroke_options, &mut buffers_builder);
        pb.begin(Position(prev_endpoint_pos.0 - edge_pos.0).into(), &[0.]);
        match edge_variant {
            EdgeVariant::Line => {
                pb.line_to(Position(next_endpoint_pos.0 - edge_pos.0).into(), &[1.0]);
            }
            EdgeVariant::Quadratic { ctrl1 } => {
                pb.quadratic_bezier_to(
                    Position(*ctrl1 - edge_pos.0).into(),
                    Position(next_endpoint_pos.0 - edge_pos.0).into(),
                    &[1.0],
                );
            }
            EdgeVariant::Cubic { ctrl1, ctrl2 } => {
                pb.cubic_bezier_to(
                    Position(*ctrl1 - edge_pos.0).into(),
                    Position(*ctrl2 - edge_pos.0).into(),
                    Position(next_endpoint_pos.0 - edge_pos.0).into(),
                    &[1.0],
                );
            }
        }
        pb.end(false);
        if let Err(reason) = pb.build() {
            warn!("BuildView<VectorEdgeVM>::build() -> Failed to tesselate edge: {reason:?}");
        }

        let mut mesh = Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );

        let VertexBuffers { vertices, indices } = geometry;
        mesh.insert_indices(Indices::U32(indices));
        mesh.insert_attribute(ATTRIBUTE_THEME_MIX, vec![0.; vertices.len()]);
        mesh.insert_attribute(ATTRIBUTE_THEME_BASE, vec![0.75; vertices.len()]);
        mesh.insert_attribute(ATTRIBUTE_THEME_BASE_OPACITY, vec![1.; vertices.len()]);

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertices.len());
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices.len());
        let mut edge_t_attr: Vec<f32> = Vec::with_capacity(vertices.len());

        for RemeshVertex {
            position,
            normal,
            edge_t,
        } in vertices
        {
            positions.push([position[0], -position[1], position[2]]);
            normals.push([normal[0], -normal[1], normal[2]]);
            edge_t_attr.push(edge_t);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(ATTRIBUTE_EDGE_T, edge_t_attr);

        // Compute AABB
        let aabb = mesh.compute_aabb();
        if aabb.is_none() {
            // warn!("Generated edge mesh could not generate an Aabb box. {mesh:?}");
        }
        (mesh, aabb)
    };

    // Build the path for the edge
    let simplified_mesh = {
        let mut stroke_tesselator = StrokeTessellator::default();
        let mut geometry: VertexBuffers<RemeshVertex, u32> = VertexBuffers::new();

        let stroke_options = StrokeOptions::default().with_line_width(10.);
        let mut buffers_builder = BuffersBuilder::new(&mut geometry, RemeshVertexConstructor);
        let mut pb =
            stroke_tesselator.builder_with_attributes(1, &stroke_options, &mut buffers_builder);
        pb.begin(Position(prev_endpoint_pos.0 - edge_pos.0).into(), &[0.]);
        match edge_variant {
            EdgeVariant::Line => {
                pb.line_to(Position(next_endpoint_pos.0 - edge_pos.0).into(), &[1.0]);
            }
            EdgeVariant::Quadratic { ctrl1 } => {
                pb.quadratic_bezier_to(
                    Position(*ctrl1 - edge_pos.0).into(),
                    Position(next_endpoint_pos.0 - edge_pos.0).into(),
                    &[1.0],
                );
            }
            EdgeVariant::Cubic { ctrl1, ctrl2 } => {
                pb.cubic_bezier_to(
                    Position(*ctrl1 - edge_pos.0).into(),
                    Position(*ctrl2 - edge_pos.0).into(),
                    Position(next_endpoint_pos.0 - edge_pos.0).into(),
                    &[1.0],
                );
            }
        }
        pb.end(false);
        if let Err(reason) = pb.build() {
            warn!("BuildView<VectorEdgeVM>::build() -> Failed to tesselate edge: {reason:?}");
        }

        let mut mesh = Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );

        let VertexBuffers { vertices, indices } = geometry;
        mesh.insert_indices(Indices::U32(indices));

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertices.len());
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices.len());
        let mut edge_t_attr: Vec<f32> = Vec::with_capacity(vertices.len());

        for RemeshVertex {
            position,
            normal,
            edge_t,
        } in vertices
        {
            positions.push([position[0], -position[1], position[2]]);
            normals.push([normal[0], -normal[1], normal[2]]);
            edge_t_attr.push(edge_t);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(ATTRIBUTE_EDGE_T, edge_t_attr);
        mesh
    };

    // Build the mesh and push asset to world
    commands.add(move |world: &mut World| {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let simplified_mesh = SimplifiedMesh { mesh: meshes.add(simplified_mesh) };
        let handle = Mesh2dHandle(meshes.add(mesh));

        let mut view_entity_mut = world.entity_mut(view_entity);
        view_entity_mut.insert(handle);
        view_entity_mut.insert(simplified_mesh);
        if let Some(aabb) = aabb {
            view_entity_mut.insert(aabb);
        }
    });
}

/// Updates the connected views/vector_edge.rs (VectorEdgeVM) mesh whenever an
/// endpoints position is updated.
///
/// * `world`:
/// * `r_uid_registry`:
/// * `q_moved_endpoints`:
/// * `q_edge`:
/// * `commands`:
pub fn sys_update_vector_edge_vm_mesh_when_endpoint_move(
    world: &World,
    r_uid_registry: Res<UidRegistry>,
    q_moved_endpoints: Query<&Endpoint, Changed<Position>>,
    q_edge: Query<(Entity, &Model<VectorEdgeVM>), With<Edge>>,
    mut commands: Commands,
) {
    let mut updated_edges: HashSet<Entity> = HashSet::new();

    for moved_endpoint in q_moved_endpoints.iter() {
        let next_edge = moved_endpoint
            .next_edge_entity()
            .map(|uid| r_uid_registry.entity(uid))
            .and_then(|e| q_edge.get(e).ok());
        let next_stale_edge = next_edge.filter(|(e, _)| !updated_edges.contains(e));
        if let Some((model_entity, model)) = next_stale_edge {
            updated_edges.insert(model_entity);
            update_vector_edge_mesh(world, model_entity, model.view().entity(), &mut commands);
        }
        let prev_edge = moved_endpoint
            .prev_edge_entity()
            .map(|uid| r_uid_registry.entity(uid))
            .and_then(|e| q_edge.get(e).ok());
        let prev_stale_edge = prev_edge.filter(|(e, _)| !updated_edges.contains(e));
        if let Some((model_entity, model)) = prev_stale_edge {
            updated_edges.insert(model_entity);
            update_vector_edge_mesh(world, model_entity, model.view().entity(), &mut commands);
        }
    }
}
