use std::{any::TypeId, sync::Arc};

use bevy_ecs::{
    component::Component, entity::Entity, reflect::ReflectComponent, world::{EntityRef, EntityWorldMut, World}
};
use bevy_reflect::{Reflect, TypeRegistry};
use bevy_scene::SceneFilter;

use crate::uid::Uid;

#[derive(Debug, Clone)]
pub enum FragmentApplyError {
    NoMatchingUid { uid: Uid },
    NoRepresentedType { type_path: String },
    UnregisteredButReflectedType { type_path: String },
    UnregisteredComponent { type_path: String },
}

#[derive(Debug, Clone)]
pub struct ComponentFragment {
    component: Arc<dyn Reflect>,
}

impl ComponentFragment {
    pub fn new(component: Arc<dyn Reflect>) -> Self {
        Self { component }
    }

    pub fn from_type_id_filtered(
        type_registry: &TypeRegistry,
        entity_ref: &EntityRef,
        type_id: TypeId,
        filter: &SceneFilter,
    ) -> Option<Self> {
        if filter.is_allowed_by_id(type_id) {
            Self::from_type_id(type_registry, entity_ref, type_id)
        } else {
            None
        }
    }

    pub fn from_type_id(
        type_registry: &TypeRegistry,
        entity_ref: &EntityRef,
        type_id: TypeId,
    ) -> Option<Self> {
        let v = type_registry
            .get(type_id)
            .and_then(|reg| reg.data::<ReflectComponent>())
            .and_then(|reflect| reflect.reflect(*entity_ref));

        v.map(|reflect| Self::new(reflect.clone_value().into()))
    }

    /// Creates a new ComponentFragment from a reflectable component
    ///
    /// * `component`: Component to copy as reflectable.
    pub fn from_component<T: Component + Reflect>(component: &T) -> Self {
        Self::new(component.as_reflect().clone_value().into())
    }


    /// If an Entity in the World has component T, create a ComponentFragment from it.
    ///
    /// * `world`: 
    /// * `entity`:  Entity to extract ComponentFragment from
    pub fn try_from_entity<T: Component + Reflect>(world: &mut World, entity: Entity) -> Option<Self> {
        let entity_ref = world.get_entity(entity)?;
        Self::try_from_entity_ref::<T>(&entity_ref)
    }

    /// If an EntityWorldMut has component T, create a ComponentFragment from it.
    ///
    /// * `entity_mut`: 
    pub fn try_from_entity_ref<T: Component + Reflect>(entity_ref: &EntityRef) -> Option<Self> {
        entity_ref.get::<T>().map(|component| {
            Self::from_component(component)
        })
    }

    pub fn insert_to_entity_world_mut(
        &self,
        type_registry: &TypeRegistry,
        entity_mut: &mut EntityWorldMut,
    ) -> Result<(), FragmentApplyError> {
        let type_info = self.component.get_represented_type_info().ok_or_else(|| {
            FragmentApplyError::NoRepresentedType {
                type_path: self.component.reflect_type_path().to_string(),
            }
        })?;
        let registration = type_registry.get(type_info.type_id()).ok_or_else(|| {
            FragmentApplyError::UnregisteredButReflectedType {
                type_path: type_info.type_path().to_string(),
            }
        })?;
        let reflect_component = registration.data::<ReflectComponent>().ok_or_else(|| {
            FragmentApplyError::UnregisteredComponent {
                type_path: type_info.type_path().to_string(),
            }
        })?;

        reflect_component.insert(entity_mut, &*self.component);
        Ok(())
    }

    pub fn insert_to_uid(
        &self,
        world: &mut World,
        type_registry: &TypeRegistry,
        uid: Uid,
    ) -> Result<(), FragmentApplyError> {
        let entity = uid
            .entity(world)
            .ok_or(FragmentApplyError::NoMatchingUid { uid })?;
        let mut entity_mut = world.entity_mut(entity);

        self.insert_to_entity_world_mut(type_registry, &mut entity_mut)
    }
}
