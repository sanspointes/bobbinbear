use anyhow::anyhow;

use std::{any::TypeId, fmt::Debug};

use bevy_ecs::{reflect::ReflectComponent, world::World};
use bevy_spts_fragments::prelude::{ComponentFragment, Uid};

use crate::resource::ChangesetContext;

use super::{Change, ChangeIter, IntoChangeIter};

#[derive(Debug)]
pub struct InsertChange {
    target: Uid,
    component: ComponentFragment,
}

impl InsertChange {
    pub fn new(target: Uid, component: ComponentFragment) -> Self {
        Self { target, component }
    }
}

impl Change for InsertChange {
    fn apply(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<ChangeIter, anyhow::Error> {
        let mut entity_mut = self.target.entity_world_mut(world).ok_or(anyhow!(
            "Can't insert component {}. Can't get target. No Entity with uid {}",
            self.component.try_type_info()?.type_path(),
            self.target,
        ))?;

        self.component.insert(&mut entity_mut, cx.type_registry)?;
        let type_id = self.component.try_type_id(cx.type_registry).unwrap();

        Ok(RemoveChange::new(self.target, type_id).into_change_iter())
    }
}

#[derive(Debug)]
pub struct ApplyChange {
    target: Uid,
    component: ComponentFragment,
}

impl ApplyChange {
    pub fn new(target: Uid, component: ComponentFragment) -> Self {
        Self { target, component }
    }
}

impl Change for ApplyChange {
    fn apply(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<ChangeIter, anyhow::Error> {
        let mut entity_mut = self.target.entity_world_mut(world).ok_or(anyhow!(
            "Can't apply component {}. Can't get target. No Entity with uid {}",
            self.component.try_type_info()?.type_path(),
            self.target,
        ))?;

        let mut component = self.component.clone();
        component.swap(&mut entity_mut, cx.type_registry).unwrap();

        Ok(ApplyChange::new(self.target, component).into_change_iter())
    }
}

#[derive(Debug)]
pub struct RemoveChange {
    target: Uid,
    type_id: TypeId,
}

impl RemoveChange {
    pub fn new(target: Uid, type_id: TypeId) -> Self {
        Self { target, type_id }
    }
}

impl Change for RemoveChange {
    fn apply(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<ChangeIter, anyhow::Error> {
        let mut entity_mut = self.target.entity_world_mut(world).ok_or(anyhow!(
            "Can't remove component {}. Can't get target. No Entity with uid {}",
            match cx.type_registry.get(self.type_id) {
                Some(registration) => registration.type_info().type_path().to_string(),
                None => "Unknown component (unregistered)".to_string(),
            },
            self.target,
        ))?;

        let registration = cx.type_registry.get(self.type_id).unwrap();
        let reflect_component = registration.data::<ReflectComponent>().unwrap();
        let component = reflect_component.reflect_mut(&mut entity_mut).unwrap();

        let cf = ComponentFragment::new(component.clone_value().into());
        Ok(InsertChange::new(self.target, cf).into_change_iter())
    }
}
