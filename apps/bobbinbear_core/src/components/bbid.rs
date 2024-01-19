use std::fmt::{Debug, Display};

use bevy::{ecs::world::EntityMut, prelude::*, reflect::Reflect, utils::Uuid};
use thiserror::Error;

#[derive(
    Component, Reflect, Eq, PartialEq, Hash, Copy, Clone, serde::Serialize, serde::Deserialize,
)]
/// A unique identifier that can be used to
pub struct BBId(pub [u64; 2]);

impl Default for BBId {
    fn default() -> Self {
        Self(Uuid::new_v4().as_u64_pair().into())
    }
}

impl Debug for BBId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BBID({}-{})", self.0[0], self.0[1])
    }
}
impl Display for BBId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BBID({}-{})", self.0[0], self.0[1])
    }
}

#[derive(Error, Debug)]
pub enum BbidWorldError {
    #[error("No entity with Bbid {0:?}")]
    NoEntityWithBbid(BBId),
}

// TODO: Add thread local caching for entity lookup? Eagerly check cache, compare with real and
// fallback to query all?

pub trait BBIdUtils {
    fn bbid(&mut self, bbid: BBId) -> Entity;
    fn try_bbid(&mut self, bbid: BBId) -> Option<Entity>;
    fn bbid_mut(&mut self, bbid: BBId) -> EntityWorldMut;
    fn try_bbid_mut(&mut self, bbid: BBId) -> Option<EntityWorldMut>;

    fn bbid_get<C: Component>(&mut self, bbid: BBId) -> &C;
    fn try_bbid_get<C: Component>(&mut self, bbid: BBId) -> Option<&C>;

    fn bbid_get_mut<C: Component>(&mut self, bbid: BBId) -> Mut<C>;
    fn try_bbid_get_mut<C: Component>(&mut self, bbid: BBId) -> Option<Mut<C>>;

    fn try_entities_by_bbid_vec(&mut self, bbids: &[BBId]) -> Result<Vec<Entity>, BbidWorldError>;
}

impl BBIdUtils for World {
    /// Gets an entity from the scene by BBId
    ///
    /// * `bbid`:
    fn bbid(&mut self, bbid: BBId) -> Entity {
        match self.try_bbid(bbid) {
            Some(entity) => entity,
            None => panic!(
                "entity_by_uuid: Could not find entity with uuid {:?}",
                bbid.0
            ),
        }
    }

    /// Tries to get an entity from the scene by bbid.
    ///
    /// * `bbid`:
    fn try_bbid(&mut self, bbid: BBId) -> Option<Entity> {
        self.query::<(Entity, &BBId)>()
            .iter(self)
            .find(|(_entity, id)| (*id).eq(&bbid))
            .map(|(e, _)| e)
    }

    fn bbid_mut(&mut self, bbid: BBId) -> EntityWorldMut {
        let e = self.bbid(bbid);
        self.entity_mut(e)
    }

    fn try_bbid_mut(&mut self, bbid: BBId) -> Option<EntityWorldMut> {
        match self.try_bbid(bbid) {
            Some(e) => Some(self.entity_mut(e)),
            None => None,
        }
    }

    fn bbid_get<C: Component>(&mut self, bbid: BBId) -> &C {
        let e = self.bbid(bbid);
        self.get::<C>(e).unwrap()
    }

    fn try_bbid_get<C: Component>(&mut self, bbid: BBId) -> Option<&C> {
        self.try_bbid(bbid).and_then(|e| self.get::<C>(e))
    }

    fn bbid_get_mut<C: Component>(&mut self, bbid: BBId) -> Mut<C> {
        let e = self.bbid(bbid);
        self.get_mut::<C>(e).unwrap()
    }

    fn try_bbid_get_mut<C: Component>(&mut self, bbid: BBId) -> Option<Mut<C>> {
        let Some(e) = self.try_bbid(bbid) else {
            return None;
        };
        self.get_mut::<C>(e)
    }

    fn try_entities_by_bbid_vec(&mut self, bbids: &[BBId]) -> Result<Vec<Entity>, BbidWorldError> {
        let mut q_bbids = self.query::<(Entity, &BBId)>();

        let result: Result<Vec<Entity>, BbidWorldError> = bbids
            .iter()
            .map(|bbid| {
                for (e, bbid_other) in q_bbids.iter(self) {
                    if bbid_other.eq(bbid) {
                        return Ok(e);
                    }
                }
                Err(BbidWorldError::NoEntityWithBbid(*bbid))
            })
            .collect();

        result
    }
}
