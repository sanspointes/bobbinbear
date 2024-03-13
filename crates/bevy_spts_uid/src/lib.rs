use bevy_mod_index::{
    index::IndexInfo, refresh_policy::ConservativeRefreshPolicy, storage::HashmapStorage,
};
use bevy_utils::Uuid;
use std::fmt::{Debug, Display};
use uuid::Error;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    prelude::ReflectComponent,
    world::{EntityWorldMut, World},
};
use bevy_reflect::Reflect;

pub use bevy_mod_index::*;

pub mod extension;

/// A unique identifier that can be used to lookup entities, persists between
///
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Reflect, Component, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[reflect(Component)]
pub struct Uid(Uuid);

impl Uid {
    pub fn new(uuid: Uuid) -> Self {
        Uid(uuid)
    }

    pub fn inner(&self) -> &Uuid {
        &self.0
    }

    pub fn entity(&self, world: &mut World) -> Option<Entity> {
        world
            .query::<(Entity, &Uid)>()
            .iter(world)
            .find_map(|(e, uid)| if *self == *uid { Some(e) } else { None })
    }

    pub fn entity_world_mut<'a>(&'a self, world: &'a mut World) -> Option<EntityWorldMut> {
        let entity = self.entity(world)?;
        Some(world.entity_mut(entity))
    }
}

impl TryFrom<&String> for Uid {
    type Error = uuid::Error;
    fn try_from(value: &String) -> Result<Self, Error> {
        let uuid = Uuid::parse_str(value)?;
        Ok(Uid(uuid))
    }
}
impl TryFrom<&str> for Uid {
    type Error = uuid::Error;
    fn try_from(value: &str) -> Result<Self, Error> {
        let uuid = Uuid::parse_str(value)?;
        Ok(Uid(uuid))
    }
}
impl From<&Uid> for String {
    fn from(value: &Uid) -> Self {
        value.inner().to_string()
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl Default for Uid {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        Uid::new(uuid)
    }
}

impl IndexInfo for Uid {
    type Component = Uid;
    type Value = Uid;
    type Storage = HashmapStorage<Self>;
    type RefreshPolicy = ConservativeRefreshPolicy;

    fn value(c: &Self::Component) -> Self::Value {
        *c
    }
}
