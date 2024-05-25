use bevy::{
    asset::Assets, core::Name, ecs::{
        component::Component, event::Events, reflect::ReflectComponent, system::Commands,
        world::World,
    }, hierarchy::BuildWorldChildren, log::warn, reflect::{List, Reflect}, render::{mesh::{Indices, Mesh}, render_asset::RenderAssetUsages}, sprite::{Mesh2d, Mesh2dHandle}
};
use bevy_spts_uid::{Uid, UidRegistry};
use bevy_spts_vectorgraphic::{
    components::EdgeVariant,
    lyon_path::{math::Point, Path},
    lyon_tessellation::{BuffersBuilder, StrokeOptions, StrokeVertex, StrokeVertexConstructor, VertexBuffers},
    prelude::Edge,
    SptsStrokeTessellator,
};
use moonshine_core::{kind::Instance, object::Object};

use crate::{
    ecs::{InternalObject, ObjectBundle, ObjectType, Position, ProxiedObjectBundle}, materials::{UiElementMaterialCache, ATTRIBUTE_THEME_MIX}, plugins::{
        effect::Effect,
        model_view::{BuildView, View, ViewBuilder}, viewport::BobbinViewportResource,
    }
};

#[repr(C)]
#[derive(Copy, Clone)]
struct RemeshVertex {
    position: [f32; 3],
    normal: [f32; 3],
}
struct RemeshVertexConstructor;
impl StrokeVertexConstructor<RemeshVertex> for RemeshVertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> RemeshVertex {
        let pos = vertex.position();
        let norm = vertex.normal();
        RemeshVertex {
            position: [pos.x, pos.y, 0.],
            normal: [norm.x, norm.y, 0.],
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
        // Build the proxied viewport object
        let material = world.resource::<UiElementMaterialCache>().default.clone();
        let endpoint_uid = world.resource::<UidRegistry>().uid(object.entity());
        let uid = Uid::default();
        view.insert((
            Name::from("VectorEdge (View)"),
            ObjectBundle::new(ObjectType::VectorEdge).with_z_position(10.),
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
            world.resource_mut::<UidRegistry>().register(uid, view_entity);
        });

        // Collect data for building the mesh
        let edge = world.get::<Edge>(object.entity()).unwrap();
        let edge_variant = world.get::<EdgeVariant>(object.entity()).unwrap();
        let uid_registry = world.resource::<UidRegistry>();
        let e_prev_endpoint = uid_registry.entity(edge.prev_endpoint_uid());
        let e_next_endpoint = uid_registry.entity(edge.next_endpoint_uid());
        let prev_endpoint_pos = *world.get::<Position>(e_prev_endpoint).unwrap();
        let next_endpoint_pos = *world.get::<Position>(e_next_endpoint).unwrap();

        // Build the path for the edge
        let mut pb = Path::builder();
        pb.begin(prev_endpoint_pos.into());
        match edge_variant {
            EdgeVariant::Line => {
                pb.line_to(next_endpoint_pos.into());
            }
            EdgeVariant::Quadratic { ctrl1 } => {
                pb.quadratic_bezier_to(Point::new(ctrl1.x, ctrl1.y), next_endpoint_pos.into());
            }
            EdgeVariant::Cubic { ctrl1, ctrl2 } => {
                pb.cubic_bezier_to(
                    Point::new(ctrl1.x, ctrl1.y),
                    Point::new(ctrl2.x, ctrl2.y),
                    next_endpoint_pos.into(),
                );
            }
        }
        pb.end(false);
        let path = pb.build();

        // Build the mesh and push asset to world
        view.commands().commands().add(move |world: &mut World| {
            let mut stroke_tesselator = world.resource_mut::<SptsStrokeTessellator>();
            let mut geometry: VertexBuffers<RemeshVertex, u32> = VertexBuffers::new();

            if let Err(reason) = stroke_tesselator.tessellate_path(
                &path,
                &StrokeOptions::default().with_line_width(3.),
                &mut BuffersBuilder::new(&mut geometry, RemeshVertexConstructor),
            ) {
                warn!("BuildView<VectorEdgeVM>::build() -> Failed to tesselate edge: {reason:?}");
            }
            let mut theme_mix_attr = vec![0.; geometry.vertices.len()];

            let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);

            let VertexBuffers { vertices, indices } = geometry;
            mesh.insert_indices(Indices::U32(indices));

            let (positions, normals): (Vec<[f32; 3]>, Vec<[f32; 3]>) = vertices
                .into_iter()
                .map(|vert| {
                    let RemeshVertex { position, normal } = vert;
                    (
                        [position[0], -position[1], position[2]],
                        [normal[0], -normal[1], normal[2]],
                    )
                })
                .unzip();

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            mesh.insert_attribute(ATTRIBUTE_THEME_MIX, theme_mix_attr);

            warn!("Edge mesh {mesh:?}");

            let mut meshes = world.resource_mut::<Assets<Mesh>>();
            let handle = Mesh2dHandle::from(meshes.add(mesh));
            world.entity_mut(view_entity).insert(handle);
        });
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
