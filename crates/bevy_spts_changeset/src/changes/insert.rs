use anyhow::anyhow;
use as_any::AsAny;

use std::{
    any::{Any, TypeId},
    fmt::Debug,
    sync::Arc,
};

use bevy_ecs::{event::Events, reflect::ReflectComponent, world::World};
use bevy_spts_fragments::prelude::{BundleFragment, ComponentFragment, Uid};

use crate::{
    events::{ChangedType, ChangesetEvent},
    resource::ChangesetContext,
};

use super::{Change, NotRepeatableReason};

#[derive(Debug)]
pub struct InsertChange {
    target: Uid,
    bundle: BundleFragment,
}

impl InsertChange {
    pub fn new(target: Uid, bundle: BundleFragment) -> Self {
        Self { target, bundle }
    }
}

impl Change for InsertChange {
    fn apply(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<Arc<dyn Change>, anyhow::Error> {
        let mut entity_mut = self.target.entity_world_mut(world).ok_or(anyhow!(
            "Can't insert bundle {:?}. Can't get target. No Entity with uid {}",
            self.bundle.components(),
            self.target,
        ))?;

        self.bundle.insert(&mut entity_mut, cx.type_registry)?;

        let mut events = world.resource_mut::<Events<ChangesetEvent>>();

        let type_ids: Vec<_> = self
            .bundle
            .components()
            .iter()
            .map(|c| c.try_type_id().unwrap())
            .collect();

        for ty_id in &type_ids {
            events.send(ChangesetEvent::Changed(
                self.target,
                *ty_id,
                ChangedType::Inserted,
            ));
        }

        Ok(Arc::new(RemoveChange::new(self.target, type_ids)))
    }

    fn is_repeatable(
        &self,
        other: Arc<dyn Change>,
    ) -> Result<(), crate::prelude::NotRepeatableReason> {
        let type_name = other.type_name();
        let other_any = other.as_any_arc();
        (*other_any)
            .downcast_ref::<InsertChange>()
            .ok_or_else(|| NotRepeatableReason::DifferentType(self.type_name(), type_name))?;

        Err(super::NotRepeatableReason::ChangesWorldLayout)
    }
}

#[derive(Debug)]
pub struct ApplyChange {
    target: Uid,
    bundle: BundleFragment,
}

impl ApplyChange {
    pub fn new(target: Uid, bundle: BundleFragment) -> Self {
        Self { target, bundle }
    }

    pub fn target(&self) -> &Uid {
        &self.target
    }

    pub fn bundle(&self) -> &BundleFragment {
        &self.bundle
    }

    pub fn components(&self) -> &[ComponentFragment] {
        self.bundle.components()
    }
}

impl Change for ApplyChange {
    fn apply(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<Arc<dyn Change>, anyhow::Error> {
        let mut entity_mut = self.target.entity_world_mut(world).ok_or(anyhow!(
            "ApplyChange: Can't get target. No Entity with uid {}",
            self.target,
        ))?;

        let mut components = Vec::with_capacity(self.components().len());
        let mut type_ids = Vec::with_capacity(self.components().len());
        for comp in self.components() {
            let mut comp = comp.clone();
            comp.swap(&mut entity_mut, cx.type_registry)?;
            type_ids.push(comp.try_type_id()?);
            components.push(comp);
        }

        for type_id in &type_ids {
            let mut events = world.resource_mut::<Events<ChangesetEvent>>();
            events.send(ChangesetEvent::Changed(
                self.target,
                *type_id,
                ChangedType::Applied,
            ));
        }

        let inverse = ApplyChange::new(self.target, BundleFragment::new(components));

        Ok(Arc::new(inverse))
    }

    fn is_repeatable(
        &self,
        other: Arc<dyn Change>,
    ) -> Result<(), crate::prelude::NotRepeatableReason> {
        let type_name = other.type_name();
        let other_any = other.as_any_arc();
        let other = (*other_any)
            .downcast_ref::<ApplyChange>()
            .ok_or_else(|| NotRepeatableReason::DifferentType(self.type_name(), type_name))?;

        if *self.target() != *other.target() {
            return Err(NotRepeatableReason::DifferentContent);
        }

        if other.components().len() != self.components().len() {
            return Err(NotRepeatableReason::DifferentContent);
        }

        for (a, b) in self.components().iter().zip(other.components().iter()) {
            if a.type_id() != b.type_id() {
                return Err(NotRepeatableReason::DifferentContent);
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct RemoveChange {
    target: Uid,
    type_ids: Vec<TypeId>,
}

impl RemoveChange {
    pub fn new(target: Uid, type_ids: Vec<TypeId>) -> Self {
        Self { target, type_ids }
    }
}

impl Change for RemoveChange {
    fn apply(
        &self,
        world: &mut World,
        cx: &mut ChangesetContext,
    ) -> Result<Arc<dyn Change>, anyhow::Error> {
        let mut entity_mut = self.target.entity_world_mut(world).ok_or(anyhow!(
            "RemoveChange: Can't get target. No Entity with uid {}",
            self.target,
        ))?;

        let (components, type_ids): (Vec<_>, Vec<_>) = self
            .type_ids
            .iter()
            .map(|type_id| {
                let registration = cx.type_registry.get(*type_id).unwrap();
                let reflect_component = registration.data::<ReflectComponent>().unwrap();
                let component = reflect_component.reflect_mut(&mut entity_mut).unwrap();

                let cf = ComponentFragment::new(component.clone_value().into());
                let type_id = cf.try_type_id().unwrap();
                cf.remove(&mut entity_mut, cx.type_registry).unwrap();

                (cf, type_id)
            })
            .unzip();

        let mut events = world.resource_mut::<Events<ChangesetEvent>>();
        for ty_id in type_ids.iter() {
            events.send(ChangesetEvent::Changed(
                self.target,
                *ty_id,
                ChangedType::Removed,
            ));
        }

        Ok(Arc::new(InsertChange::new(
            self.target,
            BundleFragment::new(components),
        )))
    }

    fn is_repeatable(
        &self,
        other: Arc<dyn Change>,
    ) -> Result<(), crate::prelude::NotRepeatableReason> {
        let type_name = other.type_name();
        let other_any = other.as_any_arc();
        (*other_any)
            .downcast_ref::<RemoveChange>()
            .ok_or_else(|| NotRepeatableReason::DifferentType(self.type_name(), type_name))?;

        Err(super::NotRepeatableReason::ChangesWorldLayout)
    }
}
