use bevy::{
    core::Name,
    ecs::{
        entity::Entity,
        system::Resource,
        world::{FromWorld, Mut, World},
    },
    hierarchy::Parent,
    math::Vec2,
    render::{color::Color, view::Visibility},
    utils::thiserror::Error,
};
use bevy_mod_raycast::deferred::RaycastMesh;
use bevy_spts_changeset::{
    builder::ChangesetCommands, commands_ext::WorldChangesetExt, resource::ChangesetResource,
};
use bevy_spts_uid::{Uid, UidRegistry};
use bevy_spts_vectorgraphic::{
    components::{Edge, EdgeVariant, Endpoint, VectorGraphic, VectorGraphicPathStorage},
    lyon_components::StrokeOptions,
    material::StrokeColor,
    prelude::VectorGraphicChangesetExt,
};

use crate::{
    ecs::{InternalObject, ObjectBundle, ObjectType, Position},
    plugins::{selected::Selectable, undoredo::UndoRedoTag},
    utils::curve::{cubic_point_at, quadratic_point_at},
    views::{vector_edge::VectorEdgeVM, vector_endpoint::VectorEndpointVM},
};

#[derive(Error, Debug)]
pub enum SplitEdgeError {
    #[error("Provided entity {0:?} does not have Edge or EdgeVariant components.")]
    EntityNotEdge(Entity),
}

pub fn get_position_of_edge_at_t_value(
    world: &World,
    edge: &Edge,
    edge_variant: &EdgeVariant,
    t_value: f32,
) -> Vec2 {
    let uid_registry = world.resource::<UidRegistry>();
    let prev_endpoint_e = uid_registry.entity(edge.prev_endpoint_uid());
    let next_endpoint_e = uid_registry.entity(edge.next_endpoint_uid());
    let prev_pos = world.get::<Position>(prev_endpoint_e).unwrap();
    let next_pos = world.get::<Position>(next_endpoint_e).unwrap();

    match edge_variant {
        EdgeVariant::Line => prev_pos.lerp(**next_pos, t_value),
        EdgeVariant::Quadratic { ctrl1 } => {
            quadratic_point_at(**prev_pos, *ctrl1, **next_pos, t_value)
        }
        EdgeVariant::Cubic { ctrl1, ctrl2 } => {
            cubic_point_at(**prev_pos, *ctrl1, *ctrl2, **next_pos, t_value)
        }
    }
}

pub fn split_edge_at_t_value(
    world: &World,
    changeset: &mut ChangesetCommands,
    edge_e: Entity,
    t_value: f32,
) -> Result<(), SplitEdgeError> {
    let edge = world.get::<Edge>(edge_e).copied();
    let edge_variant = world.get::<EdgeVariant>(edge_e).copied();

    let (Some(edge), Some(edge_variant)) = (edge, edge_variant) else {
        return Err(SplitEdgeError::EntityNotEdge(edge_e));
    };
    let parent_e = world.get::<Parent>(edge_e).unwrap().get();
    let edge_uid = world.get::<Uid>(edge_e).unwrap();

    let uid_registry = world.resource::<UidRegistry>();
    let parent_uid = uid_registry.uid(parent_e);

    let split_position = get_position_of_edge_at_t_value(world, &edge, &edge_variant, t_value);

    changeset.despawn_edge(*edge_uid);

    let split_endpoint_uid = changeset
        .spawn((
            Name::from("Endpoint"),
            ObjectBundle::new(ObjectType::VectorEndpoint).with_position(split_position),
            VectorEndpointVM,
            Endpoint::default(),
        ))
        .set_parent(parent_uid)
        .uid();

    // TODO: handle quadratic / cubics using derivative methods in crate::utils::curve
    changeset
        .spawn_edge(
            EdgeVariant::Line,
            edge.prev_endpoint_uid(),
            split_endpoint_uid,
        )
        .insert((
            Name::from("Edge"),
            ObjectBundle::new(ObjectType::VectorEdge),
            VectorEdgeVM,
        ))
        .set_parent(parent_uid);
    changeset
        .spawn_edge(
            EdgeVariant::Line,
            split_endpoint_uid,
            edge.next_endpoint_uid(),
        )
        .insert((
            Name::from("Edge"),
            ObjectBundle::new(ObjectType::VectorEdge),
            VectorEdgeVM,
        ))
        .set_parent(parent_uid);

    Ok(())
}

#[derive(Resource, Debug, Clone)]
pub struct PenToolResource {
    pub preview: PenToolPreview,
}

impl PenToolResource {
    pub fn resource_scope<T>(
        world: &mut World,
        f: impl FnOnce(&mut World, Mut<PenToolResource>) -> T,
    ) -> T {
        world.resource_scope::<PenToolResource, T>(f)
    }
}

impl FromWorld for PenToolResource {
    fn from_world(world: &mut World) -> Self {
        Self {
            preview: PenToolPreview::from_world(world),
        }
    }
}

#[derive(Resource, Debug, Clone)]
/// PenToolPreview dummy endpoints and line for visualising the next endpoint/line before it's
/// committed to the VectorObject.
pub struct PenToolPreview {
    vector_object: Uid,
    endpoint_0: Uid,
    edge: Uid,
    endpoint_1: Uid,
}

impl PenToolPreview {
    pub fn set_endpoint_0_world_pos(&self, world: &mut World, world_position: Vec2) {
        let entity = self.endpoint_0.entity(world).unwrap();
        let mut position = world.get_mut::<Position>(entity).unwrap();
        position.0 = world_position;
    }
    pub fn set_endpoint_1_world_pos(&self, world: &mut World, world_position: Vec2) {
        let entity = self.endpoint_1.entity(world).unwrap();
        let mut position = world.get_mut::<Position>(entity).unwrap();
        position.0 = world_position;
    }

    pub fn hide_all(&self, world: &mut World) {
        let entities = [
            self.vector_object.entity(world).unwrap(),
            self.endpoint_0.entity(world).unwrap(),
            self.endpoint_1.entity(world).unwrap(),
            self.edge.entity(world).unwrap(),
        ];

        for entity in entities {
            let mut visibility = world.get_mut::<Visibility>(entity).unwrap();
            *visibility = Visibility::Hidden;
        }
    }

    pub fn show_all(&self, world: &mut World) {
        let entities = [
            self.vector_object.entity(world).unwrap(),
            self.endpoint_0.entity(world).unwrap(),
            self.endpoint_1.entity(world).unwrap(),
            self.edge.entity(world).unwrap(),
        ];

        for entity in entities {
            let mut visibility = world.get_mut::<Visibility>(entity).unwrap();
            *visibility = Visibility::Visible;
        }
    }

    pub fn show_only_endpoint_0(&self, world: &mut World) {
        self.hide_all(world);

        let entity = self.endpoint_0.entity(world).unwrap();
        let mut visiblity = world.get_mut::<Visibility>(entity).unwrap();
        *visiblity = Visibility::Visible;
    }

    pub fn update_to_line(&self, world: &mut World) {
        let entity = self.edge.entity(world).unwrap();
        let mut edge_variant = world.get_mut::<EdgeVariant>(entity).unwrap();
        *edge_variant = EdgeVariant::Line;
    }
}

impl FromWorld for PenToolPreview {
    fn from_world(world: &mut World) -> Self {
        let mut builder = world.changeset();

        let vector_object = builder
            .spawn((
                Name::from("PenToolPreview_VectorObject"),
                ObjectBundle::new(ObjectType::Vector),
                VectorGraphic::default(),
                VectorGraphicPathStorage::default(),
                StrokeOptions::default(),
                StrokeColor(Color::WHITE),
                InternalObject,
            ))
            .remove::<RaycastMesh<Selectable>>()
            .apply(Selectable::Locked)
            .apply(Visibility::Hidden)
            .uid();

        let endpoint_0 = builder
            .spawn((
                Name::from("Endpoint0"),
                ObjectBundle::new(ObjectType::VectorEndpoint),
                Endpoint::default(),
                InternalObject,
            ))
            .remove::<RaycastMesh<Selectable>>()
            .apply(Selectable::Locked)
            .apply(Visibility::Hidden)
            .set_parent(vector_object)
            .uid();

        let endpoint_1 = builder
            .spawn((
                Name::from("Endpoint1"),
                ObjectBundle::new(ObjectType::VectorEndpoint),
                Endpoint::default(),
                InternalObject,
            ))
            .remove::<RaycastMesh<Selectable>>()
            .apply(Selectable::Locked)
            .apply(Visibility::Hidden)
            .set_parent(vector_object)
            .uid();

        let edge = builder
            .spawn_edge(EdgeVariant::Line, endpoint_0, endpoint_1)
            .insert((
                Name::from("Edge"),
                ObjectBundle::new(ObjectType::VectorEdge),
                InternalObject,
            ))
            .remove::<RaycastMesh<Selectable>>()
            .apply(Selectable::Locked)
            .apply(Visibility::Hidden)
            // .insert(ObjectBundle::new(ObjectType::VectorSegment))
            .set_parent(vector_object)
            .uid();

        let changeset = builder.build();
        ChangesetResource::<UndoRedoTag>::context_scope(world, |world, cx| {
            changeset
                .apply(world, cx)
                .expect("Error creating PenToolPreview.");
        });

        world
            .entity_mut(endpoint_0.entity(world).unwrap())
            .insert(VectorEndpointVM);
        world
            .entity_mut(endpoint_1.entity(world).unwrap())
            .insert(VectorEndpointVM);
        world
            .entity_mut(edge.entity(world).unwrap())
            .insert(VectorEdgeVM);

        Self {
            vector_object,
            endpoint_0,
            endpoint_1,
            edge,
        }
    }
}
