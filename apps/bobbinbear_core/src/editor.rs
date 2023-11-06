use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};
use bevy_prototype_lyon::{prelude::*, plugin::BuildShapes};

#[cfg(feature = "debug_text")]
use bevy_debug_text_overlay::OverlayPlugin;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    components::{bbid::BBId, scene::{BBObject, BBNode}, bbpath::{BBPathEvent, BBPath}},
    msgs::{cmds::CmdMsgPlugin, api::ApiMsg, sys_msg_handler, Msg, ToolMsgPlugin},
    plugins::{
        bounds_2d_plugin::Bounds2DPlugin,
        input_plugin::{InputMessage, InputPlugin},
        screen_space_root_plugin::{ScreenSpaceRootPlugin, ScreenSpaceRoot},
        selection_plugin::SelectionPlugin, inspect_plugin::InspectPlugin,
    },
    systems::camera::sys_setup_camera,
    utils::reflect_shims::{ReflectableFill, ReflectablePath},
};

pub fn start_bobbin_bear(default_plugins: PluginGroupBuilder) -> App {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("start_bobbin_bear").entered();

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

    app.add_plugins(default_plugins).add_plugins(EditorPlugin);

    #[cfg(feature = "inspector")]
    app.add_plugins(WorldInspectorPlugin::default());

    #[cfg(feature = "debug_text")]
    app.add_plugins(OverlayPlugin { font_size: 12.0, ..default() });

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
            .add_event::<ApiMsg>()
            .add_event::<Msg>()
            // Internal generic plugins
            .add_plugins((InputPlugin, SelectionPlugin, ScreenSpaceRootPlugin, Bounds2DPlugin))
            // Internal App Logic plugins
            .add_plugins((ToolMsgPlugin, CmdMsgPlugin, InspectPlugin))

            .configure_sets(Update, (EditorSet::PreMsgs, EditorSet::Msgs, EditorSet::PostMsgs).chain()) 
            .configure_set(PostUpdate, EditorSet::PostPlugins.after(BuildShapes))

            .add_systems(PreStartup, sys_setup_camera)
            .add_systems(Update, sys_handle_pre_editor_msgs.in_set(EditorSet::PreMsgs))
            .add_systems(Update, sys_msg_handler.in_set(EditorSet::Msgs))

            .register_type::<BBId>()
            .register_type::<BBObject>()
            .register_type::<BBPath>()
            .register_type::<BBNode>()
            .register_type::<ScreenSpaceRoot>()

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
    mut msg_writer: EventWriter<Msg>,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_handle_pre_editor_msgs").entered();

    let msgs: Vec<Msg> = input_msg_receiver
        .iter()
        .cloned()
        .map(|msg| {
            let msg: Msg = msg.into();
            msg
        })
        .collect();
    msg_writer.send_batch(msgs);
}
