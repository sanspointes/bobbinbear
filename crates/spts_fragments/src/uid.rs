use std::fmt::{Debug, Display};
use bevy_utils::Uuid;

use bevy_ecs::{component::Component, entity::Entity, prelude::ReflectComponent, world::{EntityWorldMut, World}};
use bevy_reflect::Reflect;


/// A unique identifier that can be used to lookup entities, persists between 
///
#[derive(Debug, Clone, Copy, Reflect, Component, PartialEq, Eq)]
#[reflect(Component)]
pub struct Uid(u64, u64);

impl Uid {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        uuid.as_u64_pair().into()
    }

    pub fn entity(&self, world: &mut World) -> Option<Entity> {
        world.query::<(Entity, &Uid)>().iter(world).find_map(|(e, uid)| {
            if *self == *uid {
                Some(e)
            } else {
                None
            }
        })
    }

    pub fn entity_world_mut<'a>(&'a self, world: &'a mut World) -> Option<EntityWorldMut> {
        let entity = self.entity(world)?;
        Some(world.entity_mut(entity))
    }
}

impl From<(u64, u64)> for Uid {
    fn from(value: (u64, u64)) -> Self {
        Self(value.0, value.1)
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl Default for Uid {
    fn default() -> Self {
        Uid::new()
    }
}
