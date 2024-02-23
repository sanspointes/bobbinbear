use bevy_ecs::{entity::Entity, world::World};
use bevy_hierarchy::Children;
use bevy_reflect::TypeRegistry;
use bevy_scene::SceneFilter;
use smallvec::SmallVec;

use crate::uid::Uid;

use super::EntityFragment;

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

    /// Creates a HierarchyFragment from a root entity and will collect and serialize any children
    ///
    /// * `world`: 
    /// * `type_registry`: 
    /// * `filter`: 
    /// * `uid`: 
    pub fn from_world_uid(world: &mut World, type_registry: &TypeRegistry, filter: &SceneFilter, uid: Uid) -> HierarchyFragment {
        let entity = uid.entity(world).unwrap();

        let entity_fragment = EntityFragment::from_world_entity(world, type_registry, filter, entity);

        let children: Option<SmallVec<[Entity; 8]>> = world.get::<Children>(entity).map(|children| children.into_iter().cloned().collect());
        let children: Option<Vec<HierarchyFragment>> = children.map(|children| {
            children.into_iter().map(|child| {
                HierarchyFragment::from_world_entity(world, type_registry, filter, child)
            }).collect()
        });

        Self::new(entity_fragment, children)
    }

    /// Creates a new HierarchyFragment from the hieararchy of a world entity.
    ///
    /// * `world`: World to copy from
    /// * `type_registry`: Type registry used to control what will be extracted from the world.
    /// * `filter`: 
    /// * `entity`: 
    pub fn from_world_entity(world: &mut World, type_registry: &TypeRegistry, filter: &SceneFilter, entity: Entity) -> HierarchyFragment {
        let entity_fragment = EntityFragment::from_world_entity(world, type_registry, filter, entity);

        let children: Option<SmallVec<[Entity; 8]>> = world.get::<Children>(entity).map(|children| children.into_iter().cloned().collect());
        let children: Option<Vec<HierarchyFragment>> = children.map(|children| {
            children.into_iter().map(|child| {
                HierarchyFragment::from_world_entity(world, type_registry, filter, child)
            }).collect()
        });

        Self::new(entity_fragment, children)
    }

    /// Spawns the entity in the world, recursively spawning children
    ///
    /// * `world`: World to spawn into
    /// * `type_registry`: Type registry used to construct this object 
    pub fn spawn_in_world(&self, world: &mut World, type_registry: &TypeRegistry) -> Entity {
        let parent = self.entity_fragment.spawn_in_world(world, type_registry).id();

        if let Some(children) = &self.children {
            for child in children {
                child.spawn_in_world_with_parent_entity(world, type_registry, parent);
            }
        }

        parent
    }

    /// Spawns the entity in the world with a parent entity, recursively spawns children
    ///
    /// * `world`: World to spawn into
    /// * `type_registry`: Type registry used when constructing this HierarchyFragment
    /// * `parent`: Parent to spawn as a child of
    pub fn spawn_in_world_with_parent_entity(&self, world: &mut World, type_registry: &TypeRegistry, parent: Entity) -> Entity {
        let entity = self.entity_fragment.spawn_in_world_with_parent_entity(world, type_registry, parent).id();

        if let Some(children) = &self.children {
            for child in children {
                child.spawn_in_world_with_parent_entity(world, type_registry, entity);
            }
        }

        entity
    }

    /// Spawns the entity in the world with a parent entity (lookup by Uid), recursively spawns children
    ///
    /// * `world`: World to spawn into
    /// * `type_registry`: Type registry used when constructing this HierarchyFragment
    /// * `parent`: Parent to spawn as a child of
    pub fn spawn_in_world_with_parent_uid(&self, world: &mut World, type_registry: &TypeRegistry, parent: Uid) -> Entity {
        let parent = parent.entity(world).unwrap();
        self.spawn_in_world_with_parent_entity(world, type_registry, parent)

    }
}
