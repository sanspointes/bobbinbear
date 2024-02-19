use bevy_ecs::world::World;

use crate::{error::ChangeError, uid::Uid};

use super::Change;

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
    fn apply(&self, world: &mut World) -> Result<Box<dyn Change>, ChangeError> {
        world.spawn(self.uid);
        Ok(Box::new(DespawnChange::new(self.uid)))
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
    fn apply(&self, world: &mut World) -> Result<Box<dyn Change>, ChangeError> {
        let entity = self
            .uid
            .entity(world)
            .ok_or(ChangeError::NoEntity(self.uid))?;
        world.despawn(entity);
        Ok(Box::new(SpawnChange::new(self.uid)))
    }
}
