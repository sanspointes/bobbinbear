use bevy_ecs::world::World;

use crate::builder::ChangesetBuilder;

pub trait WorldChangesetExt {
    fn changeset(&mut self) -> ChangesetBuilder;
}

impl WorldChangesetExt for World {
    fn changeset(&mut self) -> ChangesetBuilder {
        ChangesetBuilder::new(self)
    }
}
