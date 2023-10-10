pub mod camera;
pub mod constants;
pub mod entities;
pub mod frontend;
pub mod input;
pub mod msgs;
pub mod systems;
pub mod utils;

use bevy::{prelude::*, transform::TransformSystem};
use bevy_prototype_lyon::{plugin::BuildShapes, prelude::ShapePlugin};

use self::entities::vector::debug_vector_node_order;
pub use self::msgs::Message;
pub use self::msgs::DocMessage;
pub use self::msgs::ToolMessage;
use self::{
    camera::CameraPlugin,
    frontend::{FrontendMessage, FrontendReceiver, FrontendSender, InitModel},
    input::InputProcessorPlugin,
    msgs::{editor_msg_system, DocumentPlugin, ToolControllerPlugin},
    systems::{calculate_vector_object_bounds, focus_rings::{update_focus_ring_styles, update_focus_ring_positions}}, entities::vector::{handle_vec_node_moved, handle_vec_object_node_updated, VectorResource},
};

pub struct EditorPlugin {}

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

            .add_plugins((CameraPlugin, InputProcessorPlugin, ToolControllerPlugin, DocumentPlugin))

            .add_systems(PreUpdate, from_frontend_system)
            .add_systems(Update, editor_msg_system)


            // Editable Vector handle changes to nodes. 
            .insert_resource(VectorResource::new())
            .add_systems(Update, (
                handle_vec_node_moved.after(editor_msg_system),
                handle_vec_object_node_updated.after(handle_vec_node_moved)
            ))

            // Post update systems
            .add_systems(PostUpdate, (
                update_focus_ring_styles.before(TransformSystem::TransformPropagate),
                calculate_vector_object_bounds.after(BuildShapes),
                update_focus_ring_positions,
                debug_vector_node_order,
            ))
        ;
        if let Some(frontend_sender) = app.world.get_resource_mut::<FrontendSender>() {
            frontend_sender.0.send(FrontendMessage::Init(InitModel)).expect("Editor: Failed to send init message.");
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
