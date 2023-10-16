use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};
use bevy_prototype_lyon::{prelude::*, plugin::BuildShapes};

#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    components::{bbid::BBId, scene::BBObject},
    msgs::{cmds::CmdMsgPlugin, frontend::FrontendMsg, sys_msg_handler, Message, ToolMsgPlugin},
    plugins::{
        bounds_2d_plugin::Bounds2DPlugin,
        input_plugin::{InputMessage, InputPlugin},
        screen_space_root_plugin::ScreenSpaceRootPlugin,
        selection_plugin::SelectionPlugin,
    },
    systems::camera::sys_setup_camera,
    utils::reflect_shims::{ReflectableFill, ReflectablePath}, api::ApiToEditorReceiver,
};

pub fn start_bobbin_bear(default_plugins: PluginGroupBuilder) -> App {
    #[cfg(debug_assertions)]
    let default_plugins = {

        #[cfg(all(not(feature = "trace_bevy"), not(feature = "debug_trace")))]
        let default_plugins = default_plugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            filter: "debug,wgpu_core=warn,wgpu_hal=warn,naga=warn,bevy_render=info,bevy_app=info,mygame=debug".into(),
        });

        #[cfg(all(feature = "trace_bey", not(feature = "debug_trace")))]
        let default_plugins = default_plugins.set(LogPlugin {
            level: bevy::log::Level::TRACE,
            ..Default::default()
        });

        #[cfg(feature = "debug_trace")]
        let default_plugins = default_plugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            filter: "debug,wgpu_core=warn,wgpu_hal=warn,naga=warn,bevy_app=warn,bevy_render=warn,bevy_ecs=warn,bevy_core_pipeline=warn,bevy_mod_raycast=warn,renderer=debug".into(),
        });

        default_plugins
    };

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    let default_plugins = default_plugins.set(LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "info,wgpu_core=warn,wgpu_hal=warn,naga=info,bevy_app=info,bevy_render=info".into(),
    });

    let mut app = App::new();

    #[cfg(feature = "debug_trace")]
    app.insert_resource(TaskPoolOptions::with_num_threads(1));

    app.add_plugins(default_plugins).add_plugins(EditorPlugin);

    #[cfg(feature = "inspector")]
    app.add_plugins(WorldInspectorPlugin::default());

    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(1., 0., 0.)));

    app
}

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub enum EditorSet {
    PreMsgs,
    Msgs,
    PostMsgs,
    PostPlugins,
}

/// The entyr point for the app, containing all non-platform specific behaviour.
pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // 3rd Party Plugins
            .add_plugins(ShapePlugin)
            // Internals
            .add_event::<FrontendMsg>()
            .add_event::<Message>()
            // Internal generic plugins
            .add_plugins((InputPlugin, SelectionPlugin, ScreenSpaceRootPlugin, Bounds2DPlugin))
            // Internal App Logic plugins
            .add_plugins((ToolMsgPlugin, CmdMsgPlugin))

            .configure_sets(Update, (EditorSet::PreMsgs, EditorSet::Msgs, EditorSet::PostMsgs).chain()) 
            .configure_set(PostUpdate, EditorSet::PostPlugins.after(BuildShapes))

            .add_systems(PreStartup, sys_setup_camera)
            .add_systems(Update, sys_handle_pre_editor_msgs.in_set(EditorSet::PreMsgs))
            .add_systems(Update, sys_msg_handler.in_set(EditorSet::Msgs))

            .register_type::<BBId>()
            .register_type::<BBObject>()

            .register_type::<ReflectablePath>() // Also need reflection shimed path for ser/de
            .register_type::<ReflectableFill>() // Also need reflection shimed path for ser/de
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
    api_receiver: ResMut<ApiToEditorReceiver>,
    mut msg_writer: EventWriter<Message>,
) {
    let _span = info_span!("sys_handle_pre_editor_msgs").entered();

    let receiver = &api_receiver.0;
    if let Ok(msg) = receiver.try_recv() {
        msg_writer.send(msg);
    }

    let msgs: Vec<Message> = input_msg_receiver
        .iter()
        .cloned()
        .map(|msg| {
            let msg: Message = msg.into();
            msg
        })
        .collect();
    msg_writer.send_batch(msgs);
}
