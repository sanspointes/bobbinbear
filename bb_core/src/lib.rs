//! Displays a single [`Sprite`], created from an image.
mod ipc;

use bevy::prelude::*;

pub fn init() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/bevy_bird_dark.png"),
        ..default()
    });
}
