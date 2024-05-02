use bevy::{ecs::{bundle::Bundle, component::Component, reflect::ReflectComponent}, reflect::Reflect};

pub mod core;
pub mod node;
pub mod position;

#[derive(Component, Reflect)]
#[reflect(Component)]
/// Marker component for an object that should not be visible in the frontend / editor.
pub struct InternalObject;

#[derive(Bundle)]
pub struct ObjectBundle {

}
