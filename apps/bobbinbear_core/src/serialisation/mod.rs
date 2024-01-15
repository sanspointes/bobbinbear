mod serialised_component;

use bevy::{prelude::*, reflect::List};

use bevy_mod_raycast::RaycastMesh;
use serde::{Serialize, Deserialize};

use crate::{components::bbid::BBId, plugins::{selection_plugin::{Selectable, Selected}, bounds_2d_plugin::GlobalBounds2D}};

use self::serialised_component::SerialisedComponent;

#[derive(Serialize, Deserialize)]
struct SerializedEntity {
    bbid: BBId,
    components: Vec<SerialisedComponent>,
    children: Vec<SerializedEntity>,
}
impl SerializedEntity {
    pub fn new(bbid: BBId) -> Self {
        Self {
            bbid,
            components: vec![],
            children: vec![],
        }
    }

    pub fn from_entity_recursive(world: &World, entity: Entity) -> Option<SerializedEntity> {
        let Some(bbid) = world.get::<BBId>(entity) else {
            return None;
        };

        let mut serialised = SerializedEntity::new(bbid);

        if let Some(name) = world.get::<Name>(entity) {
            serialised.components.push(name.into());
        }

        if let Some(transform) = world.get::<Transform>(entity) {
            serialised.components.push(transform.into())
        }
        if let Some(value) = world.get::<GlobalTransform>(entity) {
            serialised.components.push(value.into())
        }
        if let Some(value) = world.get::<Visibility>(entity) {
            serialised.components.push(value.into())
        }
        if let Some(value) = world.get::<ComputedVisibility>(entity) {
            serialised.components.push(value.into())
        }
        if let Some(value) = world.get::<RaycastMesh<Selectable>>(entity) {
            serialised.components.push(value.into())
        }
        if let Some(value) = world.get::<Selectable>(entity) {
            serialised.components.push(value.into())
        }
        if let Some(value) = world.get::<Selected>(entity) {
            serialised.components.push(value.into())
        }
        if let Some(value) = world.get::<GlobalBounds2D>(entity) {
            serialised.components.push(value.into())
        }
        if let Some(children) = world.get::<Children>(entity) {
            for child in children {
                if let Some(serialised_child) = Self::from_entity_recursive(world, *child) {
                    serialised.children.push(serialised_child);
                }
            }
        }

        Some(serialised)
    }

    pub fn to_entity_recursive(&self, world: &mut World) -> Entity {
        let mut e = world.spawn(self.bbid);

        for comp in &self.components {
            match comp {
                SerialisedComponent::Name(value) => e.insert(value),
                SerialisedComponent::Transform(value) => e.insert(value),
                SerialisedComponent::GlobalTransform(value) => e.insert(value),
                SerialisedComponent::Visibility(value) => e.insert(value),
                SerialisedComponent::ComputedVisibility(value) => e.insert(value),
                SerialisedComponent::RaycastMesh(value) => e.insert(value),
                SerialisedComponent::Selectable(value) => e.insert(value),
                SerialisedComponent::Selected(value) => e.insert(value),
                SerialisedComponent::GlobalBounds2D(value) => e.insert(value),
            }
        }

        for child in &self.children {
            let mut child_e = Self::to_entity_recursive(child, world);
            e.add_child(child_e);
        }

        e.id()
    }
}
