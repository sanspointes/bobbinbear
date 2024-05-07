use bevy_ecs::world::World;

use crate::builder::ChangesetCommands;

pub trait WorldChangesetExt {
    fn changeset(&mut self) -> ChangesetCommands;
}

impl WorldChangesetExt for World {
    fn changeset(&mut self) -> ChangesetCommands {
        ChangesetCommands::new(self)
    }
}
