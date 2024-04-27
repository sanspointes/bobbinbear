use bevy_ecs::{
    entity::Entity, system::ResMut, world::{EntityWorldMut, Mut, World}
};
use bevy_hierarchy::BuildWorldChildren;
use bevy_reflect::TypeRegistry;
use bevy_scene::SceneFilter;
use bevy_spts_uid::UidRegistry;
use thiserror::Error;

// #[cfg(feature = "serde")]
// use serde::{Serialize, Deserialize};

use crate::prelude::Uid;

use super::component::{ComponentFragment, ComponentFragmentError};

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
    components: Vec<ComponentFragment>,
}

impl EntityFragment {
    pub fn new(uid: Uid, components: Vec<ComponentFragment>) -> Self {
        Self { uid, components }
    }

    pub fn uid(&self) -> Uid {
        self.uid
    }

    pub(crate) fn components_from_entity(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        entity: Entity,
    ) -> Result<Vec<ComponentFragment>, EntityFragmentNewError> {
        let entity_ref = world
            .get_entity(entity)
            .ok_or(EntityFragmentNewError::NoMatchingEntity { entity })?;
        let mut components = vec![];

        for comp_id in entity_ref.archetype().components() {
            let component_fragment = world
                .components()
                .get_info(comp_id)
                .and_then(|c| c.type_id())
                .and_then(|id| {
                    if filter.is_allowed_by_id(id) {
                        Some(id)
                    } else {
                        None
                    }
                })
                .and_then(|id| ComponentFragment::from_type_id(type_registry, &entity_ref, id));

            if let Some(cf) = component_fragment {
                components.push(cf);
            }
        }

        Ok(components)
    }

    fn from_world(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
        entity: Entity,
    ) -> Result<Self, EntityFragmentNewError> {
        let components = EntityFragment::components_from_entity(world, type_registry, filter, entity)?;
        Ok(EntityFragment::new(uid, components))
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

    pub fn despawn_from_world_uid(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
    ) -> Result<Self, EntityFragmentNewError> {
        let fragment = Self::from_world_uid(world, type_registry, filter, uid)?;
        world.resource_mut::<UidRegistry>().unregister(fragment.uid);
        Ok(fragment)
    }

    pub fn despawn_from_world_entity(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        entity: Entity,
    ) -> Result<Self, EntityFragmentNewError> {
        let fragment = Self::from_world_entity(world, type_registry, filter, entity)?;
        world.resource_mut::<UidRegistry>().unregister(fragment.uid);
        Ok(fragment)
    }

    pub fn spawn_in_world<'ewm, 'w: 'ewm>(
        &self,
        world: &'w mut World,
        type_registry: &TypeRegistry,
    ) -> Result<Entity, EntityFragmentSpawnError> {
        let mut entity_mut = world.spawn(self.uid);

        for comp in &self.components {
            comp.insert_to_entity_world_mut(type_registry, &mut entity_mut)?;
        }

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

