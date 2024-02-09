use std::marker::PhantomData;

use bevy::ecs::component::Component;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::index::{Idx, WorldIndex};

mod serialisable;

pub trait BBChange {
    fn apply(&self, world: &mut World) -> Result<Box<dyn BBChange>, anyhow::Error>;
}

/// Mutate component BBChange.  Mutates a component on an entity
///
/// * `old_value`:
/// * `new_value`:
/// * `target`:
pub struct MutateComponent<C: Component + Serialize + for<'a> Deserialize<'a> + Clone> {
    value: C,
    target: Idx,
}
impl<C: Component + Serialize + for<'a> Deserialize<'a> + Clone> BBChange for MutateComponent<C> {
    fn apply(&self, world: &mut World) -> Result<Box<dyn BBChange>, anyhow::Error> {
        let entity = world.idx(self.target).unwrap();
        let mut entity_mut = world.entity_mut(entity);
        let old_value = entity_mut.get::<C>().unwrap().clone();
        entity_mut.insert::<C>(self.value);
        Ok(Box::new(MutateComponent {
            target: self.target,
            value: old_value,
        }))
    }
}

/// AddComponent BBChange.
/// Adds a component to an entity
///
/// * `to_add`:
/// * `target`:
pub struct AddComponent<C: Component + Serialize + for<'a> Deserialize<'a> + Clone> {
    to_add: C,
    target: Idx,
}
impl<C: Component + Serialize + for<'a> Deserialize<'a> + Clone> BBChange for AddComponent<C> {
    fn apply(&self, world: &mut World) -> Result<Box<dyn BBChange>, anyhow::Error> {
        let entity = world.idx(self.target).unwrap();
        let mut entity_mut = world.entity_mut(entity);
        entity_mut.insert::<C>(self.to_add);
        Ok(Box::new(RemoveComponent {
            target: self.target,
            to_remove: PhantomData::<C>::default(),
        }))
    }
}

/// Removes a component from an entity.
///
/// * `to_remove`:
/// * `target`:
pub struct RemoveComponent<C: Component + Serialize + for<'a> Deserialize<'a> + Clone> {
    to_remove: PhantomData<C>,
    target: Idx,
}
impl<C: Component + Serialize + for<'a> Deserialize<'a> + Clone> BBChange for RemoveComponent<C> {
    fn apply(&self, world: &mut World) -> Result<Box<dyn BBChange>, anyhow::Error> {
        let entity = world.idx(self.target).unwrap();
        let mut entity_mut = world.entity_mut(entity);
        let component = entity_mut.get::<C>().unwrap().clone();
        entity_mut.remove::<C>();
        Ok(Box::new(AddComponent {
            target: self.target,
            to_add: component,
        }))
    }
}

// /// Removes a component from an entity.
// ///
// /// * `to_remove`:
// /// * `target`:
// pub struct SpawnEntity<C: Vec<Component + Serialize + for<'a> Deserialize<'a> + Clone>> {
//     parent_target: Idx,
//     components: Vec<dyn Component + Serialize + for<'a> Deserialize<'a> + Clone>,
// }
//
// impl<C: Component + Serialize + for<'a> Deserialize<'a> + Clone> SpawnEntity<C> {
//     pub fn new(parent: Idx, components: Vec<C>) -> Self {
//         Self {
//             parent_target: parent,
//             components,
//         }
//     }
// }
//
// const VVV = SpawnEntity::new(Idx::new(), vec![Idx::new(), Transform::default()]);
//
// impl<C: Component + Serialize + for<'a> Deserialize<'a> + Clone> BBChange for SpawnEntity<C> {
//     fn apply(&self, world: &mut World) -> Result<Box<dyn BBChange>, anyhow::Error> {
//         let idx = Idx::new();
//         let entity_mut = world.spawn(idx);
//
//         for comp in self.components {
//             entity_mut.insert(comp);
//         }
//
//         Ok(Box::new(DespawnEntity<C> { target: idx }))
//     }
// }
//
// /// Despawn Entity
// ///
// /// * `to_remove`:
// /// * `target`:
// pub struct DespawnEntity<C: Component + Serialize + for<'a> Deserialize<'a> + Clone> {
//     target: Idx,
// }
// impl<C: Component + Serialize + for<'a> Deserialize<'a> + Clone> BBChange for DespawnEntity<C> {
//     fn apply(&self, world: &mut World) -> Result<Box<dyn BBChange>, anyhow::Error> {
//         todo!()
//     }
// }
