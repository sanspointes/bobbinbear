use bevy::{
    core::Name, ecs::{entity::Entity, world::World}, hierarchy::Parent, utils::thiserror::Error
};
use bevy_spts_changeset::builder::ChangesetCommands;
use bevy_spts_uid::{Uid, UidRegistry};
use bevy_spts_vectorgraphic::{components::{Edge, EdgeVariant, Endpoint}, prelude::VectorGraphicChangesetExt};

use crate::{ecs::{InternalObject, ObjectBundle, ObjectType, Position}, utils::curve::{cubic_point_at, quadratic_point_at}, views::{vector_edge::VectorEdgeVM, vector_endpoint::VectorEndpointVM}};

#[derive(Error, Debug)]
pub enum SplitEdgeError {
    #[error("Provided entity {0:?} does not have Edge or EdgeVariant components.")]
    EntityNotEdge(Entity),
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
    let prev_endpoint_e = uid_registry.entity(edge.prev_endpoint_uid());
    let next_endpoint_e = uid_registry.entity(edge.next_endpoint_uid());
    let prev_pos = world.get::<Position>(prev_endpoint_e).unwrap();
    let next_pos = world.get::<Position>(next_endpoint_e).unwrap();

    let split_position = match edge_variant {
        EdgeVariant::Line => prev_pos.lerp(**next_pos, t_value),
        EdgeVariant::Quadratic { ctrl1 } => {
            quadratic_point_at(**prev_pos, ctrl1, **next_pos, t_value)
        }
        EdgeVariant::Cubic { ctrl1, ctrl2 } => {
            cubic_point_at(**prev_pos, ctrl1, ctrl2, **next_pos, t_value)
        }
    };

    changeset.entity(*edge_uid).despawn();
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
    changeset.spawn_edge(EdgeVariant::Line, edge.prev_endpoint_uid(), split_endpoint_uid).insert((
        Name::from("Edge"),
        ObjectBundle::new(ObjectType::VectorEdge),
        VectorEdgeVM,
    )).set_parent(parent_uid);
    changeset.spawn_edge(EdgeVariant::Line, split_endpoint_uid, edge.next_endpoint_uid()).insert((
        Name::from("Edge"),
        ObjectBundle::new(ObjectType::VectorEdge),
        VectorEdgeVM,
    )).set_parent(parent_uid);

    Ok(())
}
