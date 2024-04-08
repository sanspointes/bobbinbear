use bevy::prelude::*;

mod components;

pub use components::*;

pub struct InspectingPlugin;

impl Plugin for InspectingPlugin {
    fn build(&self, _app: &mut App) {
        todo!()
    }
}
