use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*};

#[cfg(feature = "debug_text")]
use bevy_debug_text_overlay::OverlayPlugin;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    components::{
        bbid::BBId,
        scene::{BBNode, BBObject},
    },
    msgs::{
        api::ApiMsg, cmds::CmdMsgPlugin, effect::EffectMsg, sys_msg_handler, Msg, MsgPlugin,
    },
    plugins::{
        bounds_2d_plugin::Bounds2DPlugin,
        input_plugin::{InputMessage, InputPlugin},
        inspect_plugin::InspectPlugin,
        screen_space_root_plugin::{ScreenSpaceRoot, ScreenSpaceRootPlugin},
        selection_plugin::SelectionPlugin,
        vector_graph_plugin::{BuildShapes, VectorGraphPlugin},
        // inspect_plugin::InspectPlugin,
    },
    systems::camera::sys_setup_camera, shared::{CachedMeshes, sys_setup_cached_meshes},
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
    app.add_plugins(OverlayPlugin {
        font_size: 12.0,
        ..default()
    });

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
            // Internals
            .add_event::<ApiMsg>()
            .add_event::<Msg>()
            .add_event::<EffectMsg>()
            // Internal generic plugins
            .add_plugins((
                InputPlugin,
                SelectionPlugin,
                ScreenSpaceRootPlugin,
                Bounds2DPlugin,
            ))
            // Internal App Logic plugins
            .add_plugins(MsgPlugin)
            .add_plugins((InspectPlugin, VectorGraphPlugin))
            .configure_sets(
                Update,
                (EditorSet::PreMsgs, EditorSet::Msgs, EditorSet::PostMsgs).chain(),
            )
            .configure_sets(PostUpdate, EditorSet::PostPlugins.after(BuildShapes))
            .add_systems(PreStartup, (sys_setup_camera, sys_setup_cached_meshes))
            .add_systems(
                Update,
                sys_handle_pre_editor_msgs.in_set(EditorSet::PreMsgs),
            )
            .add_systems(Update, sys_msg_handler.in_set(EditorSet::Msgs))
            .insert_resource(CachedMeshes::default())

            .register_type::<BBId>()
            .register_type::<BBObject>()
            .register_type::<BBNode>()
            .register_type::<ScreenSpaceRoot>();

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
