use std::{any::Any, sync::Arc};

use anyhow::anyhow;

use as_any::AsAny;
use bevy_ecs::{event::Events, world::World};
use bevy_hierarchy::{BuildWorldChildren, Children, Parent};
use bevy_reflect::Typed;
use bevy_spts_fragments::prelude::{HierarchyFragment, Uid};

use crate::{
    events::{ChangedType, ChangesetEvent},
    resource::ChangesetContext,
};

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

        let mut events = world.resource_mut::<Events<ChangesetEvent>>();

        let changed_type = if self.parent.is_some() {
            ChangedType::Mutated
        } else {
            ChangedType::Inserted
        };
        events.send(ChangesetEvent::Changed(
            self.target,
            Parent::type_info().type_id(),
            changed_type,
        ));
        if let Some(parent) = self.parent {
            events.send(ChangesetEvent::Changed(
                parent,
                Children::type_info().type_id(),
                ChangedType::Removed,
            ));
        }

        Ok(Arc::new(SetParentChange {
            target: self.target,
            parent: prev_parent,
        }))
    }

    fn is_repeatable(&self, other: Arc<dyn Change>) -> Result<(), crate::prelude::NotRepeatableReason> {
        if self.type_id() != other.type_id() {
            return Err(super::NotRepeatableReason::DifferentType(self.type_name(), other.type_name()));
        }
        Err(super::NotRepeatableReason::ChangesWorldLayout)
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

        let mut events = world.resource_mut::<Events<ChangesetEvent>>();
        for uid in hierarchy_fragment.all_uids() {
            events.send(ChangesetEvent::Spawned(*uid));
        }

        Ok(Arc::new(DespawnRecursiveChange::new(
            hierarchy_fragment.root_uid(),
        )))
    }

    fn is_repeatable(
        &self,
        other: Arc<dyn Change>,
    ) -> Result<(), crate::prelude::NotRepeatableReason> {
        let type_name = other.type_name();
        let other_any = other.as_any_arc();
        (*other_any)
            .downcast_ref::<SpawnRecursiveChange>()
            .ok_or_else(|| super::NotRepeatableReason::DifferentType(self.type_name(), type_name))?;

        Err(super::NotRepeatableReason::ChangesWorldLayout)
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

        let hierarchy_fragment = HierarchyFragment::despawn_from_world_uid(
            world,
            cx.type_registry,
            cx.filter,
            self.uid,
        )?;

        let mut events = world.resource_mut::<Events<ChangesetEvent>>();
        for uid in hierarchy_fragment.all_uids() {
            events.send(ChangesetEvent::Despawned(*uid));
        }

        Ok(Arc::new(SpawnRecursiveChange::new(
            hierarchy_fragment,
            parent,
        )))
    }

    fn is_repeatable(
        &self,
        other: Arc<dyn Change>,
    ) -> Result<(), crate::prelude::NotRepeatableReason> {
        let type_name = other.type_name();
        let other_any = other.as_any_arc();
        (*other_any)
            .downcast_ref::<DespawnRecursiveChange>()
            .ok_or_else(|| super::NotRepeatableReason::DifferentType(self.type_name(), type_name))?;

        Err(super::NotRepeatableReason::ChangesWorldLayout)
    }
}
