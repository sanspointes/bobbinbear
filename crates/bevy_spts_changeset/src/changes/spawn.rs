use std::{fmt::Debug, sync::Arc};

use anyhow::anyhow;
use as_any::AsAny;
use bevy_ecs::{event::Events, world::World};
use bevy_hierarchy::{BuildWorldChildren, Parent};
use bevy_spts_fragments::prelude::{EntityFragment, Uid};
use bevy_spts_uid::UidRegistry;

use crate::{events::ChangesetEvent, resource::ChangesetContext};

use super::Change;

#[derive(Debug)]
pub struct SpawnChange {
    entity: EntityFragment,
    parent: Option<Uid>,
}

impl SpawnChange {
    pub fn new(entity: EntityFragment, parent: Option<Uid>) -> Self {
        Self { entity, parent }
    }
}
impl Change for SpawnChange {
    fn apply(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<Arc<dyn Change>, anyhow::Error> {
        let parent_e = self
            .parent
            .map(|uid| world.resource::<UidRegistry>().get_entity(uid));

        let entity_fragment = &self.entity;
        let mut entity_mut = entity_fragment.spawn_in_world(world, cx.type_registry)?;

        if let Some(parent) = parent_e {
            entity_mut.set_parent(parent?);
        }

        let mut events = world.resource_mut::<Events<ChangesetEvent>>();
        events.send(ChangesetEvent::Spawned(self.entity.uid()));

        Ok(Arc::new(DespawnChange::new(entity_fragment.uid())))
    }

    fn is_repeatable(
        &self,
        other: Arc<dyn Change>,
    ) -> Result<(), crate::prelude::NotRepeatableReason> {
        let type_name = other.type_name();
        let other_any = other.as_any_arc();
        (*other_any)
            .downcast_ref::<DespawnChange>()
            .ok_or_else(|| {
                super::NotRepeatableReason::DifferentType(self.type_name(), type_name)
            })?;

        Err(super::NotRepeatableReason::ChangesWorldLayout)
    }
}

#[derive(Debug)]
pub struct DespawnChange {
    uid: Uid,
}
impl DespawnChange {
    pub fn new(uid: Uid) -> Self {
        Self { uid }
    }
}
impl Change for DespawnChange {
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

        let entity_fragment =
            EntityFragment::despawn_from_world_uid(world, cx.type_registry, cx.filter, self.uid)?;

        let mut events = world.resource_mut::<Events<ChangesetEvent>>();
        events.send(ChangesetEvent::Despawned(self.uid));

        Ok(Arc::new(SpawnChange::new(entity_fragment, parent)))
    }

    fn is_repeatable(
        &self,
        other: Arc<dyn Change>,
    ) -> Result<(), crate::prelude::NotRepeatableReason> {
        let type_name = other.type_name();
        let other_any = other.as_any_arc();
        (*other_any)
            .downcast_ref::<DespawnChange>()
            .ok_or_else(|| {
                super::NotRepeatableReason::DifferentType(self.type_name(), type_name)
            })?;

        Err(super::NotRepeatableReason::ChangesWorldLayout)
    }
}
