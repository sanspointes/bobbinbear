mod heirarchy;
mod insert;
mod spawn;

use std::fmt::Debug;

use bevy_ecs::{component::Component, world::World};

use crate::{error::ChangeError, uid::Uid};

use self::{
    heirarchy::SetParentChange,
    insert::{InsertChange, RemoveChange},
    spawn::{DespawnChange, SpawnChange},
};

pub trait Change: Debug {
    fn apply(&self, world: &mut World) -> Result<Box<dyn Change>, ChangeError>;
}

#[derive(Debug)]
pub struct ChangeSet {
    changes: Vec<Box<dyn Change>>,
}

impl ChangeSet {
    pub fn apply(self, world: &mut World) -> Result<ChangeSet, ChangeError> {
        println!("Applying {} changes...", self.changes.len());
        let inverse: Result<Vec<Box<dyn Change>>, ChangeError> = self
            .changes
            .into_iter()
            .map(|change| {
                println!("Applying change {change:?}");
                change.apply(world)
            })
            .collect();
        let mut inverse = inverse?;
        inverse.reverse();

        Ok(ChangeSet { changes: inverse })
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

    pub fn entity<'a>(&'a mut self, uid: Uid) -> EntityChangeset<'w, 'a> {
        EntityChangeset {
            target: uid,
            builder: self,
        }
    }

    pub fn spawn_empty<'a>(&'a mut self) -> EntityChangeset<'w, 'a> {
        let uid = Uid::default();
        self.push(Box::new(SpawnChange::new(uid)));
        EntityChangeset {
            target: uid,
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
    pub fn insert<C: Component + Clone + Debug>(&mut self, component: C) -> &mut Self {
        self.builder
            .push(Box::new(InsertChange::new(self.target, component)));
        self
    }
    pub fn remove<C: Component + Clone + Debug>(&mut self) -> &mut Self {
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

    pub fn uid(&self) -> Uid {
        self.target
    }
}
