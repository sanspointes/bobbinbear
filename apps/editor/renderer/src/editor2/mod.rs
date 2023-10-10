pub mod camera;
pub mod constants;
pub mod frontend;
pub mod input;
pub mod msgs;
pub mod utils;
pub mod entities;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

pub use self::msgs::Message;
pub use self::msgs::ToolMessage;
use self::{
    camera::CameraPlugin,
    frontend::{FrontendMessage, FrontendReceiver, FrontendSender, InitModel},
    input::InputProcessorPlugin,
    msgs::{editor_msg_system, ToolControllerPlugin},
};

pub struct EditorPlugin;

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PostSet;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // 3rd Party Plugins
            .add_plugins(ShapePlugin)
            // Internals
            .add_event::<FrontendMessage>()
            .add_event::<Message>()
            .add_plugins((
                CameraPlugin,
                InputProcessorPlugin,
                ToolControllerPlugin,
            ))
            .add_systems(PreUpdate, from_frontend_system)
            .add_systems(Update, editor_msg_system);
        if let Some(frontend_sender) = app.world.get_resource_mut::<FrontendSender>() {
            frontend_sender
                .0
                .send(FrontendMessage::Init(InitModel))
                .expect("Editor: Failed to send init message.");
        }
    }
}

/// Receives messages from the javascript layer and passes them into the main event handler
///
/// * `frontend_receiver`:
/// * `message_writer`:
fn from_frontend_system(
    frontend_receiver: ResMut<FrontendReceiver>,
    mut msg_writer: EventWriter<Message>,
) {
    let receiver = &frontend_receiver.0;
    if let Ok(msg) = receiver.try_recv() {
        msg_writer.send(msg);
    }
}
