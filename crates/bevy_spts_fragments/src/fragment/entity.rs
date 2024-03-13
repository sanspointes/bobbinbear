use bevy_ecs::{
    entity::Entity,
    world::{EntityWorldMut, World},
};
use bevy_hierarchy::BuildWorldChildren;
use bevy_reflect::TypeRegistry;
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
                .and_then(|id| ComponentFragment::from_type_id(type_registry, &entity_ref, id));

            if let Some(cf) = component_fragment {
                components.push(cf);
            }
        }

        Ok(components)
    }

    pub fn from_world_uid(
        world: &mut World,
        type_registry: &TypeRegistry,
        uid: Uid,
    ) -> Result<Self, EntityFragmentNewError> {
        let entity = uid
            .entity(world)
            .ok_or(EntityFragmentNewError::NoMatchingUid { uid })?;
        let components = EntityFragment::components_from_entity(world, type_registry, entity)?;
        Ok(EntityFragment::new(uid, components))
    }

    pub fn from_world_entity(
        world: &mut World,
        type_registry: &TypeRegistry,
        entity: Entity,
    ) -> Result<Self, EntityFragmentNewError> {
        let uid = *world
            .get::<Uid>(entity)
            .ok_or(EntityFragmentNewError::NoMatchingEntity { entity })?;
        let components = EntityFragment::components_from_entity(world, type_registry, entity)?;
        Ok(EntityFragment::new(uid, components))
    }

    pub fn spawn_in_world<'w>(
        &self,
        world: &'w mut World,
        type_registry: &TypeRegistry,
    ) -> Result<EntityWorldMut<'w>, EntityFragmentSpawnError> {
        let mut entity_mut = world.spawn(self.uid);

        for comp in &self.components {
            comp.insert_to_entity_world_mut(type_registry, &mut entity_mut)?;
        }

        Ok(entity_mut)
    }

    pub fn spawn_in_world_with_parent_entity<'w>(
        &self,
        world: &'w mut World,
        type_registry: &TypeRegistry,
        parent: Entity,
    ) -> Result<EntityWorldMut<'w>, EntityFragmentSpawnError> {
        let mut entity_mut = self.spawn_in_world(world, type_registry)?;
        entity_mut.set_parent(parent);
        Ok(entity_mut)
    }

    pub fn spawn_in_world_with_parent_uid<'w>(
        &self,
        world: &'w mut World,
        type_registry: &TypeRegistry,
        parent: Uid,
    ) -> Result<EntityWorldMut<'w>, EntityFragmentSpawnError> {
        let parent = parent
            .entity(world)
            .ok_or(EntityFragmentSpawnError::NoMatchingUid { uid: parent })?;
        self.spawn_in_world_with_parent_entity(world, type_registry, parent)
    }
}
