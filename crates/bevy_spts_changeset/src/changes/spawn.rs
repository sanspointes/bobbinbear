use std::fmt::Debug;

use anyhow::anyhow;
use bevy_ecs::world::World;
use bevy_hierarchy::Parent;
use bevy_spts_fragments::prelude::{EntityFragment, Uid};

use crate::resource::ChangesetContext;

use super::{Change, ChangeIter, IntoChangeIter};

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
    ) -> Result<ChangeIter, anyhow::Error> {
        let entity_fragment = &self.entity;
        match self.parent {
            Some(parent) => {
                entity_fragment.spawn_in_world_with_parent_uid(world, cx.type_registry, parent)?
            }
            None => entity_fragment.spawn_in_world(world, cx.type_registry)?,
        };
        Ok(DespawnChange::new(entity_fragment.uid()).into_change_iter())
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
    ) -> Result<ChangeIter, anyhow::Error> {
        let entity = self
            .uid
            .entity(world)
            .ok_or(anyhow!("No Entity with uid {}", self.uid))?;

        let parent = world
            .get::<Parent>(entity)
            .map(|p| p.get())
            .and_then(|p| world.get::<Uid>(p))
            .copied();

        let entity_fragment = EntityFragment::from_world_uid(world, cx.type_registry, self.uid)?;

        world.despawn(entity);

        Ok(SpawnChange::new(entity_fragment, parent).into_change_iter())
    }
}
