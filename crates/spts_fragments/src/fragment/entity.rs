use bevy_ecs::{entity::Entity, world::{EntityWorldMut, World}};
use bevy_hierarchy::BuildWorldChildren;
use bevy_reflect::TypeRegistry;
use bevy_scene::SceneFilter;

// #[cfg(feature = "serde")]
// use serde::{Serialize, Deserialize};

use crate::uid::Uid;

use super::component::ComponentFragment;

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
    ) -> Vec<ComponentFragment> {
        let entity_ref = world.get_entity(entity).unwrap();
        let mut components = vec![];

        for comp_id in entity_ref.archetype().components() {
            let component_fragment = world
                .components()
                .get_info(comp_id)
                .and_then(|c| c.type_id())
                .and_then(|id| {
                    ComponentFragment::from_type_id(type_registry, &entity_ref, id)
                });

            if let Some(cf) = component_fragment {
                components.push(cf);
            }
        }

        components
    }

    pub fn from_world_uid(
        world: &mut World,
        type_registry: &TypeRegistry,
        uid: Uid,
    ) -> Self {
        let entity = uid.entity(world).unwrap();
        let components =
            EntityFragment::components_from_entity(world, type_registry, entity);
        EntityFragment::new(uid, components)
    }

    pub fn from_world_entity(
        world: &mut World,
        type_registry: &TypeRegistry,
        entity: Entity,
    ) -> Self {
        let uid = *world.get::<Uid>(entity).unwrap();
        let components =
            EntityFragment::components_from_entity(world, type_registry, entity);
        EntityFragment::new(uid, components)
    }

    pub fn spawn_in_world<'w>(&self, world: &'w mut World, type_registry: &TypeRegistry) -> EntityWorldMut<'w> {
        let mut entity_mut = world.spawn(self.uid);

        for comp in &self.components {
            comp.insert_to_entity_world_mut(type_registry, &mut entity_mut).unwrap();
        }

        entity_mut
    }

    pub fn spawn_in_world_with_parent_entity<'w>(&self, world: &'w mut World, type_registry: &TypeRegistry, parent: Entity) -> EntityWorldMut<'w> {
        let mut entity_mut = self.spawn_in_world(world, type_registry);
        entity_mut.set_parent(parent);
        entity_mut
    }

    pub fn spawn_in_world_with_parent_uid<'w>(&self, world: &'w mut World, type_registry: &TypeRegistry, parent: Uid) -> EntityWorldMut<'w> {
        let parent = parent.entity(world).unwrap();
        self.spawn_in_world_with_parent_entity(world, type_registry, parent)
    }
}
