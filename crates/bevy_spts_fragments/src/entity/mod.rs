use bevy_ecs::{
    entity::Entity,
    world::World,
};
use bevy_hierarchy::BuildWorldChildren;
use bevy_reflect::TypeRegistry;
use bevy_scene::SceneFilter;
use bevy_spts_uid::UidRegistry;
use thiserror::Error;

// #[cfg(feature = "serde")]
// use serde::{Serialize, Deserialize};

use crate::prelude::*;

#[derive(Debug, Clone, Error)]
pub enum EntityFragmentNewError {
    #[error("Could not create EntityFragment from Uid because no entity in world with Uid {uid}.")]
    NoMatchingUid { uid: Uid },
    #[error("Could not create EntityFragment from Entity because no entity in scene at Entity id {entity:?}.")]
    NoMatchingEntity { entity: Entity },
}

#[derive(Debug, Clone, Error)]
pub enum EntityFragmentSpawnError {
    #[error("Could not spawn EntityFragment because no entity in scene with Uid {uid}.")]
    NoMatchingUid { uid: Uid },
    #[error("Could not spawn EntityFragment because there was an error when inserting its components: {0}.")]
    Component(ComponentFragmentError),
}

impl From<ComponentFragmentError> for EntityFragmentSpawnError {
    fn from(value: ComponentFragmentError) -> Self {
        Self::Component(value)
    }
}

#[derive(Debug, Clone)]
pub struct EntityFragment {
    uid: Uid,
    bundle: BundleFragment,
}

impl EntityFragment {
    pub fn new(uid: Uid, bundle: BundleFragment) -> Self {
        Self { uid, bundle }
    }

    pub fn uid(&self) -> Uid {
        self.uid
    }

    fn from_world(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
        entity: Entity,
    ) -> Result<Self, EntityFragmentNewError> {
        let bundle = BundleFragment::from_world(world, type_registry, filter, entity)?;
        Ok(EntityFragment::new(uid, bundle))
    }

    pub fn from_world_uid(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
    ) -> Result<Self, EntityFragmentNewError> {
        let entity = uid
            .entity(world)
            .ok_or(EntityFragmentNewError::NoMatchingUid { uid })?;
        Self::from_world(world, type_registry, filter, uid, entity)
    }

    pub fn from_world_entity(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        entity: Entity,
    ) -> Result<Self, EntityFragmentNewError> {
        let uid = *world
            .get::<Uid>(entity)
            .ok_or(EntityFragmentNewError::NoMatchingEntity { entity })?;
        Self::from_world(world, type_registry, filter, uid, entity)
    }

    fn despawn_from_world(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
        entity: Entity,
    ) -> Result<Self, EntityFragmentNewError> {
        let bundle = BundleFragment::from_world(world, type_registry, filter, entity)?;
        world.despawn(entity);
        world.resource_mut::<UidRegistry>().unregister(uid);
        Ok(EntityFragment::new(uid, bundle))
    }

    pub fn despawn_from_world_uid(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
    ) -> Result<Self, EntityFragmentNewError> {
        let entity = uid
            .entity(world)
            .ok_or(EntityFragmentNewError::NoMatchingUid { uid })?;
        Self::despawn_from_world(world, type_registry, filter, uid, entity)
    }

    pub fn despawn_from_world_entity(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        entity: Entity,
    ) -> Result<Self, EntityFragmentNewError> {
        let uid = *world
            .get::<Uid>(entity)
            .ok_or(EntityFragmentNewError::NoMatchingEntity { entity })?;
        Self::despawn_from_world(world, type_registry, filter, uid, entity)
    }

    pub fn spawn_in_world<'ewm, 'w: 'ewm>(
        &self,
        world: &'w mut World,
        type_registry: &TypeRegistry,
    ) -> Result<Entity, EntityFragmentSpawnError> {
        let mut entity_mut = world.spawn(self.uid);

        self.bundle.insert(&mut entity_mut, type_registry)?;

        let id = entity_mut.id();
        world.resource_mut::<UidRegistry>().register(self.uid, id);
        Ok(id)
    }

    pub fn spawn_in_world_with_parent_entity(
        &self,
        world: &mut World,
        type_registry: &TypeRegistry,
        parent: Entity,
    ) -> Result<Entity, EntityFragmentSpawnError> {
        let id = self.spawn_in_world(world, type_registry)?;
        let mut entity_mut = world.entity_mut(id);
        entity_mut.set_parent(parent);
        Ok(id)
    }

    pub fn spawn_in_world_with_parent_uid(
        &self,
        world: &mut World,
        type_registry: &TypeRegistry,
        parent: Uid,
    ) -> Result<Entity, EntityFragmentSpawnError> {
        let parent = parent
            .entity(world)
            .ok_or(EntityFragmentSpawnError::NoMatchingUid { uid: parent })?;
        self.spawn_in_world_with_parent_entity(world, type_registry, parent)
    }
}
