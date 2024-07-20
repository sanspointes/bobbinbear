//! # Pen Utilities

use bevy::{
    asset::{Assets, Handle}, color::Color, core::Name, ecs::{entity::Entity, query::With, world::World}, hierarchy::Parent, math::Vec2
};
use thiserror::Error;
use bevy_spts_changeset::builder::ChangesetCommands;
use bevy_spts_uid::{Uid, UidRegistry};
use bevy_spts_vectorgraphic::{
    components::{Edge, EdgeVariant, Endpoint, VectorGraphic, VectorGraphicPathStorage},
    lyon_components::{FillOptions, StrokeOptions},
    material::{FillColor, StrokeColor, VectorGraphicMaterial},
    prelude::VectorGraphicChangesetExt,
};

use crate::{
    ecs::{InternalObject, ObjectBundle, ObjectType, Position},
    utils::curve::{cubic_point_at, quadratic_point_at},
    views::{vector_edge::VectorEdgeVM, vector_endpoint::VectorEndpointVM},
};

use super::{PenToolBuildingFromEndpointTag, PenToolBuildingVectorObjectTag};

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
    commands: &mut ChangesetCommands,
    edge_e: Entity,
    t_value: f32,
) -> Result<(Uid, Uid, Uid), SplitEdgeError> {
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

    commands.despawn_edge(*edge_uid);

    let split_endpoint_uid = commands
        .spawn((
            Name::from("Endpoint"),
            ObjectBundle::new(ObjectType::VectorEndpoint).with_position(split_position),
            VectorEndpointVM,
            Endpoint::default(),
        ))
        .set_parent(parent_uid)
        .uid();

    // TODO: handle quadratic / cubics using derivative methods in crate::utils::curve
    let edge_0 = commands
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
        .set_parent(parent_uid)
        .uid();
    let edge_1 = commands
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
        .set_parent(parent_uid)
        .uid();

    Ok((split_endpoint_uid, edge_0, edge_1))
}

pub(super) fn get_new_vector_graphic_material(world: &mut World) -> Handle<VectorGraphicMaterial> {
    let mut materials = world.resource_mut::<Assets<VectorGraphicMaterial>>();
    materials.add(VectorGraphicMaterial::default())
}

pub(super) fn build_default_vector_graphic(
    builder: &mut ChangesetCommands,
    material: Handle<VectorGraphicMaterial>,
) -> Uid {
    let vector_graphic = builder
        .spawn((
            Name::from("Shape"),
            ObjectBundle::new(ObjectType::Vector),
            VectorGraphic::default(),
            VectorGraphicPathStorage::default(),
            StrokeOptions::default().with_line_width(5.),
            StrokeColor(Color::BLACK),
            FillOptions::default(),
            FillColor(Color::srgba(0.5, 0.5, 0.5, 0.5)),
            material,
            PenToolBuildingVectorObjectTag,
        ))
        .uid();
    vector_graphic
}

/// Strategy when creating a new edge
pub(super) enum BuildEndpointAndEdgeTarget {
    NewEndpoint { world_pos: Vec2 },
    ExistingLinkPrevious(Uid),
    ExistingLinkNext(Uid),
}

pub(super) struct BuildEndpointAndEdgeOptions {
    pub parent_uid: Uid,
    pub from_endpoint: Uid,
    pub edge_variant: EdgeVariant,
}

pub(super) fn build_next_endpoint(
    builder: &mut ChangesetCommands,
    position: Vec2,
    parent: Uid,
) -> Uid {
    builder
        .spawn((
            Name::from("Endpoint"),
            ObjectBundle::new(ObjectType::VectorEndpoint).with_position(position),
            Endpoint::default(),
            VectorEndpointVM,
            InternalObject,
            PenToolBuildingFromEndpointTag,
        ))
        .set_parent(parent)
        .uid()
}

/// Gets the Uid of the vector object currently being build
pub(super) fn get_current_building_vector_object(world: &mut World) -> Option<Uid> {
    let mut q_building_vector_object =
        world.query_filtered::<&Uid, With<PenToolBuildingVectorObjectTag>>();
    q_building_vector_object.get_single(world).ok().copied()
}

/// Gets the Uid of the previous endpoint/endpoint we're building from.
pub(super) fn get_current_building_prev_endpoint(world: &mut World) -> Option<Uid> {
    let mut q_building_endpoint =
        world.query_filtered::<&Uid, With<PenToolBuildingFromEndpointTag>>();
    q_building_endpoint.get_single(world).ok().copied()
}
