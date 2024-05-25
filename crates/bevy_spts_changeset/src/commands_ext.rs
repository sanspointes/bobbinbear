use bevy_ecs::world::World;

use crate::builder::ChangesetCommands;

pub trait WorldChangesetExt {
    fn changeset(&self) -> ChangesetCommands;
}

impl WorldChangesetExt for World {
    fn changeset(&self) -> ChangesetCommands {
        ChangesetCommands::new(self)
    }
}
