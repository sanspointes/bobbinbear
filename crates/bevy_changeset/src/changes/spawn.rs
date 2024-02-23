use std::fmt::Debug;

use bevy_ecs::world::World;

use crate::{error::ChangeError, uid::Uid};

use super::{Change, ChangeIter, IntoChangeIter};

#[derive(Debug)]
pub struct SpawnChange {
    uid: Uid,
}
impl SpawnChange {
    pub fn new(uid: Uid) -> Self {
        Self { uid }
    }
}
impl Change for SpawnChange {
    fn apply(&self, world: &mut World) -> Result<ChangeIter, ChangeError> {
        world.spawn(self.uid);
        Ok(DespawnChange::new(self.uid).into_change_iter())
    }
}

#[derive(Debug)]
pub struct DespawnChange {
    uid: Uid,
}
impl DespawnChange {
    pub fn new(uid: Uid) -> Self {
        Self { uid }
    }
}
impl Change for DespawnChange {
    fn apply(&self, world: &mut World) -> Result<ChangeIter, ChangeError> {
        let entity = self
            .uid
            .entity(world)
            .ok_or(ChangeError::NoEntity(self.uid))?;

        let entity_ref = world.entity(entity);
        let archetype = entity_ref.archetype();

        let mut inverse = vec![];
        for component_id in archetype.components() {
            let v = world.get_by_id(entity, component_id).unwrap();
        }

        world.despawn(entity);
        Ok(SpawnChange::new(self.uid).into_change_iter())
    }
}
