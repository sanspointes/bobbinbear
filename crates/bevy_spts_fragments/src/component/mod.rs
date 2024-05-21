use std::{any::TypeId, sync::Arc};

use bevy_ecs::{
    component::Component,
    entity::Entity,
    reflect::ReflectComponent,
    world::{EntityRef, EntityWorldMut, World},
};
use bevy_reflect::{Reflect, TypeInfo, TypeRegistry};

mod errors;
pub use errors::*;

pub trait ComponentToFragment: Component + Reflect + Sized {
    fn to_fragment(&self) -> ComponentFragment {
        ComponentFragment::from_component(self)
    }
}

#[derive(Debug, Clone)]
/// Wrapper around a Arc<dyn Reflect> that stores a Reflect component.
///
/// * `component`:
pub struct ComponentFragment {
    component: Arc<dyn Reflect>,
}

impl ComponentFragment {
    pub fn new(component: Arc<dyn Reflect>) -> Self {
        Self { component }
    }

    pub fn from_type_id(
        type_registry: &TypeRegistry,
        entity_ref: &EntityRef,
        type_id: TypeId,
    ) -> Result<Self, ComponentFromTypeIdError> {
        let registration = type_registry
            .get(type_id)
            .ok_or_else(|| ComponentFromTypeIdError::NotRegistered { type_id })?;

        let reflect_component = registration
            .data::<ReflectComponent>()
            .ok_or_else(|| ComponentFromTypeIdError::NotReflectComponent { type_id })?;

        let component_value = reflect_component.reflect(*entity_ref).ok_or_else(|| {
            ComponentFromTypeIdError::NotContainedByEntity {
                entity: entity_ref.id(),
                type_id,
            }
        })?;

        Ok(Self { component: component_value.clone_value().into() })
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
    pub fn try_from_entity<T: Component + Reflect>(
        world: &mut World,
        entity: Entity,
    ) -> Option<Self> {
        let entity_ref = world.get_entity(entity)?;
        Self::try_from_entity_ref::<T>(&entity_ref)
    }

    /// If an EntityWorldMut has component T, create a ComponentFragment from it.
    ///
    /// * `entity_mut`:
    pub fn try_from_entity_ref<T: Component + Reflect>(entity_ref: &EntityRef) -> Option<Self> {
        entity_ref
            .get::<T>()
            .map(|component| Self::from_component(component))
    }

    /// Gets the ReflectComponent of the component that this fragment represents.
    pub fn get_reflect_component<'a>(
        &'a self,
        type_registry: &'a TypeRegistry,
    ) -> Result<&'a ReflectComponent, ComponentReflectError> {
        use ComponentReflectError::*;

        let type_info =
            self.component
                .get_represented_type_info()
                .ok_or_else(|| NoRepresentedType {
                    type_path: self.component.reflect_type_path().to_string(),
                })?;
        let registration =
            type_registry
                .get(type_info.type_id())
                .ok_or_else(|| UnregisteredButReflectedType {
                    type_path: type_info.type_path().to_string(),
                })?;
        registration
            .data::<ReflectComponent>()
            .ok_or_else(|| UnregisteredComponent {
                type_path: type_info.type_path().to_string(),
            })
    }

    /// Inserts this Component into a EntityWorldMut
    pub fn insert(
        &self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentReflectError> {
        let comp = self.get_reflect_component(type_registry)?;
        comp.insert(entity, &*self.component, type_registry);
        Ok(())
    }
    /// Updates the Component on an EntityWorldMut
    /// WARN: Will panic if the entity doesn't already have this component.
    /// TODO: Figure out how to use `EntityComponent::contains` to check and throw error instead.
    pub fn apply(
        &self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentApplyError> {
        let comp = self.get_reflect_component(type_registry)?;
        comp.apply(entity, &*self.component);
        Ok(())
    }
    pub fn apply_or_insert(
        &self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentReflectError> {
        let comp = self.get_reflect_component(type_registry)?;
        comp.apply_or_insert(entity, &*self.component, type_registry);
        Ok(())
    }
    pub fn remove(
        &self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentReflectError> {
        let comp = self.get_reflect_component(type_registry)?;
        comp.remove(entity);
        Ok(())
    }
    /// Tries to swap the reflected component on the  
    ///
    /// * `entity`:
    /// * `type_registry`:
    pub fn swap(
        &mut self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentApplyError> {
        let comp = self.get_reflect_component(type_registry)?;
        let reflected = comp.reflect_mut(entity).ok_or_else(|| {
            ComponentApplyError::ComponentMissingOnEntity {
                type_path: self.component.reflect_type_path().to_string(),
            }
        })?;
        let reflected: Arc<_> = reflected.clone_value().into();
        comp.apply(entity, &*self.component);
        self.component = reflected;
        Ok(())
    }

    /// Gets the TypeId of the Component
    ///
    /// * `type_registry`: Type Registry to get the type id from
    pub fn try_type_id(&self) -> Result<TypeId, ComponentReflectError> {
        let type_info = self.try_type_info()?;
        Ok(type_info.type_id())
    }

    pub fn try_type_info(&self) -> Result<&TypeInfo, ComponentReflectError> {
        use ComponentReflectError::*;

        let type_info =
            self.component
                .get_represented_type_info()
                .ok_or_else(|| NoRepresentedType {
                    type_path: self.component.reflect_type_path().to_string(),
                })?;
        Ok(type_info)
    }
}
