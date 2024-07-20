use bevy_app::App;
use bevy_ecs::{
    prelude::{Bundle, Component},
    reflect::{ReflectBundle, ReflectComponent},
};
use bevy_reflect::Reflect;
use bevy_spts_uid::{Uid, UidRegistry};

use bevy_spts_changeset::{commands_ext::WorldChangesetExt, events::ChangesetEvent, resource::ChangesetResource};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Comp1(usize);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Comp2;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Comp3;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Comp4;

#[derive(Bundle, Reflect, Default)]
#[reflect(Bundle)]
struct Bundle1 {
    comp1: Comp1,
    comp2: Comp2,
}

#[derive(Bundle, Reflect, Default)]
#[reflect(Bundle)]
struct Bundle2 {
    bundle1: Bundle1,
    comp4: Comp4,
}

#[derive(Default)]
struct MyChangeset;

fn build_app() -> (App, Uid) {
    let mut app = App::new();
    app.add_event::<ChangesetEvent>();
    app.insert_resource(UidRegistry::default());
    app.insert_resource(ChangesetResource::<MyChangeset>::new());
    app.register_type::<Comp1>();
    app.register_type::<Comp2>();
    app.register_type::<Comp3>();
    app.register_type::<Comp4>();
    app.register_type::<Bundle1>();
    app.register_type::<Bundle2>();

    let uid = Uid::default();
    app.world_mut().spawn(uid);

    (app, uid)
}

#[test]
fn insert_single_component() {
    let (mut app, uid) = build_app();
    let world = app.world_mut();

    let mut changeset = world.changeset();
    let uid = changeset.entity(uid).insert(Comp1(0)).uid();
    let changeset = changeset.build();

    ChangesetResource::<MyChangeset>::context_scope(world, |world, cx| {
        let entity = uid.entity(world).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());

        let undo = changeset.apply(world, cx).unwrap();

        assert!(world.get::<Comp1>(entity).is_some());

        undo.apply(world, cx).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
    });
}

#[test]
fn insert_tuple_bundle() {
    let (mut app, uid) = build_app();
    let world = app.world_mut();

    let mut changeset = world.changeset();
    let uid = changeset.entity(uid).insert((Comp1(0), Comp2)).uid();
    let changeset = changeset.build();

    ChangesetResource::<MyChangeset>::context_scope(world, |world, cx| {
        let entity = uid.entity(world).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
        assert!(world.get::<Comp2>(entity).is_none());

        let undo = changeset.apply(world, cx).unwrap();

        assert!(world.get::<Comp1>(entity).is_some());
        assert!(world.get::<Comp2>(entity).is_some());

        undo.apply(world, cx).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
        assert!(world.get::<Comp2>(entity).is_none());
    });
}

#[test]
fn insert_nested_struct_bundle_in_tuple_bundle() {
    let (mut app, uid) = build_app();
    let world = app.world_mut();

    let mut changeset = world.changeset();
    let uid = changeset.entity(uid).insert((Comp3, Comp4, Bundle1::default())).uid();
    let changeset = changeset.build();

    ChangesetResource::<MyChangeset>::context_scope(world, |world, cx| {
        let entity = uid.entity(world).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
        assert!(world.get::<Comp2>(entity).is_none());

        let undo = changeset.apply(world, cx).unwrap();

        assert!(world.get::<Comp1>(entity).is_some());
        assert!(world.get::<Comp2>(entity).is_some());

        undo.apply(world, cx).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
        assert!(world.get::<Comp2>(entity).is_none());
    });
}

#[test]
fn insert_struct_bundle() {
    let (mut app, uid) = build_app();
    let world = app.world_mut();

    let mut changeset = world.changeset();
    let uid = changeset.entity(uid).insert(Bundle1::default()).uid();
    let changeset = changeset.build();

    ChangesetResource::<MyChangeset>::context_scope(world, |world, cx| {
        let entity = uid.entity(world).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
        assert!(world.get::<Comp2>(entity).is_none());

        let undo = changeset.apply(world, cx).unwrap();

        assert!(world.get::<Comp1>(entity).is_some());
        assert!(world.get::<Comp2>(entity).is_some());

        undo.apply(world, cx).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
        assert!(world.get::<Comp2>(entity).is_none());
    });
}

#[test]
fn insert_nested_struct_bundle_in_struct_bundle() {
    let (mut app, uid) = build_app();
    let world = app.world_mut();

    let mut changeset = world.changeset();
    let uid = changeset.entity(uid).insert((Comp1(0), Comp2, Bundle1::default())).uid();
    let changeset = changeset.build();

    ChangesetResource::<MyChangeset>::context_scope(world, |world, cx| {
        let entity = uid.entity(world).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
        assert!(world.get::<Comp2>(entity).is_none());

        let undo = changeset.apply(world, cx).unwrap();

        assert!(world.get::<Comp1>(entity).is_some());
        assert!(world.get::<Comp2>(entity).is_some());

        undo.apply(world, cx).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
        assert!(world.get::<Comp2>(entity).is_none());
    });
}
