use bevy_ecs::{
    bundle::Bundle,
    entity::Entity,
    reflect::ReflectComponent,
    world::{EntityWorldMut, World},
};
use bevy_reflect::{Reflect, ReflectRef, TypeRegistry};
use bevy_scene::SceneFilter;
use bevy_spts_uid::Uid;

use super::{ComponentFragment, ComponentFragmentError, EntityFragmentNewError};

pub trait BundleToFragment {
    fn to_fragment(&self, type_registry: &TypeRegistry) -> BundleFragment;
}

fn bundle_to_component_array_recursive(
    object: ReflectRef<'_>,
    type_registry: &TypeRegistry,
    out: &mut Vec<ComponentFragment>,
) {
    match object {
        ReflectRef::Tuple(tup) => {
            for el in tup.iter_fields() {
                let type_id = el.type_id();
                if type_registry.get_type_data::<ReflectComponent>(type_id).is_some() {
                    let cf = ComponentFragment::new(el.clone_value().into());
                    out.push(cf);
                } else {
                    bundle_to_component_array_recursive(el.reflect_ref(), type_registry, out);
                }
            }
        }
        ReflectRef::Struct(str) => {
            for el in str.iter_fields() {
                let type_id = el.type_id();
                if type_registry.get_type_data::<ReflectComponent>(type_id).is_some() {
                    let cf = ComponentFragment::new(el.clone_value().into());
                    out.push(cf);
                } else {
                    bundle_to_component_array_recursive(el.reflect_ref(), type_registry, out);
                }
            }
        }
        _ => panic!("bevy_spts_fragments: Error converting bundle to BundleFragment.  Only Struct/Tuple objects are supported.")
    }
}

impl<B: Bundle + Reflect> BundleToFragment for B {
    fn to_fragment(&self, type_registry: &TypeRegistry) -> BundleFragment {
        let mut components = vec![];

        bundle_to_component_array_recursive(self.reflect_ref(), type_registry, &mut components);

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
    ) -> Result<(), ComponentFragmentError> {
        for component in &self.components {
            component.insert(entity, type_registry)?;
        }
        Ok(())
    }

    pub fn apply(
        &self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentFragmentError> {
        for component in &self.components {
            component.apply(entity, type_registry)?;
        }
        Ok(())
    }

    pub fn apply_or_insert(
        &self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentFragmentError> {
        for component in &self.components {
            component.apply_or_insert(entity, type_registry)?;
        }
        Ok(())
    }

    pub fn remove(
        &self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentFragmentError> {
        for component in &self.components {
            component.apply_or_insert(entity, type_registry)?;
        }
        Ok(())
    }
    pub fn swap(
        &mut self,
        entity: &mut EntityWorldMut,
        type_registry: &TypeRegistry,
    ) -> Result<(), ComponentFragmentError> {
        for component in &mut self.components {
            component.swap(entity, type_registry)?;
        }
        Ok(())
    }

    pub fn from_world(
        world: &mut World,
        type_registry: &TypeRegistry,
        filter: &SceneFilter,
        entity: Entity,
    ) -> Result<Self, EntityFragmentNewError> {
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

        Ok(Self::new(components))
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
        Self::from_world(world, type_registry, filter, entity)
    }
}
