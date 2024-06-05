use bevy_ecs::{reflect::AppTypeRegistry, world::World};

use crate::builder::ChangesetCommands;

pub trait WorldChangesetExt {
    fn changeset(&self) -> ChangesetCommands;
}

impl WorldChangesetExt for World {
    fn changeset(&self) -> ChangesetCommands {
        let app_type_registry = self.get_resource::<AppTypeRegistry>().unwrap();
        ChangesetCommands::new(app_type_registry)
    }
}
