use std::marker::PhantomData;

use bevy_ecs::{
    reflect::AppTypeRegistry,
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
    pd: PhantomData<TTag>,
    pub filter: SceneFilter,
}

impl<TTag: Sync + Send + Default + 'static> ChangesetResource<TTag> {
    pub fn new() -> Self {
        Self {
            pd: PhantomData::<TTag>,
            filter: SceneFilter::Unset,
        }
    }
    pub fn with_filter(mut self, filter: SceneFilter) -> Self {
        self.filter = filter;
        self
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
    pub fn context_scope<U>(
        world: &mut World,
        scope_fn: impl FnOnce(&mut World, &mut ChangesetContext) -> U,
    ) -> U {
        world.resource_scope::<AppTypeRegistry, U>(|world, type_registry| {
            Self::changeset_scope(world, |world, res| {
                let mut cx = ChangesetContext {
                    type_registry: &(type_registry.read()),
                    filter: &res.filter,
                };
                (scope_fn)(world, &mut cx)
            })
        })
    }
}

pub struct ChangesetContext<'a> {
    pub(crate) type_registry: &'a TypeRegistry,
    pub(crate) filter: &'a SceneFilter,
}
