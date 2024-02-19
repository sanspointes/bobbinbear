mod heirarchy;
mod insert;
mod spawn;

use bevy_ecs::{component::Component, world::World};

use crate::{error::ChangeError, uid::Uid};

use self::{
    heirarchy::SetParentChange,
    insert::{InsertChange, RemoveChange},
    spawn::DespawnChange,
};

pub trait Change {
    fn apply(&self, world: &mut World) -> Result<Box<dyn Change>, ChangeError>;
}

pub struct ChangeSet {
    changes: Vec<Box<dyn Change>>,
}

impl ChangeSet {
    fn apply(self, world: &mut World) -> Result<ChangeSet, ChangeError> {
        let inverse: Result<Vec<Box<dyn Change>>, ChangeError> = self
            .changes
            .into_iter()
            .map(|change| change.apply(world))
            .rev() // Reverse order 
            .collect();

        Ok(ChangeSet { changes: inverse? })
    }
}

pub struct ChangesetBuilder<'w> {
    world: &'w mut World,
    changes: Vec<Box<dyn Change>>,
}

impl<'w> ChangesetBuilder<'w> {
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world,
            changes: Vec::default(),
        }
    }

    fn push(&mut self, change: Box<dyn Change>) {
        self.changes.push(change);
    }

    pub fn spawn_empty<'a>(&'a mut self) -> EntityChangeset<'w, 'a> {
        EntityChangeset {
            target: Uid::default(),
            builder: self,
        }
    }
    pub fn despawn(&mut self, uid: Uid) -> &mut Self {
        self.push(Box::new(DespawnChange::new(uid)));
        self
    }

    pub fn build(self) -> ChangeSet {
        ChangeSet {
            changes: self.changes,
        }
    }
}

pub struct EntityChangeset<'w, 'a> {
    pub(crate) target: Uid,
    pub(crate) builder: &'a mut ChangesetBuilder<'w>,
}

impl<'w, 'a> EntityChangeset<'w, 'a> {
    pub fn insert<C: Component + Clone>(&mut self, component: C) -> &mut Self {
        self.builder
            .push(Box::new(InsertChange::new(self.target, component)));
        self
    }
    pub fn remove<C: Component + Clone>(&mut self) -> &mut Self {
        self.builder
            .push(Box::new(RemoveChange::<C>::new(self.target)));
        self
    }
    pub fn set_parent(&mut self, parent: Uid) -> &mut Self {
        self.builder
            .push(Box::new(SetParentChange::parent(self.target, parent)));
        self
    }
    pub fn remove_parent(&mut self) -> &mut Self {
        self.builder
            .push(Box::new(SetParentChange::unparent(self.target)));
        self
    }
}
