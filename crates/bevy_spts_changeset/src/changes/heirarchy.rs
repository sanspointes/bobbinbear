use std::sync::Arc;

use anyhow::anyhow;

use bevy_ecs::world::World;
use bevy_hierarchy::{BuildWorldChildren, Parent};
use bevy_spts_fragments::prelude::{HierarchyFragment, Uid};

use crate::resource::ChangesetContext;

use super::Change;

#[derive(Debug)]
/// A Change that parents 1 entity to another
///
/// * `target`: Target entity to act upon
/// * `parent`: The desired parent for this entity.
pub struct SetParentChange {
    target: Uid,
    parent: Option<Uid>,
}
impl SetParentChange {
    pub fn parent(target: Uid, parent: Uid) -> Self {
        Self {
            target,
            parent: Some(parent),
        }
    }
    pub fn unparent(target: Uid) -> Self {
        Self {
            target,
            parent: None,
        }
    }
}

impl Change for SetParentChange {
    fn apply(
        &self,
        world: &mut World,
        _cx: &mut ChangesetContext,
    ) -> Result<Arc<dyn Change>, anyhow::Error> {
        let target = self.target.entity(world).ok_or(anyhow!(
            "Can't set parent. Can't get target. No Entity with uid {}",
            self.target
        ))?;
        let prev_parent = {
            if let Some(p) = world.get::<Parent>(target) {
                world.get::<Uid>(p.get()).cloned()
            } else {
                None
            }
        };

        match self.parent {
            Some(parent) => {
                let parent = parent.entity(world).ok_or(anyhow!(
                    "Can't set parent. Can't get parent. No Entity with uid {}",
                    parent
                ))?;

                let mut entity_mut = world.entity_mut(target);
                entity_mut.set_parent(parent);
            }
            None => {
                let mut entity_mut = world.entity_mut(target);
                entity_mut.remove_parent();
            }
        }

        Ok(Arc::new(SetParentChange {
            target: self.target,
            parent: prev_parent,
        }))
    }
}

#[derive(Debug)]
pub struct SpawnRecursiveChange {
    hierarchy: HierarchyFragment,
    parent: Option<Uid>,
}

impl SpawnRecursiveChange {
    pub fn new(hierarchy: HierarchyFragment, parent: Option<Uid>) -> Self {
        Self { hierarchy, parent }
    }
}
impl Change for SpawnRecursiveChange {
    fn apply(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<Arc<dyn Change>, anyhow::Error> {
        let hierarchy_fragment = &self.hierarchy;
        match self.parent {
            Some(parent) => hierarchy_fragment.spawn_in_world_with_parent_uid(
                world,
                cx.type_registry,
                parent,
            )?,
            None => hierarchy_fragment.spawn_in_world(world, cx.type_registry)?,
        };
        hierarchy_fragment.spawn_in_world(world, cx.type_registry)?;
        Ok(Arc::new(DespawnRecursiveChange::new(
            hierarchy_fragment.root_uid(),
        )))
    }
}

#[derive(Debug)]
pub struct DespawnRecursiveChange {
    uid: Uid,
}
impl DespawnRecursiveChange {
    pub fn new(uid: Uid) -> Self {
        Self { uid }
    }
}
impl Change for DespawnRecursiveChange {
    fn apply(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<Arc<dyn Change>, anyhow::Error> {
        let entity = self
            .uid
            .entity(world)
            .ok_or(anyhow!("No Entity with uid {}", self.uid))?;

        let parent = world
            .get::<Parent>(entity)
            .map(|p| p.get())
            .and_then(|p| world.get::<Uid>(p))
            .copied();

        let hierarchy_fragment =
            HierarchyFragment::from_world_uid(world, cx.type_registry, self.uid)?;

        world.despawn(entity);

        Ok(Arc::new(SpawnRecursiveChange::new(hierarchy_fragment, parent)))
    }
}
