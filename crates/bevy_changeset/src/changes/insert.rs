use std::{any::TypeId, fmt::Debug, sync::Arc};

use bevy_ecs::reflect::ReflectComponent;
use bevy_reflect::Reflect;
use bevy_spts_fragments::prelude::{ComponentFragment, Uid};

use crate::{error::ChangeError, resource::ChangesetContext};

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
    fn apply(&self, cx: &mut ChangesetContext) -> Result<ChangeIter, ChangeError> {
        let mut entity_mut = self
            .target
            .entity_world_mut(cx.world)
            .ok_or(ChangeError::NoEntity(self.target))?;

        self.component.insert(&mut entity_mut, cx.type_registry).unwrap();
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
    fn apply(&self, cx: &mut ChangesetContext) -> Result<ChangeIter, ChangeError> {
        let mut entity_mut = self
            .target
            .entity_world_mut(cx.world)
            .ok_or(ChangeError::NoEntity(self.target))?;

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
    fn apply(&self, cx: &mut ChangesetContext) -> Result<ChangeIter, ChangeError> {
        let mut entity_mut = self
            .target
            .entity_world_mut(cx.world)
            .ok_or(ChangeError::NoEntity(self.target))?;

        let registration = cx.type_registry.get(self.type_id).unwrap();
        let reflect_component = registration.data::<ReflectComponent>().unwrap();
        let component = reflect_component.reflect_mut(&mut entity_mut).unwrap();
        // self.component.remove(&mut entity_mut, cx.type_registry).unwrap();

        let cf = ComponentFragment::new(component.clone_value().into());
        Ok(InsertChange::new(self.target, cf).into_change_iter())
    }
}
