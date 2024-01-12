mod serialised_component;

use bevy::{prelude::*, reflect::List, sprite::Mesh2dHandle};

use bevy_mod_raycast::RaycastMesh;
use serde::{Deserialize, Serialize};

use crate::{
    components::bbid::BBId,
    plugins::{
        bounds_2d_plugin::GlobalBounds2D,
        selection_plugin::{Selectable, Selected},
        vector_graph_plugin::{Fill, Stroke, VectorGraph},
    },
};

use self::serialised_component::{SerialisedComponent, ColorMaterialHandleDef};

#[derive(Clone, Serialize, Deserialize)]
pub struct SerialisableEntity {
    bbid: BBId,
    components: Vec<SerialisedComponent>,
    children: Vec<SerialisableEntity>,
}
impl SerialisableEntity {
    pub fn new(bbid: BBId) -> Self {
        Self {
            bbid,
            components: vec![],
            children: vec![],
        }
    }

    pub fn from_entity_recursive(world: &World, entity: Entity) -> Option<SerialisableEntity> {
        let Some(bbid) = world.get::<BBId>(entity) else {
            return None;
        };

        let mut serialised = SerialisableEntity::new(*bbid);

        if let Some(name) = world.get::<Name>(entity) {
            serialised.components.push(name.clone().into());
        }

        if let Some(transform) = world.get::<Transform>(entity) {
            serialised.components.push((*transform).into())
        }
        if let Some(value) = world.get::<GlobalTransform>(entity) {
            serialised.components.push((*value).into())
        }
        if let Some(value) = world.get::<Visibility>(entity) {
            serialised.components.push((*value).into())
        }
        if let Some(value) = world.get::<ComputedVisibility>(entity) {
            serialised.components.push((value.clone()).into())
        }
        if let Some(value) = world.get::<RaycastMesh<Selectable>>(entity) {
            serialised.components.push((value.clone()).into())
        }

        if let Some(value) = world.get::<Handle<ColorMaterial>>(entity) {
            let def = ColorMaterialHandleDef::from_world_and_handle(&world, value.clone()).into();
            serialised.components.push(SerialisedComponent::ColorMaterial(def));
        }


        if let Some(value) = world.get::<Selectable>(entity) {
            serialised.components.push((*value).into())
        }
        if let Some(value) = world.get::<Selected>(entity) {
            serialised.components.push((*value).into())
        }
        if let Some(value) = world.get::<Fill>(entity) {
            serialised.components.push((*value).into())
        }
        if let Some(value) = world.get::<Stroke>(entity) {
            serialised.components.push((*value).into())
        }
        if let Some(value) = world.get::<VectorGraph>(entity) {
            serialised.components.push((value.clone()).into())
        }
        if let Some(value) = world.get::<GlobalBounds2D>(entity) {
            serialised.components.push((*value).into())
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
                SerialisedComponent::Name(value) => e.insert(Name::from(value.clone())),
                SerialisedComponent::Transform(value) => e.insert(Transform::from(value.clone())),
                SerialisedComponent::GlobalTransform(value) => {
                    e.insert(GlobalTransform::from(value.clone()))
                }
                SerialisedComponent::Visibility(value) => e.insert(Visibility::from(value.clone())),
                SerialisedComponent::ComputedVisibility(value) => {
                    e.insert(ComputedVisibility::from(value.clone()))
                }

                SerialisedComponent::ColorMaterial(def) => {
                    e.insert(def.to_world_and_handle(world))
                }
                SerialisedComponent::Mesh2dHandle(def) => {
                    e.insert(Mesh2dHandle::from(def.clone()))
                }

                SerialisedComponent::RaycastMeshSelectable(value) => {
                    e.insert(RaycastMesh::<Selectable>::from(value.clone()))
                }
                SerialisedComponent::Selectable(value) => e.insert(Selectable::from(value.clone())),
                SerialisedComponent::Selected(value) => e.insert(Selected::from(value.clone())),
                SerialisedComponent::Fill(value) => e.insert(value.clone()),
                SerialisedComponent::Stroke(value) => e.insert(value.clone()),
                SerialisedComponent::VectorGraph(value) => e.insert(value.clone()),
                SerialisedComponent::GlobalBounds2D(value) => {
                    e.insert(GlobalBounds2D::from(value.clone()))
                }
            };
        }

        let parent_e = e.id();

        for child in &self.children {
            let mut child_e = Self::to_entity_recursive(child, world);
            world.entity_mut(parent_e).add_child(child_e);
        }

        parent_e
    }
}
