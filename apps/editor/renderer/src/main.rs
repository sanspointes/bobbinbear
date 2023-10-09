mod messages;
mod resources;
mod systems;
mod types;

mod editor;
mod plugins;

use bevy::prelude::*;
use crossbeam_channel::unbounded;

use editor::EditorPlugin;
use messages::{frontend::FrontendMsg, Msg};
use resources::{FrontendReceiver, FrontendSender};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    // TODO add webview gui
    let (_from_frontend_sender, from_frontend_receiver) = unbounded::<Msg>();
    let (to_frontend_sender, _to_frontend_receiver) = unbounded::<FrontendMsg>();
    let mut app = App::new();
    app.insert_resource(TaskPoolOptions::with_num_threads(1))
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_event::<Msg>()
        // .register_type::<Fill>()
        // .register_type::<Stroke>()
        .insert_resource(FrontendReceiver(from_frontend_receiver))
        .insert_resource(FrontendSender(to_frontend_sender))
        .add_plugins(DefaultPlugins.build().set(WindowPlugin {
            primary_window: Some(Window {
                // title: "Bevy game".to_string(), // ToDo
                // resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(EditorPlugin);
    // .add_plugin(OverlayPlugin {
    //     font_size: 14.,
    //     ..Default::default()
    // });

    // #[cfg(debug_assertions)]
    // {
    //     app.add_plugin(WorldInspectorPlugin::new());
    //     app.register_type::<ShapeMaterial>();
    //     app.register_type::<PathSegment>();
    //     app.register_type::<HoveredState>();
    //     app.register_type::<SelectedState>();
    //     app.register_type::<Bounded>();
    //     app.register_type::<Ordered>();
    //     app.register_type::<HasFocusRing>();
    //     app.register_type::<FocusRingTag>();
    // }

    app.run();
}
