use bevy_app::App;
use bevy_changeset::{commands_ext::WorldChangesetExt, resource::ChangesetResource};
use bevy_ecs::{component::Component, world::FromWorld, reflect::ReflectComponent};
use bevy_reflect::{Reflect, TypeRegistry};
use bevy_scene::SceneFilter;
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
        let undo_change = do_change.apply(world, cx).unwrap();

        let in_world = world.query::<&Uid>().single(world);
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

    let mut changeset_type_registry = TypeRegistry::new();
    changeset_type_registry.register::<MyTag>();
    changeset_type_registry.register::<MyOtherTag>();
    app.insert_resource(ChangesetResource::<DefaultChangesetTag>::new(changeset_type_registry));

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

    let mut changeset_type_registry = TypeRegistry::new();
    changeset_type_registry.register::<MyTag>();
    changeset_type_registry.register::<MyOtherTag>();
    app.insert_resource(ChangesetResource::<DefaultChangesetTag>::new(changeset_type_registry));

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

        world.query::<(&Uid, &MyTag)>().get_single(world).expect("Did not respawn the `MyTag` component with error");
    });
}
