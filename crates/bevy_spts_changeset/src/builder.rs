use std::{any::TypeId, fmt::Display, sync::Arc};

use bevy_ecs::{bundle::Bundle, component::Component, reflect::AppTypeRegistry, world::World};
use bevy_reflect::{FromReflect, Reflect};
use bevy_spts_fragments::prelude::{
    BundleFragment, BundleToFragment, ComponentFragment, EntityFragment, Uid,
};

use crate::{
    changes::{
        ApplyChange, Change, DespawnChange, DespawnRecursiveChange, InsertChange, RemoveChange,
        SetParentChange, SpawnChange,
    },
    resource::ChangesetContext,
};

#[derive(Debug)]
pub struct ChangeSet {
    changes: Vec<Arc<dyn Change>>,
}

impl Display for ChangeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ChangeSet [\n")?;
        for change in &self.changes {
            writeln!(f, "\t{change:?}\n")?;
        }
        writeln!(f, "]\n")
    }
}

impl ChangeSet {
    pub fn apply(
        self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<ChangeSet, anyhow::Error> {
        println!("Applying {} changes...", self.changes.len());

        let mut inverse_changes = vec![];

        for change in self.changes {
            let result = change.apply(world, cx);

            match result {
                Ok(inverse) => {
                    inverse_changes.push(inverse);
                    Ok(())
                }
                Err(reason) => Err(anyhow::anyhow!(
                    "Error while applying change {}.\n{}",
                    change.name(),
                    reason
                )),
            }?
        }

        inverse_changes.reverse();

        Ok(ChangeSet {
            changes: inverse_changes,
        })
    }
}

#[allow(dead_code)]
pub struct ChangesetBuilder<'w> {
    world: &'w mut World,
    changes: Vec<Arc<dyn Change>>,
}

impl<'w> ChangesetBuilder<'w> {
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world,
            changes: Vec::default(),
        }
    }

    pub fn push(&mut self, change: Arc<dyn Change>) {
        self.changes.push(change);
    }

    pub fn entity<'a>(&'a mut self, uid: Uid) -> EntityChangeset<'w, 'a> {
        EntityChangeset {
            target: uid,
            builder: self,
        }
    }

    pub fn spawn<'a, B: Bundle + Reflect + FromReflect>(
        &'a mut self,
        bundle: B,
    ) -> EntityChangeset<'w, 'a> {
        let uid = Uid::default();

        let bundle = {
            let type_registry = self.world.resource::<AppTypeRegistry>().read();
            bundle.to_fragment(&type_registry)
        };

        let change = SpawnChange::new(EntityFragment::new(uid, bundle), None);
        self.push(Arc::new(change));
        EntityChangeset {
            target: uid,
            builder: self,
        }
    }

    pub fn spawn_empty<'a>(&'a mut self) -> EntityChangeset<'w, 'a> {
        let uid = Uid::default();
        let entity_fragment = EntityFragment::new(uid, BundleFragment::new(vec![]));
        self.push(Arc::new(SpawnChange::new(entity_fragment, None)));
        EntityChangeset {
            target: uid,
            builder: self,
        }
    }
    pub fn despawn(&mut self, uid: Uid) -> &mut Self {
        self.push(Arc::new(DespawnChange::new(uid)));
        self
    }

    pub fn build(self) -> ChangeSet {
        ChangeSet {
            changes: self.changes,
        }
    }
}

pub struct EntityChangeset<'w, 'a> {
    pub(crate) target: Uid,
    pub(crate) builder: &'a mut ChangesetBuilder<'w>,
}

impl<'w, 'a> EntityChangeset<'w, 'a> {
    pub fn insert<C: Component + Reflect>(&mut self, component: C) -> &mut Self {
        self.builder.push(Arc::new(InsertChange::new(
            self.target,
            ComponentFragment::from_component::<C>(&component),
        )));
        self
    }
    pub fn apply<C: Component + Reflect>(&mut self, component: C) -> &mut Self {
        self.builder.push(Arc::new(ApplyChange::new(
            self.target,
            ComponentFragment::from_component::<C>(&component),
        )));
        self
    }
    pub fn remove<C: Component + Reflect>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<C>();
        self.builder
            .push(Arc::new(RemoveChange::new(self.target, type_id)));
        self
    }
    pub fn set_parent(&mut self, parent: Uid) -> &mut Self {
        self.builder
            .push(Arc::new(SetParentChange::parent(self.target, parent)));
        self
    }
    pub fn remove_parent(&mut self) -> &mut Self {
        self.builder
            .push(Arc::new(SetParentChange::unparent(self.target)));
        self
    }
    pub fn despawn(&mut self) -> &mut Self {
        self.builder.push(Arc::new(DespawnChange::new(self.target)));
        self
    }
    pub fn despawn_recursive(&mut self) -> &mut Self {
        self.builder
            .push(Arc::new(DespawnRecursiveChange::new(self.target)));
        self
    }

    pub fn uid(&self) -> Uid {
        self.target
    }
}

#[cfg(test)]
mod test_spawn {
    use bevy_app::App;
    use bevy_ecs::{
        prelude::{Bundle, Component},
        reflect::{ReflectBundle, ReflectComponent},
    };
    use bevy_reflect::Reflect;
    use bevy_spts_uid::UidRegistry;

    use crate::{commands_ext::WorldChangesetExt, events::ChangesetEvent, resource::ChangesetResource};

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
            changeset.apply(world, cx).unwrap()
        });

        let entity = uid.entity(&mut world).unwrap();
        assert!(world.get::<Comp1>(entity).is_some());
    }

    #[test]
    fn spawn_tuple_bundle() {
        let app = build_app();
        let mut world = app.world;

        let mut changeset = world.changeset();
        let uid = changeset.spawn((Comp1, Comp2)).uid();
        let changeset = changeset.build();

        ChangesetResource::<MyChangeset>::context_scope(&mut world, |world, cx| {
            changeset.apply(world, cx).unwrap();
        });

        let entity = uid.entity(&mut world).unwrap();
        assert!( world.get::<Comp1>(entity).is_some());
    }

    #[test]
    fn spawn_nested_struct_bundle_in_tuple_bundle() {
        let app = build_app();
        let mut world = app.world;

        let mut changeset = world.changeset();
        let uid = changeset.spawn((Comp3, Comp4, Bundle1::default())).uid();
        let changeset = changeset.build();

        ChangesetResource::<MyChangeset>::context_scope(&mut world, |world, cx| {
            let inverse = changeset.apply(world, cx).unwrap();

            let entity = uid.entity(world).unwrap();
            assert!( world.get::<Comp1>(entity).is_some());

            inverse.apply(world, cx).unwrap();

            let entity = uid.entity(world).unwrap();
            assert!( world.get::<Comp1>(entity).is_none());
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
            changeset.apply(world, cx).unwrap();
        });

        let entity = uid.entity(&mut world).unwrap();
        assert!( world.get::<Comp1>(entity).is_some());
    }

    #[test]
    fn spawn_nested_struct_bundle_in_struct_bundle() {
        let app = build_app();
        let mut world = app.world;

        let mut changeset = world.changeset();
        let uid = changeset.spawn((Comp1, Comp2, Bundle1::default())).uid();
        let changeset = changeset.build();

        ChangesetResource::<MyChangeset>::context_scope(&mut world, |world, cx| {
            changeset.apply(world, cx).unwrap();
        });

        let entity = uid.entity(&mut world).unwrap();
        assert!( world.get::<Comp1>(entity).is_some());
    }
}
