use anyhow::anyhow;
use bevy::prelude::*;

use bevy_spts_changeset::{
    builder::{Changeset, ChangesetCommands, MultiChangesetBuilder},
    resource::ChangesetResource,
};
use bevy_spts_uid::{Uid, UidRegistry};
use bevy_spts_vectorgraphic::{
    components::{Edge, EdgeVariant, Endpoint},
    prelude::VectorGraphicChangesetExt,
};

use crate::{
    ecs::{
        InternalObject, ObjectBundle, ObjectType, Position, ProxiedPosition,
        ProxiedPositionStrategy, ProxiedUid,
    },
    plugins::{
        model_view::{Model, View},
        selected::{ProxiedHovered, ProxiedSelected, ProxiedVisibility},
        undoredo::{UndoRedoApi, UndoRedoTag},
    },
    utils::{
        curve::{cubic_point_at, quadratic_point_at},
        safe_world_ext::BBSafeWorldExt,
    },
    views::vector_edge::VectorEdgeVM,
};

use super::{PenToolBuildingFromEndpointTag, PenToolBuildingVectorObjectTag};

// TODO: These composable utility methods for mutating the world should probably be moved into
// their own layer so they can be shared between tools.  I.e. if we need to split a path in select
// mode on right click.

pub struct PenToolActions {
    parent_vector_graphic: Option<Uid>,
    from_endpoint: Option<Uid>,
    pub multi_commands: MultiChangesetBuilder,
}

impl PenToolActions {
    pub fn new(world: &mut World) -> Self {
        let parent_vector_graphic = get_current_building_vector_object(world);
        let from_endpoint = get_current_building_prev_endpoint(world);

        Self {
            parent_vector_graphic,
            from_endpoint,
            multi_commands: MultiChangesetBuilder::default(),
        }
    }

    pub fn changeset_scope<U>(
        &mut self,
        world: &mut World,
        scope: impl FnOnce(&mut World, &mut ChangesetCommands) -> U,
    ) -> anyhow::Result<U> {
        let value = self.multi_commands.changeset_scope::<U>(world, scope);
        ChangesetResource::<UndoRedoTag>::context_scope::<anyhow::Result<()>>(
            world,
            |world, cx| self.multi_commands.apply(world, cx),
        )?;
        Ok(value)
    }

    pub fn finish(self, world: &mut World) -> anyhow::Result<()> {
        let changeset = ChangesetResource::<UndoRedoTag>::context_scope::<anyhow::Result<Changeset>>(
            world,
            |world, cx| self.multi_commands.apply_and_build(world, cx),
        )?;
        UndoRedoApi::push_already_applied(world, changeset);
        Ok(())
    }
    /// Given an entity of a view or model, returns the model entity.
    fn resolve_model<C: Component>(
        &self,
        world: &mut World,
        entity: Entity,
    ) -> anyhow::Result<Entity> {
        if world.get::<C>(entity).is_some() {
            Ok(entity)
        } else if let Some(view) = world.get::<View<C>>(entity) {
            Ok(view.model().entity())
        } else {
            Err(anyhow!("Entity ({entity:?}) is neither view nor model."))
        }
    }

    /// Given an entity of a view or model, returns the view entity.
    fn resolve_view<C: Component>(
        &self,
        world: &mut World,
        entity: Entity,
    ) -> anyhow::Result<Entity> {
        if world.get::<View<C>>(entity).is_some() {
            Ok(entity)
        } else if let Some(model) = world.get::<Model<C>>(entity) {
            Ok(model.view().entity())
        } else {
            Err(anyhow!("Entity ({entity:?}) is neither view nor model."))
        }
    }

    pub fn spawn_new_endpoint(&mut self, world: &mut World, position: Vec2) -> anyhow::Result<Uid> {
        let parent_vector_graphic = self
            .parent_vector_graphic
            .ok_or_else(|| anyhow!("No parent vector graphic."))?;

        self.changeset_scope(world, |_, commands| {
            let uid = commands
                .spawn((
                    Name::from("Endpoint"),
                    ObjectBundle::new(ObjectType::VectorEndpoint).with_position(position),
                    Endpoint::default(),
                    InternalObject,
                ))
                .set_parent(parent_vector_graphic)
                .uid();
            Ok(uid)
        })?
    }

    fn get_position_of_edge_at_t_value(
        &self,
        world: &mut World,
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

    pub fn split_edge(
        &mut self,
        world: &mut World,
        edge_uid: Uid,
        t_value: f32,
    ) -> anyhow::Result<Uid> {
        let edge_e = edge_uid
            .entity(&world)
            .ok_or_else(|| anyhow!("No edge entity in world?"))?;
        let edge = world
            .get::<Edge>(edge_e)
            .copied()
            .ok_or_else(|| anyhow!("No Edge component on provided entity {edge_e:?}."))?;
        let edge_variant = world
            .get::<EdgeVariant>(edge_e)
            .copied()
            .ok_or_else(|| anyhow!("No EdgeVariant component on provided entity {edge_e:?}."))?;

        let parent_e = world
            .get::<Parent>(edge_e)
            .ok_or_else(|| anyhow!("No Parent component on provided entity {edge_e:?}."))?
            .get();

        let uid_registry = world.resource::<UidRegistry>();
        let parent_uid = uid_registry.uid(parent_e);

        let split_position =
            self.get_position_of_edge_at_t_value(world, &edge, &edge_variant, t_value);
        let split_endpoint_uid = self.spawn_new_endpoint(world, split_position)?;

        self.changeset_scope(world, |_, commands| {
            commands.despawn_edge(edge_uid);

            // TODO: handle quadratic / cubics using derivative methods in crate::utils::curve
            let _edge_0 = commands
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
            let _edge_1 = commands
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

            Ok(split_endpoint_uid)
        })?
    }

    fn spawn_merged_endpoint_if_slots_full(
        &mut self,
        world: &mut World,
        endpoint_uid: Uid,
    ) -> anyhow::Result<Option<Uid>> {
        let endpoint_e = endpoint_uid
            .entity(world)
            .ok_or_else(|| anyhow!("No entity for endpoint {endpoint_uid}."))?;
        let endpoint = *world.bb_get::<Endpoint>(endpoint_e)?;
        let needs_new_endpoint = matches!(
            (endpoint.prev_edge_entity(), endpoint.next_edge_entity()),
            (Some(_), Some(_))
        );
        if needs_new_endpoint {
            let position = world
                .get::<Position>(endpoint_e)
                .ok_or_else(|| anyhow!("Endpoint to copy doesn't have position."))?;
            let new_endpoint_uid = self.spawn_new_endpoint(world, **position)?;
            self.multi_commands.changeset_scope(world, |_, commands| {
                commands
                    .entity(new_endpoint_uid)
                    .insert(ProxiedPosition::new(
                        endpoint_uid,
                        ProxiedPositionStrategy::Local,
                    ))
                    .insert(ProxiedSelected::new(endpoint_uid, ()))
                    .insert(ProxiedHovered::new(endpoint_uid, ()))
                    .insert(ProxiedUid::new(endpoint_uid, ()))
                    .insert(ProxiedVisibility::new(endpoint_uid, ()));
            });
            Ok(Some(new_endpoint_uid))
        } else {
            Ok(None)
        }
    }

    pub fn spawn_edge_to_endpoint(
        &mut self,
        world: &mut World,
        to_endpoint_uid: Uid,
    ) -> anyhow::Result<(Uid, Option<Uid>)> {
        let parent_vector_graphic = self
            .parent_vector_graphic
            .ok_or_else(|| anyhow!("No parent vector graphic."))?;

        let from_endpoint_uid = self
            .from_endpoint
            .ok_or_else(|| anyhow!("Not build from and endpoint."))?;
        let from_endpoint_uid = self
            .spawn_merged_endpoint_if_slots_full(world, from_endpoint_uid)?
            .unwrap_or(from_endpoint_uid);

        let new_to_endpoint_uid =
            self.spawn_merged_endpoint_if_slots_full(world, to_endpoint_uid)?;
        let to_endpoint_uid = new_to_endpoint_uid.unwrap_or(to_endpoint_uid);

        let endpoint_e = to_endpoint_uid.entity(world).unwrap();
        let endpoint = *world.get::<Endpoint>(endpoint_e).unwrap();

        self.changeset_scope(world, |_, commands| {
            let mut new_edge_commands =
                match (endpoint.prev_edge_entity(), endpoint.next_edge_entity()) {
                    // SAFETY: Checked above by `needs_new_endpoint`
                    (Some(_), Some(_)) => unreachable!(),
                    (None, None) | (None, Some(_)) => {
                        commands.spawn_edge(EdgeVariant::Line, from_endpoint_uid, to_endpoint_uid)
                    }
                    (Some(_), None) => {
                        commands.spawn_edge(EdgeVariant::Line, to_endpoint_uid, from_endpoint_uid)
                    }
                };

            let uid = new_edge_commands
                .insert((
                    Name::from("Edge"),
                    ObjectBundle::new(ObjectType::VectorEdge),
                    VectorEdgeVM,
                    InternalObject,
                ))
                // .insert(ObjectBundle::new(ObjectType::VectorSegment))
                .set_parent(parent_vector_graphic)
                .uid();

            Ok((uid, new_to_endpoint_uid))
        })?
    }

    pub fn set_building_from_endpoint_tag(
        &mut self,
        world: &mut World,
        endpoint: Uid,
    ) -> anyhow::Result<()> {
        self.clear_building_from_endpoint_tag(world)?;

        self.changeset_scope(world, |_, commands| {
            commands
                .entity(endpoint)
                .insert(PenToolBuildingFromEndpointTag);
            Ok(())
        })?
    }
    pub fn clear_building_from_endpoint_tag(&mut self, world: &mut World) -> anyhow::Result<()> {
        let prev = world
            .query_filtered::<&Uid, With<PenToolBuildingFromEndpointTag>>()
            .get_single(world)
            .copied();

        self.changeset_scope(world, |_, commands| {
            if let Ok(prev_endpoint) = prev {
                commands
                    .entity(prev_endpoint)
                    .remove::<PenToolBuildingFromEndpointTag>();
            }
            Ok(())
        })?
    }
}

// /// Builds an edge from
// pub fn build_edge_to_endpoint(world: &World, endpoint: Uid) -> anyhow::Result<()> {
//     let endpoint_e = endpoint
//         .entity(world)
//         .ok_or(anyhow!("Can't get entity for endpoint."))?;
//     let endpoint = world.bb_get::<Endpoint>(endpoint_e)?;
//
//     let mut commands = world.changeset();
//
//     commands.spawn_edge(edge_variant, prev_endpoint, next_endpoint)()
// }
//
// HELPERS !!!

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
