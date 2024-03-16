use bevy::ecs::component::Component;
use serde::{Serialize, Deserialize};

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
/// Component tag that says this entity has a material that is derived 
/// from other components on this entity later on.
pub struct DerivedMaterial;

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
/// Component tag that says this entity has a mesh that is derived 
/// from other components on this entity later on.
pub struct DerivedMesh;
