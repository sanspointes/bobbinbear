use bevy_hierarchy::{BuildWorldChildren, Parent};
use bevy_spts_fragments::prelude::Uid;

use crate::{error::ChangeError, resource::ChangesetContext};

use super::{Change, ChangeIter, IntoChangeIter};

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
    fn apply(&self, cx: &mut ChangesetContext) -> Result<ChangeIter, ChangeError> {
        let target = self
            .target
            .entity(cx.world)
            .ok_or(ChangeError::NoEntity(self.target))?;
        let prev_parent = {
            if let Some(p) = cx.world.get::<Parent>(target) {
                cx.world.get::<Uid>(p.get()).cloned()
            } else {
                None
            }
        };

        match self.parent {
            Some(parent) => {
                let parent = parent.entity(cx.world).ok_or(ChangeError::NoEntity(parent))?;

                let mut entity_mut = cx.world.entity_mut(target);
                entity_mut.set_parent(parent);
            }
            None => {
                let mut entity_mut = cx.world.entity_mut(target);
                entity_mut.remove_parent();
            }
        }

        Ok(SetParentChange {
            target: self.target,
            parent: prev_parent,
        }
        .into_change_iter())
    }
}
