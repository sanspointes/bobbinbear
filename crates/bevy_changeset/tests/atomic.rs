use bevy_app::App;
use bevy_changeset::{commands_ext::WorldChangesetExt, uid::Uid};
use bevy_ecs::component::Component;


#[derive(Component, Debug, Clone)]
struct MyTag;
#[derive(Component, Debug, Clone)]
struct MyOtherTag;

#[test]
pub fn it_rollsback_errored_changesets() {
    let mut app = App::new();
    let world = &mut app.world;

    // Ok changeset
    let mut builder = world.changeset();
    let e1 = builder.spawn_empty().insert(MyTag).insert(MyOtherTag).uid();
    builder.build().apply(world).unwrap();

    world.query::<(&Uid, &MyOtherTag)>().single(world);

    // Bad change
    let mut builder = world.changeset();
    let e2 = builder.spawn_empty().insert(MyTag).uid();
    builder.entity(Uid::new()).insert(MyTag);
    let err = builder.build().apply(world).unwrap_err();
    println!("{err:?}");
    let exists = world.query::<&Uid>().iter(world).any(|uid| *uid == e2);
    assert_eq!(exists, false);
    assert!(false);
}
