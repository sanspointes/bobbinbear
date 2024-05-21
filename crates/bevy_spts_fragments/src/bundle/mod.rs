use bevy_ecs::{
    bundle::Bundle,
    entity::Entity,
    reflect::ReflectComponent,
    world::{EntityRef, EntityWorldMut, World},
};
use bevy_reflect::{Reflect, ReflectRef, TypeRegistry};
use bevy_scene::SceneFilter;
use bevy_spts_uid::{Uid, UidRegistry, UidRegistryError};

use crate::{
    component::{ComponentApplyError, ComponentFromTypeIdError, ComponentReflectError},
    prelude::*,
};

mod errors;
pub use errors::*;

pub trait BundleToFragment {
    fn to_fragment(&self, type_registry: &TypeRegistry) -> BundleFragment;
}

fn bundle_to_component_array_recursive(
    object: &dyn Reflect,
    type_registry: &TypeRegistry,
    out: &mut Vec<ComponentFragment>,
) {
    let type_id = object.type_id();
    if type_registry
        .get_type_data::<ReflectComponent>(type_id)
        .is_some()
    {
        let cf = ComponentFragment::new(object.clone_value().into());
        out.push(cf);
        return;
    }

    match object.reflect_ref() {
        ReflectRef::Tuple(tup) => {
            for el in tup.iter_fields() {
                bundle_to_component_array_recursive(el, type_registry, out);
            }
        }
        ReflectRef::Struct(str) => {
            for el in str.iter_fields() {
                bundle_to_component_array_recursive(el, type_registry, out);
            }
        }
        _ => panic!("bevy_spts_fragments: Error converting bundle to BundleFragment .\nOnly tuple/struct bundles are supported.\nAll components must be registered in the type registry.\n\nobject: {object:?}")
    }
}

impl<B: Bundle + Reflect> BundleToFragment for B {
    fn to_fragment(&self, type_registry: &TypeRegistry) -> BundleFragment {
        let mut components = vec![];

        bundle_to_component_array_recursive(self.as_reflect(), type_registry, &mut components);

        BundleFragment { components }
    }
}

#[derive(Debug, Clone)]
/// Stores a number of ComponentFragments.
///
/// * `component`:
pub struct BundleFragment {
    components: Vec<ComponentFragment>,
}

impl BundleFragment {
    pub fn new(components: Vec<ComponentFragment>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> &[ComponentFragment] {
        self.components.as_slice()
    }

    pub fn components_mut(&mut self) -> &[ComponentFragment] {
        self.components.as_slice()
    }

    pub fn insert(
        &self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentReflectError> {
        for component in &self.components {
            component.insert(entity, type_registry)?;
        }
        Ok(())
    }

    pub fn apply(
        &self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentApplyError> {
        for component in &self.components {
            component.apply(entity, type_registry)?;
        }
        Ok(())
    }

    pub fn apply_or_insert(
        &self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentReflectError> {
        for component in &self.components {
            component.apply_or_insert(entity, type_registry)?;
        }
        Ok(())
    }

    pub fn remove(
        &self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentReflectError> {
        for component in &self.components {
            component.apply_or_insert(entity, type_registry)?;
        }
        Ok(())
    }
    pub fn swap(
        &mut self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentApplyError> {
        for component in &mut self.components {
            component.swap(entity, type_registry)?;
        }
        Ok(())
    }

    pub fn from_entity(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        entity: Entity,
    ) -> Result<Self, BundleFromEntityError> {
        let entity_ref = world
            .get_entity(entity)
            .ok_or(BundleFromEntityError::EntityNotExist { entity })?;
        let mut components = vec![];

        for comp_id in entity_ref.archetype().components() {
            let type_id = world
                .components()
                .get_info(comp_id)
                .and_then(|c| c.type_id());
            let Some(type_id) = type_id else {
                continue;
            };

            if filter.is_allowed_by_id(type_id) {
                let component_fragment =
                    ComponentFragment::from_type_id(type_registry, &entity_ref, type_id)?;
                components.push(component_fragment);
            }
        }

        Ok(Self::new(components))
    }
}
