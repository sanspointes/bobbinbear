use bevy_ecs::{entity::Entity, world::World};
use bevy_hierarchy::Children;
use bevy_reflect::TypeRegistry;
use smallvec::SmallVec;
use thiserror::Error;

use crate::uid::Uid;

use super::{entity::{EntityFragmentNewError, EntityFragmentSpawnError}, EntityFragment};

#[derive(Debug, Clone, Error)]
pub enum HierarchyFragmentNewError {
    #[error("Could not create HierarchyFragment because there was an issue when processing an entity: {0}.")]
    Entity(EntityFragmentNewError)
}

impl From<EntityFragmentNewError> for HierarchyFragmentNewError {
    fn from(value: EntityFragmentNewError) -> Self {
        Self::Entity(value)
    }
}

#[derive(Debug, Clone, Error)]
pub enum HierarchyFragmentSpawnError {
    #[error("Could not spawn HierarchyFragment because there is no Entity in world at Entity id {entity:?}.")]
    NoMatchingEntity { entity: Entity },
    #[error("Could not spawn HierarchyFragment because there was an issue when spawning an entity.")]
    Entity(EntityFragmentSpawnError)
}

impl From<EntityFragmentSpawnError> for HierarchyFragmentSpawnError {
    fn from(value: EntityFragmentSpawnError) -> Self {
        Self::Entity(value)
    }
}

/// Stores an entire fragment of the scene heirarchy
///
/// * `entity_fragment`: The root entity
/// * `children`: Any children entities
#[derive(Debug, Clone)]
pub struct HierarchyFragment {
    entity_fragment: EntityFragment,
    children: Option<Vec<HierarchyFragment>>,
}

impl HierarchyFragment {
    pub fn new(entity_fragment: EntityFragment, children: Option<Vec<HierarchyFragment>>) -> Self {
        Self {
            entity_fragment,
            children,
        }
    }

    pub fn root_uid(&self) -> Uid {
        self.entity_fragment.uid()
    }

    pub fn entity_fragment(&self) -> &EntityFragment {
        &self.entity_fragment
    }

    fn resolve_children_of_entity(world: &mut World, type_registry: &TypeRegistry, entity: Entity) -> Result<Option<Vec<HierarchyFragment>>, HierarchyFragmentNewError> {
        let Some(children) = world.get::<Children>(entity) else {
            return Ok(None);
        };
        let children: SmallVec<[Entity; 8]> = children.iter().cloned().collect();

        let children: Result<Vec<HierarchyFragment>, HierarchyFragmentNewError> = children.iter().map(|child| HierarchyFragment::from_world_entity(world, type_registry, *child)).collect();

        Ok(Some(children?))
    }

    /// Creates a HierarchyFragment from a root entity and will collect and serialize any children
    ///
    /// * `world`: 
    /// * `type_registry`: 
    /// * `filter`: 
    /// * `uid`: 
    pub fn from_world_uid(world: &mut World, type_registry: &TypeRegistry, uid: Uid) -> Result<HierarchyFragment, HierarchyFragmentNewError> {
        let entity = uid.entity(world).unwrap();

        let entity_fragment = EntityFragment::from_world_entity(world, type_registry, entity)?;
        let children = Self::resolve_children_of_entity(world, type_registry, entity)?;

        Ok(Self::new(entity_fragment, children))
    }

    /// Creates a new HierarchyFragment from the hieararchy of a world entity.
    ///
    /// * `world`: World to copy from
    /// * `type_registry`: Type registry used to control what will be extracted from the world.
    /// * `filter`: 
    /// * `entity`: 
    pub fn from_world_entity(world: &mut World, type_registry: &TypeRegistry, entity: Entity) -> Result<HierarchyFragment, HierarchyFragmentNewError> {
        let entity_fragment = EntityFragment::from_world_entity(world, type_registry, entity)?;
        let children = Self::resolve_children_of_entity(world, type_registry, entity)?;

        Ok(Self::new(entity_fragment, children))
    }

    /// Spawns the entity in the world, recursively spawning children
    ///
    /// * `world`: World to spawn into
    /// * `type_registry`: Type registry used to construct this object 
    pub fn spawn_in_world(&self, world: &mut World, type_registry: &TypeRegistry) -> Result<Entity, HierarchyFragmentSpawnError> {
        let parent = self.entity_fragment.spawn_in_world(world, type_registry)?.id();

        if let Some(children) = &self.children {
            for child in children {
                child.spawn_in_world_with_parent_entity(world, type_registry, parent)?;
            }
        }

        Ok(parent)
    }

    /// Spawns the entity in the world with a parent entity, recursively spawns children
    ///
    /// * `world`: World to spawn into
    /// * `type_registry`: Type registry used when constructing this HierarchyFragment
    /// * `parent`: Parent to spawn as a child of
    pub fn spawn_in_world_with_parent_entity(&self, world: &mut World, type_registry: &TypeRegistry, parent: Entity) -> Result<Entity, HierarchyFragmentSpawnError> {
        let entity = self.entity_fragment.spawn_in_world_with_parent_entity(world, type_registry, parent)?.id();

        if let Some(children) = &self.children {
            for child in children {
                child.spawn_in_world_with_parent_entity(world, type_registry, entity)?;
            }
        }

        Ok(entity)
    }

    /// Spawns the entity in the world with a parent entity (lookup by Uid), recursively spawns children
    ///
    /// * `world`: World to spawn into
    /// * `type_registry`: Type registry used when constructing this HierarchyFragment
    /// * `parent`: Parent to spawn as a child of
    pub fn spawn_in_world_with_parent_uid(&self, world: &mut World, type_registry: &TypeRegistry, parent: Uid) -> Result<Entity, HierarchyFragmentSpawnError> {
        let parent = parent.entity(world).unwrap();
        self.spawn_in_world_with_parent_entity(world, type_registry, parent)

    }
}
