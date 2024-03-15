use bevy::{math::{vec2, vec3}, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_spts_uid::extension::EntityCommandsExt;
use bevy_spts_vectorgraphic::{commands_ext::VectorGraphicCommandsExt, prelude::*};

pub fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(VectorGraphicPlugin);
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_systems(Startup, setup);

    app.register_type::<Endpoint>()
        .register_type::<Edge>()
        .register_type::<EdgeVariant>()
        .register_type::<FillOptions>()
        .register_type::<StrokeOptions>()
    ;

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(Color::PURPLE),
        ..default()
    });

    let vector_graphic = commands
        .spawn(VectorGraphicBundle::default())
        .insert(SpatialBundle::default())
        .insert(materials.add(Color::WHITE))
        .id();

    let p0 = commands
        .spawn(EndpointBundle::default())
        .set_parent(vector_graphic)
        .uid();
    let p1 = commands
        .spawn(EndpointBundle::default().with_translation(vec3(100., 0., 0.)))
        .set_parent(vector_graphic)
        .uid();
    let p2 = commands
        .spawn(EndpointBundle::default().with_translation(vec3(100., 100., 0.)))
        .set_parent(vector_graphic)
        .uid();
    let p3 = commands
        .spawn(EndpointBundle::default().with_translation(vec3(0., 100., 0.)))
        .set_parent(vector_graphic)
        .uid();

    commands
        .spawn_edge(EdgeVariant::Quadratic { ctrl1: vec2(50., 50.) }, p0, p1)
        .set_parent(vector_graphic);
    commands
        .spawn_edge(EdgeVariant::Line, p1, p2)
        .set_parent(vector_graphic);
    commands
        .spawn_edge(EdgeVariant::Line, p2, p3)
        .set_parent(vector_graphic);
    commands
        .spawn_edge(EdgeVariant::Line, p3, p0)
        .set_parent(vector_graphic);
}
