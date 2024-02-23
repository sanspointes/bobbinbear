use std::{any::TypeId, collections::BTreeMap, marker::PhantomData, result};

use bevy_ecs::{
    component::ComponentId,
    system::Resource,
    world::{FromWorld, Mut, World},
};
use bevy_reflect::{Reflect, TypeRegistry};
use bevy_scene::SceneFilter;

#[derive(Resource, Default)]
/// Resource containing filter data for when creating / applying
///
/// * `filter`: Scene filter for when building the changesets.  Which components to include.
/// * `pd`: PhantomData for tagging changeset resource incase you want multiple.
pub struct ChangesetResource<TTag: Sync + Send + 'static> {
    filter: SceneFilter,
    pd: PhantomData<TTag>,
    components_map: BTreeMap<TypeId, ComponentId>,
}

impl<TTag: Sync + Send + 'static> ChangesetResource<TTag> {
    pub fn new(filter: SceneFilter) -> Self {
        Self {
            filter,
            pd: PhantomData::<TTag>,
            components_map: BTreeMap::new(),
        }
    }
    /// Creates a resource scope with this changeset resource.  Useful for when trying to access
    /// This ChangesetResource while still having full &mut World access.
    ///
    /// * `world`:
    /// * `scope_fn`:
    fn changeset_scope<U>(
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
    fn context_scope(world: &mut World, scope_fn: impl FnOnce(ChangesetContext)) {
        Self::changeset_scope(world, |mut world, changeset_resource| {
            let type_registry = TypeRegistry::from_world(world);
            let cx = ChangesetContext {
                type_registry: &type_registry,
                world: &mut world,
                filter: &changeset_resource.filter,
                components_map: &changeset_resource.components_map,
            };
            (scope_fn)(cx);
        });
    }

    pub fn with_filter(&mut self, filter: SceneFilter) -> &mut Self {
        self.filter = filter;
        self
    }

    /// We have to maintain a lookup map from TypeId -> ComponentId.
    /// This method pre-computes those values.
    ///
    /// * `world`: World
    pub fn pre_compute_components_map(&mut self, world: &mut World) {
        for comp_info in world.components().iter() {
            if let Some(type_id) = comp_info.type_id() {
                self.components_map.insert(type_id, comp_info.id());
            }
        }
    }
}

pub struct ChangesetContext<'a> {
    pub(crate) world: &'a mut World,
    pub(crate) filter: &'a SceneFilter,
    pub(crate) type_registry: &'a TypeRegistry,
    pub(crate) components_map: &'a BTreeMap<TypeId, ComponentId>,
}

impl ChangesetContext<'_> {
    /// Tries to get the ComponentId for a TypeId.
    /// If it doesn't exist it will look it up using self.world
    /// If it's not in world it will panic!!!!
    ///
    /// * `type_id`: TypeId of Component
    pub fn get_component_id_of_type_id(&mut self, type_id: TypeId) -> ComponentId {
        let result = self.components_map.get(&type_id);

        if let Some(result) = result {
            *result
        } else {
            let component_id = self.world.components().iter().find_map(|info| {
                let maybe_type_id = info.type_id();

                if maybe_type_id.is_some() && maybe_type_id.unwrap() == type_id {
                    Some(info.id())
                } else {
                    None
                }
            });

            component_id.unwrap_or_else(|| panic!("Could not get ComponentId of TypeId ({type_id:?}).  Is this for a component?  Has it been used in this world?"))
        }
    }
}
