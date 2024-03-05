use bevy::{
    math::{vec2, vec3},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_spts_vector_graphic::{commands_ext::VectorGraphicCommandsExt, prelude::*};
use bevy_xpbd_2d::{
    components::{MassPropertiesBundle, RigidBody}, constraints::{DistanceJoint, Joint}, math::Vector, plugins::{collision::Collider, PhysicsPlugins}, resources::{Gravity, SubstepCount}
};

pub fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(VectorGraphicPlugin);
    app.add_plugins(PhysicsPlugins::default());

    app.insert_resource(SubstepCount(50))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0));

    app.add_systems(Startup, setup);

    app.register_type::<Endpoint>()
        .register_type::<Edge>()
        .register_type::<EdgeVariant>();

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(500., 25.)).into(),
        transform: Transform::default()
            .with_translation(vec3(0., -300., 0.)),
        material: materials.add(Color::PURPLE),
        ..default()
    }).insert((
            RigidBody::Static,
            Collider::rectangle(500.0, 25.0),
        ));

    let vector_graphic = commands
        .spawn(VectorGraphicBundle::default())
        .insert(SpatialBundle::default())
        .insert((
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::circle(10.), 1.0),
            Collider::circle(10.),
        ))
        .insert(materials.add(Color::WHITE))
        .id();

    const STEPS: usize = 12;
    const JOINT_LENGTH: f32 = 2. * std::f32::consts::PI * 100. / STEPS as f32;

    let mut first_endpoint = None;
    let mut prev_endpoint = None;

    for i in 0..STEPS {
        let t = i as f32 / STEPS as f32 * std::f32::consts::PI * 2.;

        let endpoint = commands
            .spawn(EndpointBundle::default().with_translation(vec3(
                t.sin() * 100.,
                t.cos() * 100.,
                0.,
            )))
            .set_parent(vector_graphic)
            .insert((
                RigidBody::Dynamic,
                MassPropertiesBundle::new_computed(&Collider::circle(10.), 1.0),
                Collider::circle(10.),
            ))
            .id();

        if first_endpoint.is_none() {
            first_endpoint = Some(endpoint);
        }

        commands.spawn(
            DistanceJoint::new(vector_graphic, endpoint)
                .with_local_anchor_1(Vector::ZERO)
                .with_local_anchor_2(Vector::ZERO)
                .with_rest_length(100.0)
                .with_linear_velocity_damping(0.1)
                .with_angular_velocity_damping(1.0)
                .with_compliance(0.000002),
        );

        if let Some(prev_endpoint) = prev_endpoint {
            commands
                .spawn_edge(EdgeVariant::Line, prev_endpoint, endpoint)
                .set_parent(vector_graphic);

            commands.spawn(
                DistanceJoint::new(endpoint, prev_endpoint)
                    .with_rest_length(JOINT_LENGTH)
                    .with_linear_velocity_damping(0.1)
                    .with_angular_velocity_damping(1.0)
                    .with_compliance(0.000002),
            );
        }

        prev_endpoint = Some(endpoint);
    }

    if let (Some(first_endpoint), Some(last_endpoint)) = (first_endpoint, prev_endpoint) {
        commands
            .spawn_edge(EdgeVariant::Line, last_endpoint, first_endpoint)
            .set_parent(vector_graphic);
        commands.spawn(
            DistanceJoint::new(last_endpoint, first_endpoint)
                .with_rest_length(JOINT_LENGTH)
                .with_linear_velocity_damping(0.1)
                .with_angular_velocity_damping(1.0)
                .with_compliance(0.00000001),
        );
    }
}
