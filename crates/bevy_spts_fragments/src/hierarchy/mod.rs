use std::{collections::BTreeMap, fmt::Display};

use bevy_ecs::{entity::Entity, world::World};
use bevy_hierarchy::{BuildWorldChildren, Children, DespawnRecursiveExt};
use bevy_reflect::TypeRegistry;
use bevy_scene::SceneFilter;
use bevy_spts_uid::UidRegistry;
use smallvec::SmallVec;

use crate::prelude::*;

use self::errors::HierarchySpawnWithParentUidError;

mod errors;


#[derive(Debug, Clone)]
pub struct HierarchyFragmentEntity {
    entity_fragment: EntityFragment,
    children: Option<SmallVec<[Uid; 8]>>,
}

/// Stores an entire fragment of the scene heirarchy
///
/// * `entity_fragment`: The root entity
/// * `children`: Any children entities
#[derive(Debug, Clone)]
pub struct HierarchyFragment {
    root_uid: Uid,
    entities: BTreeMap<Uid, HierarchyFragmentEntity>,
}

impl Display for HierarchyFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "HierarchyFragment({}) {{", self.root_uid)?;
        for (uid, fragment) in self.entities.iter() {
            writeln!(
                f,
                "\t{}({}): children:",
                uid,
                fragment.entity_fragment.uid() == *uid
            )?;
            if let Some(children) = &fragment.children {
                for c in children {
                    writeln!(f, "\t\t{} ", c)?;
                }
            }
        }
        writeln!(f, "}}")
    }
}

impl HierarchyFragment {
    pub fn new(root_uid: Uid, entities: BTreeMap<Uid, HierarchyFragmentEntity>) -> Self {
        Self { root_uid, entities }
    }

    pub fn root_uid(&self) -> Uid {
        self.root_uid
    }

    pub fn all_uids(&self) -> impl Iterator<Item = &Uid> {
        self.entities.keys()
    }

    pub(crate) fn root(&self) -> &HierarchyFragmentEntity {
        self.entities.get(&self.root_uid).unwrap()
    }

    pub fn root_entity_fragment(&self) -> &EntityFragment {
        &self.root().entity_fragment
    }

    fn populate_entites_map_recursive(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        entity: Entity,
        entities: &mut BTreeMap<Uid, HierarchyFragmentEntity>,
    ) -> Result<(), EntityFromWorldError> {
        let entity_fragment =
            EntityFragment::from_world_entity(world, type_registry, filter, entity)?;

        let child_entities: Option<SmallVec<[Entity; 8]>> = world
            .get::<Children>(entity)
            .map(|children| children.iter().cloned().collect());

        let children: Option<SmallVec<[Uid; 8]>> = child_entities.as_ref().map(|child_entities| {
            child_entities
                .iter()
                .filter_map(|e| world.get::<Uid>(*e).cloned())
                .collect()
        });

        entities.insert(
            entity_fragment.uid(),
            HierarchyFragmentEntity {
                entity_fragment,
                children,
            },
        );

        if let Some(child_entities) = child_entities {
            for child in child_entities.iter() {
                Self::populate_entites_map_recursive(
                    world,
                    type_registry,
                    filter,
                    *child,
                    entities,
                )?;
            }
        }

        Ok(())
    }

    pub fn from_world(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
        entity: Entity,
    ) -> Result<HierarchyFragment, EntityFromWorldError> {
        let mut entities = BTreeMap::new();
        Self::populate_entites_map_recursive(world, type_registry, filter, entity, &mut entities)?;
        let frag = Self::new(uid, entities);
        Ok(frag)
    }

    /// Creates a HierarchyFragment from a root entity and will collect and serialize any children
    ///
    /// * `world`:
    /// * `type_registry`:
    /// * `filter`:
    /// * `uid`:
    pub fn from_world_uid(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
    ) -> Result<HierarchyFragment, EntityFromWorldError> {
        let entity = world.resource::<UidRegistry>().get_entity(uid)?;
        Self::from_world(world, type_registry, filter, uid, entity)
    }
    /// Creates a new HierarchyFragment from the hieararchy of a world entity.
    ///
    /// * `world`: World to copy from
    /// * `type_registry`: Type registry used to control what will be extracted from the world.
    /// * `filter`:
    /// * `entity`:
    pub fn from_world_entity(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        entity: Entity,
    ) -> Result<HierarchyFragment, EntityFromWorldError> {
        let uid = world.resource::<UidRegistry>().get_uid(entity)?;
        Self::from_world(world, type_registry, filter, uid, entity)
    }

    fn despawn_from_world(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
        entity: Entity,
    ) -> Result<HierarchyFragment, EntityFromWorldError> {
        let fragment = Self::from_world(world, type_registry, filter, uid, entity)?;

        let mut uid_registry = world.resource_mut::<UidRegistry>();
        for uid in fragment.entities.keys() {
            uid_registry.unregister(*uid);
        }

        world.entity_mut(entity).despawn_recursive();

        Ok(fragment)
    }

    pub fn despawn_from_world_uid(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        uid: Uid,
    ) -> Result<HierarchyFragment, EntityFromWorldError> {
        let entity = uid.entity(world).unwrap();
        Self::despawn_from_world(world, type_registry, filter, uid, entity)
    }

    pub fn despawn_from_world_entity(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        entity: Entity,
    ) -> Result<HierarchyFragment, EntityFromWorldError> {
        let uid = world.resource::<UidRegistry>().get_uid(entity)?;
        Self::despawn_from_world(world, type_registry, filter, uid, entity)
    }

    fn internal_spawn_recursive(
        &self,
        world: &mut World,
        type_registry: &TypeRegistry,
        uid: Uid,
        parent: Option<Entity>,
    ) -> Result<Entity, ComponentReflectError> {
        let he = self.entities.get(&uid).unwrap();
        let mut entity_mut = he.entity_fragment.spawn_in_world(world, type_registry)?;

        if let Some(parent) = parent {
            entity_mut.set_parent(parent);
        }

        let id = entity_mut.id();
        if let Some(children) = &he.children {
            for uid in children.clone() {
                self.internal_spawn_recursive(world, type_registry, uid, Some(id))?;
            }
        }

        Ok(id)
    }

    /// Spawns the entity in the world, recursively spawning children
    ///
    /// * `world`: World to spawn into
    /// * `type_registry`: Type registry used to construct this object
    pub fn spawn_in_world(
        &self,
        world: &mut World,
        type_registry: &TypeRegistry,
    ) -> Result<Entity, ComponentReflectError> {
        self.internal_spawn_recursive(world, type_registry, self.root_uid, None)
    }

    /// Spawns the entity in the world with a parent entity, recursively spawns children
    ///
    /// * `world`: World to spawn into
    /// * `type_registry`: Type registry used when constructing this HierarchyFragment
    /// * `parent`: Parent to spawn as a child of
    pub fn spawn_in_world_with_parent_entity(
        &self,
        world: &mut World,
        type_registry: &TypeRegistry,
        parent: Entity,
    ) -> Result<Entity, ComponentReflectError> {
        self.internal_spawn_recursive(world, type_registry, self.root_uid, Some(parent))
    }

    /// Spawns the entity in the world with a parent entity (lookup by Uid), recursively spawns children
    ///
    /// * `world`: World to spawn into
    /// * `type_registry`: Type registry used when constructing this HierarchyFragment
    /// * `parent`: Parent to spawn as a child of
    pub fn spawn_in_world_with_parent_uid(
        &self,
        world: &mut World,
        type_registry: &TypeRegistry,
        parent: Uid,
    ) -> Result<Entity, HierarchySpawnWithParentUidError> {
        let parent = world.resource::<UidRegistry>().get_entity(parent)?;
        let e = self.spawn_in_world_with_parent_entity(world, type_registry, parent)?;
        Ok(e)
    }
}
