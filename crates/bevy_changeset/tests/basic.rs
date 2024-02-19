use bevy_app::App;
use bevy_changeset::{commands_ext::WorldChangesetExt, uid::Uid};
use bevy_ecs::component::Component;

#[test]
pub fn spawn_change_spawns_entity() {
    let mut app = App::new();
    let world = &mut app.world;
    // Spawn empty
    let mut builder = world.changeset();
    let uid = builder.spawn_empty().uid();
    // Apply to world
    let undo = builder.build().apply(world).unwrap();
    let in_world = world.query::<&Uid>().single(world);
    assert_eq!(uid, *in_world);
    // Undo
    undo.apply(world).unwrap();
    world.query::<&Uid>().get_single(world).unwrap_err();
}

#[derive(Component, Debug, Clone)]
struct MyTag;
#[derive(Component, Debug, Clone)]
struct MyOtherTag;

#[test]
pub fn spawns_with_components() {
    let mut app = App::new();
    let world = &mut app.world;

    let mut builder = world.changeset();
    let uid = builder.spawn_empty().insert(MyTag).uid();
    let undo1 = builder.build().apply(world).unwrap();

    world.query::<(&Uid, &MyTag)>().single(world);

    let mut builder = world.changeset();
    builder.entity(uid).insert(MyOtherTag);
    let undo2 = builder.build().apply(world).unwrap();

    world.query::<(&Uid, &MyTag, &MyOtherTag)>().single(world);

    undo2.apply(world).unwrap();

    world.query::<(&Uid, &MyTag)>().single(world);

    undo1.apply(world).unwrap();

    world
        .query::<(&Uid, &MyTag)>()
        .get_single(world)
        .unwrap_err();
}

#[test]
pub fn respawns_despawned_components() {
    let mut app = App::new();
    let world = &mut app.world;

    let mut builder = world.changeset();
    let uid = builder.spawn_empty().insert(MyTag).uid();
    builder.build().apply(world).unwrap();

    let mut builder = world.changeset();
    builder.despawn(uid);
    let undo = builder.build().apply(world).unwrap();

    world.query::<&Uid>().get_single(world).unwrap_err();

    undo.apply(world).unwrap();

    world.query::<(&Uid, &MyTag)>().get_single(world).expect("Did not respawn the `MyTag` component with error");
}
