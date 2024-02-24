mod heirarchy;
mod insert;
mod spawn;

use std::{any::TypeId, fmt::Debug};

use bevy_ecs::{component::Component, world::World};
use bevy_reflect::Reflect;
use bevy_spts_fragments::prelude::{ComponentFragment, EntityFragment, Uid};

use crate::resource::ChangesetContext;

pub use self::{heirarchy::*, insert::*, spawn::*};

pub struct ChangeIter(pub Box<dyn Iterator<Item = Box<dyn Change>>>);
impl Iterator for ChangeIter {
    type Item = Box<dyn Change>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub trait IntoChangeIter {
    fn into_change_iter(self) -> ChangeIter;
}

impl<T> IntoChangeIter for T
where
    T: Change + 'static,
{
    fn into_change_iter(self) -> ChangeIter {
        let boxed = Box::new(self) as Box<dyn Change>;
        ChangeIter(Box::new(Some(boxed).into_iter()))
    }
}

pub trait Change: Debug {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn apply(
        &self,
        world: &mut World,
        context: &mut ChangesetContext,
    ) -> Result<ChangeIter, anyhow::Error>;
}

#[derive(Debug)]
pub struct ChangeSet {
    changes: Vec<Box<dyn Change>>,
}

impl ChangeSet {
    pub fn apply(
        self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<ChangeSet, anyhow::Error> {
        println!("Applying {} changes...", self.changes.len());

        let mut inverse = vec![];

        for change in self.changes {
            let result = change.apply(world, cx);

            match result {
                Ok(inverse_iter) => {
                    inverse.extend(inverse_iter);
                    Ok(())
                },
                Err(reason) => Err(anyhow::anyhow!("Error while applying change {}.\n{}", change.name(), reason)),
            }?
        }

        inverse.reverse();

        Ok(ChangeSet { changes: inverse })
    }
}

#[allow(dead_code)]
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
        let entity_fragment = EntityFragment::new(uid, vec![]);
        self.push(Box::new(SpawnChange::new(entity_fragment, None)));
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
    pub fn insert<C: Component + Reflect>(&mut self, component: C) -> &mut Self {
        self.builder.push(Box::new(InsertChange::new(
            self.target,
            ComponentFragment::from_component::<C>(&component),
        )));
        self
    }
    pub fn remove<C: Component + Reflect>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<C>();
        self.builder
            .push(Box::new(RemoveChange::new(self.target, type_id)));
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
