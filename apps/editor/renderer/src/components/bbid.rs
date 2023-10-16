use std::fmt::{Debug, Display};

use bevy::{ecs::world::EntityMut, prelude::*, reflect::Reflect, utils::Uuid};
use thiserror::Error;

#[derive(Component, Reflect, Eq, PartialEq, Hash, Copy, Clone)]
#[reflect(Component)]
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
    fn entity_id_by_bbid(&mut self, bbid: BBId) -> Entity;
    fn get_entity_id_by_bbid(&mut self, bbid: BBId) -> Option<Entity>;

    fn try_entities_by_bbid_vec(
        &mut self,
        bbids: &[BBId],
    ) -> Result<Vec<Entity>, BbidWorldError>;
}

impl BBIdUtils for World {
    fn entity_id_by_bbid(&mut self, bbid: BBId) -> Entity {
        match self.get_entity_id_by_bbid(bbid) {
            Some(entity) => entity,
            None => panic!(
                "entity_by_uuid: Could not find entity with uuid {:?}",
                bbid.0
            ),
        }
    }

    fn get_entity_id_by_bbid(&mut self, bbid: BBId) -> Option<Entity> {
        self.query::<(Entity, &BBId)>()
            .iter(self)
            .find(|(_entity, id)| (*id).eq(&bbid))
            .map(|(e, _)| e)
    }

    fn try_entities_by_bbid_vec(
        &mut self,
        bbids: &[BBId],
    ) -> Result<Vec<Entity>, BbidWorldError> {
        let mut q_bbids = self.query::<(Entity, &BBId)>();

        let result: Result<Vec<Entity>, BbidWorldError> = bbids.iter().map(|bbid| {
            for (e, bbid_other) in q_bbids.iter(self) {
                if bbid_other.eq(bbid) {
                    return Ok(e);
                }
            }
            Err(BbidWorldError::NoEntityWithBbid(*bbid))
        }).collect();

        result
    }
}
