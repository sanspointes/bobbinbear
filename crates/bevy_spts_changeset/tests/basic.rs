use bevy_app::App;
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::{Reflect, TypeRegistry};
use bevy_spts_changeset::{commands_ext::WorldChangesetExt, resource::ChangesetResource};
use bevy_spts_fragments::prelude::Uid;

#[derive(Default)]
struct DefaultChangesetTag;

#[test]
pub fn spawn_change_spawns_entity() {
    let mut app = App::new();
    app.insert_resource(ChangesetResource::<DefaultChangesetTag>::default());
    let world = &mut app.world;
    // Spawn empty
    let mut builder = world.changeset();
    let uid = builder.spawn_empty().uid();
    let do_change = builder.build();
    // Apply to world
    ChangesetResource::<DefaultChangesetTag>::context_scope(world, |world, cx| {
        let mut q_uid = world.query::<&Uid>();
        for v in q_uid.iter(world) {
            println!("Entity found {v}");
        }
        let undo_change = do_change.apply(world, cx).unwrap();

        let mut q_uid = world.query::<&Uid>();
        for v in q_uid.iter(world) {
            println!("Entity found {v}");
        }
        let in_world = q_uid.single(world);
        assert_eq!(uid, *in_world);

        // Undo
        undo_change.apply(world, cx).unwrap();
        world.query::<&Uid>().get_single(world).unwrap_err();
    });
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct MyTag;
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
struct MyOtherTag;

#[test]
pub fn spawns_with_components() {
    let mut app = App::new();
    app.register_type::<MyTag>();
    app.register_type::<MyOtherTag>();
    let mut res = ChangesetResource::<DefaultChangesetTag>::new();
    res.filter = res.filter.allow::<MyTag>().allow::<MyOtherTag>();
    app.insert_resource(res);

    let world = &mut app.world;

    ChangesetResource::<DefaultChangesetTag>::context_scope(world, |world, cx| {
        let mut builder = world.changeset();
        let uid = builder.spawn_empty().insert(MyTag).uid();

        let do_change = builder.build();

        let undo1 = do_change.apply(world, cx).unwrap();
        world.query::<(&Uid, &MyTag)>().single(world);

        let mut builder = world.changeset();
        builder.entity(uid).insert(MyOtherTag);
        let undo2 = builder.build().apply(world, cx).unwrap();

        world.query::<(&Uid, &MyTag, &MyOtherTag)>().single(world);

        undo2.apply(world, cx).unwrap();

        world.query::<(&Uid, &MyTag)>().single(world);

        undo1.apply(world, cx).unwrap();

        world
            .query::<(&Uid, &MyTag)>()
            .get_single(world)
            .unwrap_err();
    });
}

#[test]
pub fn respawns_despawned_components() {
    let mut app = App::new();

    app.register_type::<MyTag>();
    app.register_type::<MyOtherTag>();
    let mut res = ChangesetResource::<DefaultChangesetTag>::new();
    res.filter = res.filter.allow::<MyTag>().allow::<MyOtherTag>();
    app.insert_resource(res);

    let world = &mut app.world;

    ChangesetResource::<DefaultChangesetTag>::context_scope(world, |world, cx| {
        let mut builder = world.changeset();
        let uid = builder.spawn_empty().insert(MyTag).uid();
        builder.build().apply(world, cx).unwrap();

        let mut builder = world.changeset();
        builder.despawn(uid);
        let undo = builder.build().apply(world, cx).unwrap();

        world.query::<&Uid>().get_single(world).unwrap_err();

        undo.apply(world, cx).unwrap();

        world
            .query::<(&Uid, &MyTag)>()
            .get_single(world)
            .expect("Did not respawn the `MyTag` component with error");
    });
}
