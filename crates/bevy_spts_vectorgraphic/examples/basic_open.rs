use bevy::{math::vec3, prelude::*, sprite::MaterialMesh2dBundle};
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
        .register_type::<StrokeOptions>()
        .register_type::<FillOptions>();

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
        .remove::<FillOptions>()
        .insert(SpatialBundle::default())
        .insert(materials.add(Color::WHITE))
        .id();

    let p0 = commands
        .spawn(EndpointBundle::default())
        .set_parent(vector_graphic)
        .uid();
    let p1 = commands
        .spawn(EndpointBundle::default().with_translation(vec3(50., 20., 0.)))
        .set_parent(vector_graphic)
        .uid();
    let p2 = commands
        .spawn(EndpointBundle::default().with_translation(vec3(100., 0., 0.)))
        .set_parent(vector_graphic)
        .uid();
    let p3 = commands
        .spawn(EndpointBundle::default().with_translation(vec3(120., 50., 0.)))
        .set_parent(vector_graphic)
        .uid();
    let p4 = commands
        .spawn(EndpointBundle::default().with_translation(vec3(100., 100., 0.)))
        .set_parent(vector_graphic)
        .uid();
    let p5 = commands
        .spawn(EndpointBundle::default().with_translation(vec3(50., 120., 0.)))
        .set_parent(vector_graphic)
        .uid();
    let p6 = commands
        .spawn(EndpointBundle::default().with_translation(vec3(0., 100., 0.)))
        .set_parent(vector_graphic)
        .uid();
    let p7 = commands
        .spawn(EndpointBundle::default().with_translation(vec3(20., 50., 0.)))
        .set_parent(vector_graphic)
        .uid();
    println!("Endpoints {:?}", [p0, p1, p2, p3, p4, p5, p6, p7]);

    let e0 = commands
        .spawn_edge(EdgeVariant::Line, p4, p5)
        .set_parent(vector_graphic)
        .id();
    let e1 = commands
        .spawn_edge(EdgeVariant::Line, p5, p6)
        .set_parent(vector_graphic)
        .id();
    let e2 = commands
        .spawn_edge(EdgeVariant::Line, p6, p7)
        .set_parent(vector_graphic)
        .id();
    let e3 = commands
        .spawn_edge(EdgeVariant::Line, p7, p0)
        .set_parent(vector_graphic)
        .id();
    let e4 = commands
        .spawn_edge(EdgeVariant::Line, p0, p1)
        .set_parent(vector_graphic)
        .id();
    let e5 = commands
        .spawn_edge(EdgeVariant::Line, p1, p2)
        .set_parent(vector_graphic)
        .id();
    println!("Edges {:?}", [e0, e1, e2, e3, e4, e5]);
}
