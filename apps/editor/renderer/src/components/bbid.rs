use std::fmt::{Display, Debug};

use bevy::{ecs::world::EntityMut, prelude::*, reflect::Reflect, utils::Uuid};

#[derive(Component, Reflect, PartialEq, Copy, Clone)]
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
        write!(
            f,
            "BBID({}-{})",
            self.0[0],
            self.0[1]
        )
    }
}
impl Display for BBId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BBID({}-{})",
            self.0[0],
            self.0[1]
        )
    }
}

pub trait BBIdUtils {
    fn entity_id_by_bbid(&mut self, bbid: BBId) -> Entity;
    fn get_entity_id_by_bbid(&mut self, bbid: BBId) -> Option<Entity>;

    fn entity_by_bbid(&mut self, bbid: BBId) -> EntityRef<'_>;
    fn get_entity_by_bbid(&mut self, bbid: BBId) -> Option<EntityRef<'_>>;

    fn entity_mut_by_bbid(&mut self, bbid: BBId) -> EntityMut;
    fn get_entity_mut_by_bbid(&mut self, bbid: BBId) -> Option<EntityMut>;
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
            .find(|(entity, id)| (*id).eq(&bbid))
            .map(|(e, _)| e)
    }

    fn entity_by_bbid(&mut self, bbid: BBId) -> EntityRef<'_> {
        let entity = self.entity_id_by_bbid(bbid);
        self.entity(entity)
    }
    fn get_entity_by_bbid(&mut self, bbid: BBId) -> Option<EntityRef<'_>> {
        let entity = self.get_entity_id_by_bbid(bbid)?;
        self.get_entity(entity)
    }

    fn entity_mut_by_bbid(&mut self, bbid: BBId) -> EntityMut {
        let entity = self.entity_id_by_bbid(bbid);
        self.entity_mut(entity)
    }
    fn get_entity_mut_by_bbid(&mut self, bbid: BBId) -> Option<EntityMut> {
        let entity = self.entity_id_by_bbid(bbid);
        self.get_entity_mut(entity)
    }
}
