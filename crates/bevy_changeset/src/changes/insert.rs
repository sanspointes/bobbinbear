use std::{fmt::Debug, marker::PhantomData};

use bevy_ecs::component::Component;

use crate::{error::ChangeError, uid::Uid};

use super::{Change, ChangeIter, IntoChangeIter};

#[derive(Debug)]
pub struct InsertChange<T: Component + Clone + Debug> {
    target: Uid,
    component: T,
}

impl<T: Component + Clone + Debug> InsertChange<T> {
    pub fn new(target: Uid, component: T) -> Self {
        Self { target, component }
    }
}

impl<T: Component + Clone + Debug> Change for InsertChange<T> {
    fn apply(
        &self,
        world: &mut bevy_ecs::world::World,
    ) -> Result<ChangeIter, ChangeError> {
        let target = self
            .target
            .entity(world)
            .ok_or(ChangeError::NoEntity(self.target))?;

        world.entity_mut(target).insert(self.component.clone());

        Ok(RemoveChange::<T>::new(self.target).into_change_iter())
    }
}

#[derive(Debug)]
pub struct RemoveChange<T: Component + Clone + Debug> {
    target: Uid,
    pd: PhantomData<T>,
}

impl<T: Component + Clone + Debug> RemoveChange<T> {
    pub fn new(target: Uid) -> Self {
        Self {
            target,
            pd: PhantomData::<T>,
        }
    }
}

impl<T: Component + Clone + Debug> Change for RemoveChange<T> {
    fn apply(
        &self,
        world: &mut bevy_ecs::world::World,
    ) -> Result<ChangeIter, ChangeError> {
        let target = self
            .target
            .entity(world)
            .ok_or(ChangeError::NoEntity(self.target))?;

        let Some(component) = world.get::<T>(target) else {
            return Err(ChangeError::component_mismatch_missing_component(
                self.target,
                format!("{:?}", self.pd),
            ));
        };
        let inverse = Some(Box::new(InsertChange::<T>::new(
            self.target,
            component.clone(),
        )));
        world.entity_mut(target).remove::<T>();

        Ok(RemoveChange::<T>::new(self.target).into_change_iter())
    }
}
