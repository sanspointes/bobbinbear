use std::{any::TypeId, fmt::Display, sync::Arc};

use anyhow::anyhow;
use bevy_ecs::{bundle::Bundle, component::Component, reflect::AppTypeRegistry, world::World};
use bevy_reflect::{FromReflect, Reflect};
use bevy_spts_fragments::prelude::{
    BundleFragment, BundleToFragment, EntityFragment, Uid,
};

use crate::{
    changes::{
        ApplyChange, Change, DespawnChange, DespawnRecursiveChange, InsertChange, RemoveChange,
        SetParentChange, SpawnChange,
    },
    prelude::NotRepeatableReason,
    resource::ChangesetContext,
};

#[derive(Debug)]
/// Stores a list of changes to be applied to the world.
pub struct Changeset {
    changes: Vec<Arc<dyn Change>>,
}

impl Display for Changeset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ChangeSet [\n")?;
        for change in &self.changes {
            writeln!(f, "\t{change:?}\n")?;
        }
        writeln!(f, "]\n")
    }
}

impl Changeset {
    pub fn apply(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<Changeset, anyhow::Error> {
        let mut inverse_changes = vec![];

        for change in self.changes.iter() {
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

        Ok(Changeset {
            changes: inverse_changes,
        })
    }

    /// Similar to apply but compares the changeset with a prior one.  If it has the same signature
    /// as prior changeset it will apply only the 'repeatable' commands.
    pub fn try_apply_repeatable(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
        other: &Changeset,
    ) -> Result<Changeset, anyhow::Error> {
        if self.changes.len() != other.changes.len() {
            return Err(anyhow!("Changesets have different lengths."));
        }

        let next_changes: Result<Vec<_>, _> = self.changes.iter().zip(other.changes.iter().rev()).map(|(a, b)| {
            match a.is_repeatable(b.clone()) {
                Ok(()) => {
                    Ok((b.clone(), Some(a)))
                }
                Err(NotRepeatableReason::ChangesWorldLayout) => {
                    Ok((b.clone(), None))
                }
                Err(reason) => Err(reason),
            }
        }).collect();
        let next_changes = next_changes?;

        let mut inverse_changes = Vec::with_capacity(next_changes.len());
        for (change, to_apply) in next_changes {
            if let Some(to_apply) = to_apply {
                to_apply.apply(world, cx)?;
            }
            inverse_changes.push(change);
        }

        inverse_changes.reverse();

        Ok(Changeset { changes: inverse_changes })
    }
}

/// A builder for [`Changeset`] that mirrors the Bevy native [`Commands`] api.
///
/// When you've finished building your changeset call `build()` to recieve the [Changeset].
pub struct ChangesetCommands<'w> {
    world: &'w World,
    changes: Vec<Arc<dyn Change>>,
}

impl<'w> ChangesetCommands<'w> {
    /// Creates a new ChangesetCommands from the world.
    ///
    /// * `world`:
    pub fn new(world: &'w World) -> Self {
        Self {
            world,
            changes: Vec::default(),
        }
    }

    /// Adds an arbitrary [Change] to the world.  This allows you to define your own custom
    /// change in the same way Bevy lets you define Custom commands.
    ///
    /// See: https://taintedcoders.com/bevy/patterns/custom-commands/
    pub fn add(&mut self, change: Arc<dyn Change>) {
        self.changes.push(change);
    }

    /// Returns the [EntityChangeset] of a an existing entity allowing you to edit an
    /// existing entity.
    ///
    /// # Example
    /// ```
    /// fn example(world: &mut World) {
    ///     let changeset_commands = world.changeset();
    ///
    ///     // Can spawn the entity in the same changeset or use a pre-existing one.
    ///     let uid = world.spawn_empty().uid();
    ///     // Inserts Transform component to pre-existing entity.
    ///     world.entity(uid).insert(Transform::default());
    ///
    ///     // Builds the [Changeset] object which you can then apply to the world.
    ///     let changeset = changeset_commands.build();
    /// }
    /// ```
    /// * `uid`:
    pub fn entity<'a>(&'a mut self, uid: Uid) -> EntityChangeset<'w, 'a> {
        EntityChangeset {
            target: uid,
            builder: self,
        }
    }

    /// Spawns a new entity with a [Uid] component as well as the components within the bundle
    /// argument.
    ///
    /// # Example
    /// ```
    /// #[derive(Component, Reflect, Default)]
    /// #[reflect(Component)]
    /// struct MyComponent(usize);
    ///
    /// #[derive(Component, Reflect, Default)]
    /// #[reflect(Component)]
    /// struct MyComponent2(usize);
    ///
    /// #[derive(Bundle, Reflect, Default)]
    /// #[reflect(Bundle)]
    /// struct MyBundle {
    ///   my_component: MyComponent,
    ///   my_component_2: MyComponent2,
    /// }
    ///
    /// fn changeset_system(world: &mut World) {
    ///     let mut changeset_commands = world.changeset();
    ///     // Create a new entity with a single component.
    ///     changeset_commands.spawn(MyComponent(0));
    ///     // Create a new entity with multiple components.
    ///     changeset_commands.spawn((MyComponent(1), MyComponent2(1)));
    ///     // Create a new entity with a bundle.
    ///     changeset_commands.spawn(MyBundle::default());
    ///
    ///     // Spawn returns the EntityChangeset builder to insert more bundles.
    ///     changeset_commands.spawn(MyComponent(0)).insert(MyComponent2(0));
    ///     
    ///     // Builds the [Changeset] object which you can then apply to the world
    ///     let changeset = changeset_commands.build();
    /// }
    /// ```
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
        self.add(Arc::new(change));
        EntityChangeset {
            target: uid,
            builder: self,
        }
    }

    /// Spawns a new component with only a [Uid] component.  Uids are currently required for
    /// persistance between undo/redos.
    pub fn spawn_empty<'a>(&'a mut self) -> EntityChangeset<'w, 'a> {
        let uid = Uid::default();
        let entity_fragment = EntityFragment::new(uid, BundleFragment::new(vec![]));
        self.add(Arc::new(SpawnChange::new(entity_fragment, None)));
        EntityChangeset {
            target: uid,
            builder: self,
        }
    }

    /// Despawns an entity with the provided [Uid] from the world.
    ///
    /// When generating the inverse changeset it will only copy Components that are [Reflect],
    /// [ReflectComponent], and registered within the type registry.  Additionally, the components
    /// will be further filtered according to the [SceneFilter] associated with your changeset
    /// resource (allows all by default).
    ///
    /// See: [ChangesetResource]
    pub fn despawn(&mut self, uid: Uid) -> &mut Self {
        self.add(Arc::new(DespawnChange::new(uid)));
        self
    }

    /// Builds this [ChangesetCommands] into a [Changeset] which can be later used to perform and
    /// undo changes to the Bevy world.
    pub fn build(self) -> Changeset {
        Changeset {
            changes: self.changes,
        }
    }
}

/// Changeset builder for mutating an entity.
pub struct EntityChangeset<'w, 'a> {
    pub(crate) target: Uid,
    pub(crate) builder: &'a mut ChangesetCommands<'w>,
}

impl<'w, 'a> EntityChangeset<'w, 'a> {
    /// Inserts a component or bundle into the entity.  Uses an [InsertChange]
    /// under the hood and stores the bundle within a [BundleFragment].
    /// # Example
    /// ```
    /// #[derive(Bundle, Default, Reflect)]
    /// #[reflect(Bundle)]
    /// struct MyBundle {
    ///     visiblity: Visiblity,
    ///     inherited_visibility: InheritedVisibility,
    ///     view_visibility: ViewVisibility,
    /// }
    ///
    /// changeset_commands.entity(uid).insert(MyBundle::default());
    ///
    /// #[derive(Component, Default, Reflect)]
    /// #[reflect(Component)]
    /// struct MyComponent(usize);
    ///
    /// changeset_commands.entity(uid).insert(MyComponent(0));
    /// ```
    pub fn insert<B: Bundle + Reflect + FromReflect>(&mut self, bundle: B) -> &mut Self {
        let bundle = {
            let type_registry = self.builder.world.resource::<AppTypeRegistry>().read();
            bundle.to_fragment(&type_registry)
        };

        self.builder
            .add(Arc::new(InsertChange::new(self.target, bundle)));
        self
    }

    /// Updates components within the entity using the bundled components.  Uses an [ApplyChange]
    /// under the hood and stores the bundle within a [BundleFragment].
    ///
    /// Warn: if the entity does not have this component it will throw an error when applied.
    /// # Example
    /// ```
    /// #[derive(Bundle, Default, Reflect)]
    /// #[reflect(Bundle)]
    /// struct MyBundle {
    ///     visiblity: Visiblity,
    ///     inherited_visibility: InheritedVisibility,
    ///     view_visibility: ViewVisibility,
    /// }
    ///
    /// changeset_commands.entity(uid).apply(MyBundle::default());
    ///
    /// #[derive(Component, Default, Reflect)]
    /// #[reflect(Component)]
    /// struct MyComponent(usize);
    ///
    /// changeset_commands.entity(uid).apply(MyComponent(0));
    /// ```
    pub fn apply<B: Bundle + Reflect + FromReflect>(&mut self, bundle: B) -> &mut Self {
        let bundle = {
            let type_registry = self.builder.world.resource::<AppTypeRegistry>().read();
            bundle.to_fragment(&type_registry)
        };
        self.builder
            .add(Arc::new(ApplyChange::new(self.target, bundle)));
        self
    }
    /// Removes a single component from the entity.  Uses [RemoveChange] under the hood and stores
    /// the component within a [ComponentFragment].
    ///
    /// # Example
    /// ```
    /// #[derive(Component, Default, Reflect)]
    /// #[reflect(Component)]
    /// struct MyComponent(usize);
    ///
    /// changeset_commands.entity(uid).remove::<MyComponent>();
    /// ```
    pub fn remove<C: Component + Reflect>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<C>();
        self.builder
            .add(Arc::new(RemoveChange::new(self.target, vec![type_id])));
        self
    }
    /// Changes the parent of an entity.  Uses [SetParentChange] under the hood which uses
    /// `set_parent`/`remove_parent` of [EntityWorldMut].
    ///
    /// # Example
    /// ```
    /// let parent_uid = changeset_commands.spawn_empty().uid();
    /// changeset_commands.spawn_empty().set_parent(parent_uid);
    /// ```
    pub fn set_parent(&mut self, parent: Uid) -> &mut Self {
        self.builder
            .add(Arc::new(SetParentChange::parent(self.target, parent)));
        self
    }
    /// Removes the parent from an entity (moving it to top of scene hierarchy).  Uses
    /// [SetParentChange] under the hood which uses `set_parent`/`remove_parent`
    /// of [EntityWorldMut].
    ///
    /// # Example
    /// ```
    /// let parent_uid = changeset_commands.spawn_empty().uid();
    /// let child_uid = changeset_commands.spawn_empty().set_parent(parent_uid);
    ///
    /// // Unparents child from parent
    /// changeset_commands.entity(child_uid).remove_parent();
    /// ```
    pub fn remove_parent(&mut self) -> &mut Self {
        self.builder
            .add(Arc::new(SetParentChange::unparent(self.target)));
        self
    }
    /// Despawns an entity from the world.  Uses [DespawnChange] under the hood which just calls
    /// `world.despawn()` and stores the entity in an [EntityFragment].
    ///
    /// # Example
    /// ```
    /// let parent_uid = changeset_commands.spawn_empty().uid();
    /// let child_uid = changeset_commands.spawn_empty().set_parent(parent_uid);
    ///
    /// // Unparents child from parent before despawning
    /// changeset_commands.entity(child_uid).remove_parent().despawn();
    ///
    /// // If entity has no children/parents it's safe to just despawn it.
    /// changeset_commands.entity(parent_uid).despawn();
    /// ```
    ///
    /// # Gotchas
    /// Will not manage changes to the hierarchy for you.  You will need to also call
    /// [`remove_parent`] if the entity has a parent.
    ///
    /// Only components that meet the following criteria will be respawned when applying
    /// the inverse change ([SpawnChange]).
    /// 1. Component is `#[derive(Reflect)]` and `#[reflect(Component)]`
    /// 2. Component is registed in the [AppTypeRegistry]
    /// 3. Component is allowed according to the [SceneFilter] of the [ChangesetResource].
    ///
    pub fn despawn(&mut self) -> &mut Self {
        self.builder.add(Arc::new(DespawnChange::new(self.target)));
        self
    }
    /// Despawns an entity from the world.  Uses [DespawnChange] under the hood which just calls
    /// `world.despawn()` and stores all the entities and their heirarchies in a [HeirarchyFragment].  
    ///
    /// # Example
    /// ```
    /// let parent_uid = changeset_commands.spawn_empty().uid();
    /// let child_uid = changeset_commands.spawn_empty().set_parent(parent_uid);
    ///
    /// // Unparents child from parent before despawning
    /// changeset_commands.entity(child_uid).remove_parent().despawn();
    ///
    /// // If entity has no children/parents it's safe to just despawn it.
    /// changeset_commands.entity(parent_uid).despawn();
    /// ```
    ///
    /// # Gotchas
    /// Will not manage changes to the hierarchy for you.  You will need to also call
    /// [`remove_parent`] if the entity has a parent.
    ///
    /// Only components that meet the following criteria will be respawned when applying
    /// the inverse change ([SpawnChange]).
    /// 1. Component is `#[derive(Reflect)]` and `#[reflect(Component)]`
    /// 2. Component is registed in the [AppTypeRegistry]
    /// 3. Component is allowed according to the [SceneFilter] of the [ChangesetResource].
    ///
    pub fn despawn_recursive(&mut self) -> &mut Self {
        self.builder
            .add(Arc::new(DespawnRecursiveChange::new(self.target)));
        self
    }

    pub fn uid(&self) -> Uid {
        self.target
    }
}
