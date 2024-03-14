use bevy::{math::vec2, prelude::*};

use bevy_spts_uid::extension::EntityCommandsExt;
use bevy_spts_vector_graphic::{commands_ext::VectorGraphicWorldExt, prelude::*};

pub fn build_endpoints(world: &mut World) -> (Entity, (bevy_spts_uid::Uid, bevy_spts_uid::Uid, bevy_spts_uid::Uid, bevy_spts_uid::Uid)) {
    let vg = world.spawn(VectorGraphicBundle::default()).id();
    let p0 = world
        .spawn((TransformBundle::default(), Endpoint::default()))
        .set_parent(vg)
        .uid();
    let p1 = world
        .spawn((
            TransformBundle {
                local: Transform {
                    translation: Vec3::new(100., 0., 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Endpoint::default(),
        ))
        .set_parent(vg)
        .uid();

    let p2 = world
        .spawn((
            TransformBundle {
                local: Transform {
                    translation: Vec3::new(100., 100., 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Endpoint::default(),
        ))
        .set_parent(vg)
        .uid();

    let p3 = world
        .spawn((
            TransformBundle {
                local: Transform {
                    translation: Vec3::new(100., 100., 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Endpoint::default(),
        ))
        .set_parent(vg)
        .uid();

    (vg, (p0, p1, p2, p3))
}

pub fn build_box(world: &mut World) -> (Entity, (bevy_spts_uid::Uid, bevy_spts_uid::Uid, bevy_spts_uid::Uid, bevy_spts_uid::Uid)) {
    let (vg, (p0, p1, p2, p3)) = build_endpoints(world);
    world.spawn_edge(EdgeVariant::Line, p0, p1).set_parent(vg);
    world
        .spawn_edge(
            EdgeVariant::Quadratic {
                ctrl1: vec2(75., 50.),
            },
            p1,
            p2,
        )
        .set_parent(vg);
    world
        .spawn_edge(
            EdgeVariant::Cubic {
                ctrl1: vec2(75., 75.),
                ctrl2: vec2(25., 125.),
            },
            p2,
            p3,
        )
        .set_parent(vg);
    world.spawn_edge(EdgeVariant::Line, p3, p0).set_parent(vg);

    (vg, (p0, p1, p2, p3))
}

pub fn retry(attempts: usize, method: impl Fn()) {
    for _ in 0..attempts {
        (method)();
    }
}

#[test]
pub fn it_works_with_a_closed_shape() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_plugins(AssetPlugin::default());

    app.add_plugins(VectorGraphicPlugin);
    app.init_resource::<Assets<Mesh>>();

    app.update();

    let world = &mut app.world;
    let (vg, _) = build_box(world);
    world
        .entity_mut(vg)
        .insert((FillOptions::default(), StrokeOptions::default()));

    app.update();

    let world = &mut app.world;
    let result = world.query::<&VectorGraphicPathStorage>().single(world);
    assert!(matches!(result, VectorGraphicPathStorage::Calculated(_)));
}

#[test]
pub fn it_works_with_an_open_shape() {
    retry(10, || {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(AssetPlugin::default());

        app.add_plugins(VectorGraphicPlugin);
        app.init_resource::<Assets<Mesh>>();

        app.update();

        let world = &mut app.world;
        let (vg, (p0, p1, p2, _)) = build_endpoints(world);
        world.spawn_edge(EdgeVariant::Line, p0, p1).set_parent(vg);
        world
            .spawn_edge(
                EdgeVariant::Quadratic {
                    ctrl1: vec2(75., 50.),
                },
                p1,
                p2,
            )
            .set_parent(vg);

        world
            .entity_mut(vg)
            .insert((FillOptions::default(), StrokeOptions::default()));

        app.update();

        let world = &mut app.world;
        let result = world.query::<&VectorGraphicPathStorage>().single(world);
        assert!(matches!(result, VectorGraphicPathStorage::Calculated(_)));
    });
}
