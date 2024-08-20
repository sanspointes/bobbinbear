use bevy::ecs::world::{FromWorld, World};
use bevy_spts_changeset::builder::MultiChangesetBuilder;

/// Builder for complex, undo-able changes to the world.
///
/// * `multi_commands`: 
pub struct BBChangeset(MultiChangesetBuilder);

impl BBChangeset {
    pub fn new(world: &mut World) -> Self {
        Self(MultiChangesetBuilder::from_world(world))
    }
}
