use std::marker::PhantomData;

use bevy_ecs::{
    system::Resource,
    world::{Mut, World},
};
use bevy_reflect::TypeRegistry;
use bevy_scene::SceneFilter;

#[derive(Resource, Default)]
/// Resource containing filter data for when creating / applying
///
/// * `filter`: Scene filter for when building the changesets.  Which components to include.
/// * `pd`: PhantomData for tagging changeset resource incase you want multiple.
pub struct ChangesetResource<TTag: Sync + Send + Default + 'static> {
    type_registry: TypeRegistry,
    pd: PhantomData<TTag>,
}

impl<TTag: Sync + Send + Default + 'static> ChangesetResource<TTag> {
    pub fn new(type_registry: TypeRegistry) -> Self {
        Self {
            type_registry,
            pd: PhantomData::<TTag>,
        }
    }
    /// Creates a resource scope with this changeset resource.  Useful for when trying to access
    /// This ChangesetResource while still having full &mut World access.
    ///
    /// * `world`:
    /// * `scope_fn`:
    pub fn changeset_scope<U>(
        world: &mut World,
        scope_fn: impl FnOnce(&mut World, Mut<ChangesetResource<TTag>>) -> U,
    ) -> U {
        world.resource_scope::<ChangesetResource<TTag>, U>(scope_fn)
    }

    /// Creates a resource scope with this changeset resource.  Useful for when trying to access
    /// This ChangesetResource while still having full &mut World access.
    ///
    /// * `world`:
    /// * `scope_fn`:
    pub fn context_scope(world: &mut World, scope_fn: impl FnOnce(&mut World, &mut ChangesetContext)) {
        Self::changeset_scope(world, |world, changeset_resource| {
            let mut cx = ChangesetContext {
                type_registry: &changeset_resource.type_registry,
            };
            (scope_fn)(world, &mut cx);
        });
    }
}

pub struct ChangesetContext<'a> {
    pub(crate) type_registry: &'a TypeRegistry,
}
