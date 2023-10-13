use bevy::{prelude::*, utils::Uuid};
use bevy_prototype_lyon::prelude::*;

use crate::{
    msgs::{sys_msg_handler, frontend::FrontendMsg, Message, ToolControllerPlugin, cmds::CmdPlugin},
    plugins::input_plugin::{InputPlugin, InputMessage},
    wasm::FrontendReceiver, systems::camera::sys_setup_camera, components::{bbid::BBId, scene::BBObject}, utils::reflect_shims::ReflectablePath,
};

// pub use self::msgs::Message;
// pub use self::msgs::ToolMessage;
// use self::{
//     camera::CameraPlugin,
//     frontend::{FrontendMessage, FrontendReceiver, FrontendSender, InitModel},
//     input::InputProcessorPlugin,
//     msgs::{editor_msg_system, ToolControllerPlugin},
// };

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // 3rd Party Plugins
            .add_plugins(ShapePlugin)
            // Internals
            .add_event::<FrontendMsg>()
            .add_event::<Message>()
            .add_plugins(InputPlugin)
            .add_plugins((ToolControllerPlugin, CmdPlugin))

            .add_systems(Startup, sys_setup_camera)
            .add_systems(PreUpdate, sys_handle_pre_editor_msgs)
            .add_systems(Update, sys_msg_handler)

            .register_type::<BBId>()
            .register_type::<Uuid>()
            .register_type::<BBObject>()

            .register_type::<ReflectablePath>()
        ;


        // if let Some(frontend_sender) = app.world.get_resource_mut::<FrontendSender>() {
        //     frontend_sender
        //         .0
        //         .send(FrontendMessage::Init(InitModel))
        //         .expect("Editor: Failed to send init message.");
        // }
    }
}

/// Receives messages from various sources and integrates them into the Message event to be handled
/// by sys_msg_handler
fn sys_handle_pre_editor_msgs(
    mut input_msg_receiver: EventReader<InputMessage>,
    frontend_receiver: ResMut<FrontendReceiver>,
    mut msg_writer: EventWriter<Message>,
) {
    let receiver = &frontend_receiver.0;
    if let Ok(msg) = receiver.try_recv() {
        msg_writer.send(msg);
    }

    let msgs: Vec<Message> = input_msg_receiver.iter().cloned().map(|msg| {
        let msg: Message = msg.into();
        msg
    }).collect();
    msg_writer.send_batch(msgs);
}
