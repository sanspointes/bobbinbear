use bevy_ecs::{entity::Entity, world::{EntityWorldMut, World}};
use bevy_reflect::TypeRegistry;
use bevy_scene::SceneFilter;
use bevy_spts_uid::UidRegistry;

mod errors;
pub use errors::*;

use crate::{component::ComponentReflectError, prelude::*};

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
    ) -> Result<Self, EntityFromWorldError> {
        let bundle = BundleFragment::from_entity(world, type_registry, filter, entity)?;
        Ok(EntityFragment::new(uid, bundle))
    }

    pub fn from_world_uid(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
    ) -> Result<Self, EntityFromWorldError> {
        let entity = world.resource::<UidRegistry>().get_entity(uid)?;
        Self::from_world(world, type_registry, filter, uid, entity)
    }

    pub fn from_world_entity(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        entity: Entity,
    ) -> Result<Self, EntityFromWorldError> {
        let uid = world.resource::<UidRegistry>().get_uid(entity)?;
        Self::from_world(world, type_registry, filter, uid, entity)
    }

    fn despawn_from_world(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
        entity: Entity,
    ) -> Result<Self, EntityFromWorldError> {
        let bundle = BundleFragment::from_entity(world, type_registry, filter, entity)?;
        world.despawn(entity);
        world.resource_mut::<UidRegistry>().unregister(uid);
        Ok(EntityFragment::new(uid, bundle))
    }

    pub fn despawn_from_world_uid(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
    ) -> Result<Self, EntityFromWorldError> {
        let entity = world.resource::<UidRegistry>().get_entity(uid)?;
        Self::despawn_from_world(world, type_registry, filter, uid, entity)
    }

    pub fn despawn_from_world_entity(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        entity: Entity,
    ) -> Result<Self, EntityFromWorldError> {
        let uid = world.resource::<UidRegistry>().get_uid(entity)?;
        Self::despawn_from_world(world, type_registry, filter, uid, entity)
    }

    pub fn spawn_in_world<'ewm, 'w: 'ewm>(
        &self,
        world: &'w mut World,
        type_registry: &TypeRegistry,
    ) -> Result<EntityWorldMut<'ewm>, ComponentReflectError> {
        let mut entity_mut = world.spawn(self.uid);
        self.bundle.insert(&mut entity_mut, type_registry)?;

        let id = entity_mut.id();
        world.resource_mut::<UidRegistry>().register(self.uid, id);
        Ok(world.entity_mut(id))
    }
}
