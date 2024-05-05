use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    utils::{HashSet, Uuid},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    #[cfg(not(feature = "graph"))]
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "graph")]
    app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>()); // disable LogPlugin so that you can pipe the output directly into `dot -Tsvg`

    bb_core::setup(&mut app);

    app.register_type::<HashSet<Entity>>()
        .register_type::<Uuid>();

    app.add_plugins((
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
    ));

    app.add_systems(Startup, setup);

    #[cfg(not(feature = "graph"))]
    app.run();
    #[cfg(feature = "graph")]
    {
        bevy_mod_debugdump::print_schedule_graph(&mut app, Update);
    }
}

pub fn setup(world: &mut World) {
    let result = bb_core::api::debug::DebugApi::spawn_circle(world);
    println!("Spawned circle {result:?}");
    let result = bb_core::api::debug::DebugApi::spawn_box(world);
    println!("Spawned box {result:?}");
}
