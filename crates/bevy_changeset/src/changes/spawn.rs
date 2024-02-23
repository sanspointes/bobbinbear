use std::fmt::Debug;

use bevy_spts_fragments::prelude::{EntityFragment, Uid};

use crate::{error::ChangeError, resource::ChangesetContext};

use super::{Change, ChangeIter, IntoChangeIter};

#[derive(Debug)]
pub struct SpawnChange(EntityFragment);

impl SpawnChange {
    pub fn new(entity_fragment: EntityFragment) -> Self {
        Self(entity_fragment)
    }
}
impl Change for SpawnChange {
    fn apply(&self, cx: &mut ChangesetContext) -> Result<ChangeIter, ChangeError> {
        let entity_fragment = &self.0;
        entity_fragment.spawn_in_world(cx.world, cx.type_registry);
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
    fn apply(&self, cx: &mut ChangesetContext) -> Result<ChangeIter, ChangeError> {
        let entity = self
            .uid
            .entity(cx.world)
            .ok_or(ChangeError::NoEntity(self.uid))?;

        let entity_fragment =
            EntityFragment::from_world_uid(cx.world, cx.type_registry, cx.filter, self.uid);

        cx.world.despawn(entity);
        Ok(SpawnChange::new(entity_fragment).into_change_iter())
    }
}
