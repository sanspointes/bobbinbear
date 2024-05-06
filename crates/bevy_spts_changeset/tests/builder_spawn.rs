//! # builder_spawn
//!
//! Integration tests relating to spawn/despawn of entities using the changeset builder. 

use bevy_app::App;
use bevy_ecs::{
    prelude::{Bundle, Component},
    reflect::{ReflectBundle, ReflectComponent},
};
use bevy_reflect::Reflect;
use bevy_spts_uid::UidRegistry;

use bevy_spts_changeset::{commands_ext::WorldChangesetExt, events::ChangesetEvent, resource::ChangesetResource};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Comp1;

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

fn build_app() -> App {
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
    app
}

#[test]
fn spawn_single_component() {
    let app = build_app();
    let mut world = app.world;

    let mut changeset = world.changeset();
    let uid = changeset.spawn(Comp1).uid();
    let changeset = changeset.build();

    ChangesetResource::<MyChangeset>::context_scope(&mut world, |world, cx| {
        let undo = changeset.apply(world, cx).unwrap();

        let entity = uid.entity(world).unwrap();
        assert!(world.get::<Comp1>(entity).is_some());

        undo.apply(world, cx).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
    });
}

#[test]
fn spawn_tuple_bundle() {
    let app = build_app();
    let mut world = app.world;

    let mut changeset = world.changeset();
    let uid = changeset.spawn((Comp1, Comp2)).uid();
    let changeset = changeset.build();

    ChangesetResource::<MyChangeset>::context_scope(&mut world, |world, cx| {
        let undo = changeset.apply(world, cx).unwrap();

        let entity = uid.entity(world).unwrap();
        assert!(world.get::<Comp1>(entity).is_some());

        undo.apply(world, cx).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
    });
}

#[test]
fn spawn_nested_struct_bundle_in_tuple_bundle() {
    let app = build_app();
    let mut world = app.world;

    let mut changeset = world.changeset();
    let uid = changeset.spawn((Comp3, Comp4, Bundle1::default())).uid();
    let changeset = changeset.build();

    ChangesetResource::<MyChangeset>::context_scope(&mut world, |world, cx| {
        let undo = changeset.apply(world, cx).unwrap();

        let entity = uid.entity(world).unwrap();
        assert!(world.get::<Comp1>(entity).is_some());

        undo.apply(world, cx).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
    });
}

#[test]
fn spawn_struct_bundle() {
    let app = build_app();
    let mut world = app.world;

    let mut changeset = world.changeset();
    let uid = changeset.spawn(Bundle1::default()).uid();
    let changeset = changeset.build();

    ChangesetResource::<MyChangeset>::context_scope(&mut world, |world, cx| {
        let undo = changeset.apply(world, cx).unwrap();

        let entity = uid.entity(world).unwrap();
        assert!(world.get::<Comp1>(entity).is_some());

        undo.apply(world, cx).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
    });
}

#[test]
fn spawn_nested_struct_bundle_in_struct_bundle() {
    let app = build_app();
    let mut world = app.world;

    let mut changeset = world.changeset();
    let uid = changeset.spawn((Comp1, Comp2, Bundle1::default())).uid();
    let changeset = changeset.build();

    ChangesetResource::<MyChangeset>::context_scope(&mut world, |world, cx| {
        let undo = changeset.apply(world, cx).unwrap();

        let entity = uid.entity(world).unwrap();
        assert!(world.get::<Comp1>(entity).is_some());

        undo.apply(world, cx).unwrap();
        assert!(world.get::<Comp1>(entity).is_none());
    });
}

#[test]
fn adds_removes_from_uid_registry() {
    let app = build_app();
    let mut world = app.world;

    let mut change_commands = world.changeset();

    let uid = change_commands.spawn_empty().uid();

    let changeset = change_commands.build();

    ChangesetResource::<MyChangeset>::context_scope(&mut world, |world, cx| {
        assert!(uid.entity(world).is_none());

        let undo = changeset.apply(world, cx).unwrap();

        assert!(uid.entity(world).is_some());

        undo.apply(world, cx).unwrap();
        assert!(uid.entity(world).is_none());
    });
}
