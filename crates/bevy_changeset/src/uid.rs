use std::fmt::{Debug, Display};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use bevy_ecs::{component::Component, entity::Entity, prelude::ReflectComponent, world::World};
use bevy_reflect::Reflect;
use uuid::Uuid;


/// A unique identifier that can be used to lookup entities, persists between 
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
