use bevy::{app::{App, Plugin}, ecs::schedule::SystemSet, input::InputPlugin};

mod types;
mod input;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolSet {
    Input,
    ToolHandler,
}

pub struct BobbinToolsPlugin;

impl Plugin for BobbinToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin);
    }
}
