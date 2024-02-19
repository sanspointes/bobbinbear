use std::marker::PhantomData;

use bevy_ecs::component::Component;

use crate::{error::ChangeError, uid::Uid};

use super::Change;

#[derive()]
pub struct InsertChange<T: Component + Clone> {
    target: Uid,
    component: T,
}

impl<T: Component + Clone> InsertChange<T> {
    pub fn new(target: Uid, component: T) -> Self {
        Self {
            target,
            component,
        }
    }
}

impl<T: Component + Clone> Change for InsertChange<T> {
    fn apply(&self, world: &mut bevy_ecs::world::World) -> Result<Box<dyn Change>, crate::error::ChangeError> {
        let target = self.target.entity(world).ok_or(ChangeError::NoEntity(self.target))?;

        world.entity_mut(target).insert(self.component.clone());

        Ok(Box::new(RemoveChange::<T>::new(self.target)))
    }
}


pub struct RemoveChange<T: Component + Clone> {
    target: Uid,
    pd: PhantomData<T>,
}

impl<T: Component + Clone> RemoveChange<T> {
    pub fn new(target: Uid) -> Self {
        Self {
            target,
            pd: PhantomData::<T>,
        }
    }
}

impl<T: Component + Clone> Change for RemoveChange<T> {
    fn apply(&self, world: &mut bevy_ecs::world::World) -> Result<Box<dyn Change>, crate::error::ChangeError> {
        let target = self.target.entity(world).ok_or(ChangeError::NoEntity(self.target))?;

        let Some(component) = world.get::<T>(target) else {
            return Err(ChangeError::component_mismatch_missing_component(self.target, format!("{:?}", self.pd)));
        };
        let inverse = Box::new(InsertChange::<T>::new(self.target, component.clone()));
        world.entity_mut(target).remove::<T>();
        Ok(inverse)
    }
}
