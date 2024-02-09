use std::{fmt::{Debug, Display}, hash::Hash};

use bevy::{prelude::*, utils::{Uuid, HashMap}, reflect::Map};

#[derive(
    Component, Reflect, Eq, PartialEq, Hash, Copy, Clone, serde::Serialize, serde::Deserialize,
)]
/// A unique identifier that can be used to
pub struct Idx(pub [u64; 2]);

impl Idx {
    pub fn new() -> Self {
        Self(Uuid::new_v4().as_u64_pair().into())
    }
}

impl Default for Idx {
    fn default() -> Self {
        Idx::new()
    }
}

impl Debug for Idx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BBID({}-{})", self.0[0], self.0[1])
    }
}
impl Display for Idx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BBID({}-{})", self.0[0], self.0[1])
    }
}

#[derive(Resource)]
pub struct IdxResource {
    forward: HashMap<Idx, Entity>
}

pub trait WorldIndex {
    fn idx(&mut self, idx: Idx) -> Option<Entity>;
    fn register_idx(&mut self, idx: Idx, entity: Entity);
    fn unregister_idx(&mut self, idx: Idx, entity: Entity);
}

impl WorldIndex for World {
    fn idx(&mut self, idx: Idx) -> Option<Entity> {
        let res = self.resource_mut::<IdxResource>();
        if let Some(e) = res.forward.get(&idx) {
            return Some(*e);
        }

        let mut q_idx = self.query::<(Entity, &Idx)>();
        let entity = q_idx.iter(self).find_map(|(e, idx2)| {
            if idx == *idx2 {
                Some(e)
            } else {
                None
            }
        });

        if let Some(e) = entity {
            self.resource_mut::<IdxResource>().forward.insert(idx, e);
        }

        entity
    }

    fn register_idx(&mut self, idx: Idx, entity: Entity) {
        let mut res = self.resource_mut::<IdxResource>();
        debug_assert!(res.forward.get(&idx).is_none());
        res.forward.insert(idx, entity);
    }

    fn unregister_idx(&mut self, idx: Idx, entity: Entity) {
        let mut res = self.resource_mut::<IdxResource>();
        debug_assert!(res.forward.get(&idx).is_some());
        res.forward.remove(&idx);
    }
}
