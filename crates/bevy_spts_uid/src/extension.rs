use bevy_ecs::{system::EntityCommands, world::EntityWorldMut, entity::Entity};

use crate::Uid;

pub trait EntityCommandsExt {
    /// Generates a new Uid and inserts it on this entity.
    /// Warn: Will override pre-existing uids.
    fn uid(&mut self) -> Uid;

    /// Generates a new Uid and inserts it on this entity.
    /// Warn: Will override pre-existing uids.
    /// Convenience method to return the entity id and uid at same time.
    fn id_uid(&mut self) -> (Entity, Uid);
} 

impl EntityCommandsExt for EntityCommands<'_> {
    fn uid(&mut self) -> Uid {
        let uid = Uid::default();
        self.insert(uid);
        uid
    }

    fn id_uid(&mut self) -> (Entity, Uid) {
        (self.id(), self.uid())
    }
}

impl EntityCommandsExt for EntityWorldMut<'_> {
    fn uid(&mut self) -> Uid {
        let uid = Uid::default();
        self.insert(uid);
        uid
    }

    fn id_uid(&mut self) -> (Entity, Uid) {
        (self.id(), self.uid())
    }
}
