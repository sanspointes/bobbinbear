mod heirarchy;
mod insert;
mod spawn;

use std::{any::TypeId, fmt::Debug, sync::Arc};

use bevy_ecs::{component::Component, world::World};
use bevy_reflect::Reflect;
use bevy_spts_fragments::prelude::{ComponentFragment, EntityFragment, Uid};

use crate::resource::ChangesetContext;

pub use self::{heirarchy::*, insert::*, spawn::*};

pub trait Change: Debug + Send + Sync {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn apply(
        &self,
        world: &mut World,
        context: &mut ChangesetContext,
    ) -> Result<Arc<dyn Change>, anyhow::Error>;
}

#[derive(Debug)]
pub struct ChangeSet {
    changes: Vec<Arc<dyn Change>>,
}

impl ChangeSet {
    pub fn apply(
        self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<ChangeSet, anyhow::Error> {
        println!("Applying {} changes...", self.changes.len());

        let mut inverse_changes = vec![];

        for change in self.changes {
            let result = change.apply(world, cx);

            match result {
                Ok(inverse) => {
                    inverse_changes.push(inverse);
                    Ok(())
                },
                Err(reason) => Err(anyhow::anyhow!("Error while applying change {}.\n{}", change.name(), reason)),
            }?
        }

        inverse_changes.reverse();

        Ok(ChangeSet { changes: inverse_changes })
    }
}

#[allow(dead_code)]
pub struct ChangesetBuilder<'w> {
    world: &'w mut World,
    changes: Vec<Arc<dyn Change>>,
}

impl<'w> ChangesetBuilder<'w> {
    pub fn new(world: &'w mut World) -> Self {
        Self {
            world,
            changes: Vec::default(),
        }
    }

    fn push(&mut self, change: Arc<dyn Change>) {
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
        self.push(Arc::new(SpawnChange::new(entity_fragment, None)));
        EntityChangeset {
            target: uid,
            builder: self,
        }
    }
    pub fn despawn(&mut self, uid: Uid) -> &mut Self {
        self.push(Arc::new(DespawnChange::new(uid)));
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
        self.builder.push(Arc::new(InsertChange::new(
            self.target,
            ComponentFragment::from_component::<C>(&component),
        )));
        self
    }
    pub fn remove<C: Component + Reflect>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<C>();
        self.builder
            .push(Arc::new(RemoveChange::new(self.target, type_id)));
        self
    }
    pub fn set_parent(&mut self, parent: Uid) -> &mut Self {
        self.builder
            .push(Arc::new(SetParentChange::parent(self.target, parent)));
        self
    }
    pub fn remove_parent(&mut self) -> &mut Self {
        self.builder
            .push(Arc::new(SetParentChange::unparent(self.target)));
        self
    }

    pub fn uid(&self) -> Uid {
        self.target
    }
}
