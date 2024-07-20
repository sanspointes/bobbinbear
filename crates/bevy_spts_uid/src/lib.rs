use bevy_utils::{tracing::warn, HashMap};
use uuid::Uuid;
use core::panic;
use std::fmt::{Debug, Display};
use thiserror::Error;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    prelude::ReflectComponent,
    system::Resource,
    world::{EntityWorldMut, World},
    reflect::ReflectResource,
};
use bevy_reflect::Reflect;

pub mod extension;

pub use uuid;

#[cfg(feature = "tsify")]
use tsify::Tsify;
#[cfg(feature = "tsify")]
use wasm_bindgen::prelude::*;

/// A unique identifier that can be used to lookup entities, persists between
///
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Reflect, Component, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[reflect(Component)]
pub struct Uid(#[cfg_attr(feature = "serde", serde(with = "uuid_string"))] (u64, u64));

#[cfg(feature = "serde")]
mod uuid_string {
    use std::str::FromStr;

    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};
    use uuid::Uuid;

    pub fn serialize<S: Serializer>(v: &(u64, u64), s: S) -> Result<S::Ok, S::Error> {
        let uuid = Uuid::from_u64_pair(v.0, v.1);
        String::serialize(&uuid.to_string(), s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<(u64, u64), D::Error> {
        let string = String::deserialize(d)?;
        match Uuid::from_str(string.as_str()) {
            Ok(uuid) => Ok(uuid.as_u64_pair()),
            Err(reason) => Err(serde::de::Error::custom(reason)),
        }
    }
}

impl Uid {
    pub fn new(uuid: Uuid) -> Self {
        Uid(uuid.as_u64_pair())
    }

    pub fn inner(&self) -> Uuid {
        Uuid::from_u64_pair(self.0 .0, self.0 .1)
    }

    pub fn entity(&self, world: &World) -> Option<Entity> {
        world.resource::<UidRegistry>().get_entity(*self).ok()
    }

    pub fn get_entity(&self, world: &World) -> Result<Entity, UidRegistryError> {
        world.resource::<UidRegistry>().get_entity(*self)
    }

    pub fn register(&self, world: &mut World, entity: Entity) {
        let mut res = world.resource_mut::<UidRegistry>();
        let old = res.register(*self, entity);
        if old.is_some() {
            warn!("bevy_spts_uid: Registered uid ({self}, {entity:?}) has old value {old:?}.");
        }
    }

    pub fn unregister(&self, world: &mut World) {
        let mut res = world.resource_mut::<UidRegistry>();
        let old = res.unregister(*self);
        if old.is_none() {
            warn!("bevy_spts_uid: Unregistered uid ({self}) but not registered.");
        }
    }

    pub fn entity_world_mut<'a>(&'a self, world: &'a mut World) -> Option<EntityWorldMut> {
        let entity = self.entity(world)?;
        Some(world.entity_mut(entity))
    }
}

#[derive(Error, Debug)]
pub enum UidError {
    #[error("Unknown error.")]
    Unknown,
}

impl Debug for Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Uid({:?}", self.0)?;
        (self as &dyn Display).fmt(f)?;
        write!(f, ")")
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.inner())
    }
}

impl Default for Uid {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        Uid::new(uuid)
    }
}
//
// impl IndexInfo for Uid {
//     type Component = Uid;
//     type Value = Uid;
//     type Storage = HashmapStorage<Self>;
//     type RefreshPolicy = ConservativeRefreshPolicy;
//
//     fn value(c: &Self::Component) -> Self::Value {
//         *c
//     }
// }

#[derive(Debug, Error, Clone, Copy)]
pub enum UidRegistryError {
    #[error("No uid in registry: {0}. Can't lookup entity.")]
    NoEntity(Uid),
    #[error("No entity in registry: {0:?}. Can't lookup uid.")]
    NoUid(Entity),
}

#[derive(Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct UidRegistry{ 
    uid_to_e: HashMap<Uid, Entity>,
    e_to_uid: HashMap<Entity, Uid>
}

impl UidRegistry {
    pub fn register(&mut self, uid: Uid, entity: Entity) -> Option<(Entity, Uid)> {
        // info!("UidRegistry::register(uid: {uid}, entity: {entity:?})");
        let old_entity = self.uid_to_e.insert(uid, entity);
        let old_uid = self.e_to_uid.insert(entity, uid);
        match (old_entity, old_uid) {
            (Some(entity), Some(uid)) => Some((entity, uid)),
            (None, None) => None,
            state => panic!("Impossible registry state {state:?}."),
        }
    }
    pub fn unregister(&mut self, uid: Uid) -> Option<Entity> {
        // info!("UidRegistry::unregister(uid: {uid})");
        if let Some(entity) = self.uid_to_e.get(&uid) {
            self.e_to_uid.remove(entity);
            self.uid_to_e.remove(&uid)
        } else {
            None
        }
    }

    pub fn get_entity(&self, uid: Uid) -> Result<Entity, UidRegistryError> {
        match self.uid_to_e.get(&uid) {
            Some(entity) => Ok(*entity),
            None => Err(UidRegistryError::NoEntity(uid)),
        }
    }
    pub fn get_uid(&self, entity: Entity) -> Result<Uid, UidRegistryError> {
        match self.e_to_uid.get(&entity) {
            Some(uid) => Ok(*uid),
            None => Err(UidRegistryError::NoUid(entity)),
        }
    }

    pub fn entity(&self, uid: Uid) -> Entity {
        self.get_entity(uid).unwrap()
    }

    pub fn uid(&self, entity: Entity) -> Uid {
        self.get_uid(entity).unwrap()
    }
}
