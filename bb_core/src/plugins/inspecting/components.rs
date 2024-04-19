use bevy::{
    ecs::{component::Component, reflect::ReflectComponent},
    reflect::Reflect,
};
use bevy_spts_uid::Uid;

#[derive(Component, Reflect, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[reflect(Component)]
/// Tag struct declaring this object as inspected.
/// Must be applied to a scene object.
pub struct Inspected;

#[derive(Component, Reflect, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
/// Tags a scene object as temporary, existing because another element is inspected.
/// Stores the uid of the inspected object.
pub struct BecauseInspected(pub Uid);
