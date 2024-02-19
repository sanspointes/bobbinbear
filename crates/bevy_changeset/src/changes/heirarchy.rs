use bevy_ecs::world::World;
use bevy_hierarchy::{BuildWorldChildren, Parent};

use crate::{error::ChangeError, uid::Uid};

use super::Change;

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
    fn apply(&self, world: &mut World) -> Result<Box<dyn Change>, ChangeError> {
        let target = self
            .target
            .entity(world)
            .ok_or(ChangeError::NoEntity(self.target))?;
        let prev_parent = {
            if let Some(p) = world.get::<Parent>(target) {
                world.get::<Uid>(p.get()).cloned()
            } else {
                None
            }
        };

        match self.parent {
            Some(parent) => {
                let parent = parent.entity(world).ok_or(ChangeError::NoEntity(parent))?;

                let mut entity_mut = world.entity_mut(target);
                entity_mut.set_parent(parent);
            }
            None => {
                let mut entity_mut = world.entity_mut(target);
                entity_mut.remove_parent();
            }
        }

        Ok(Box::new(SetParentChange {
            target: self.target,
            parent: prev_parent,
        }))
    }
}
