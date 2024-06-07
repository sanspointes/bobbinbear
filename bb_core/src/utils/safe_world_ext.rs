use std::{fmt::Debug, marker::PhantomData};

use bevy::{
    core::Name, ecs::{component::Component, entity::Entity, world::World}, utils::thiserror::Error
};

use crate::ecs::ObjectType;

// #[derive(Error, Debug)]
// pub enum BBUidGetError<C: Component + Debug> {
//     #[error("Couldn't get component {pd:?} on by uid.  Reason:\n {err:?}")]
//     UidRegistryError {
//         err: UidRegistryError,
//         pd: PhantomData<C>,
//     },
//
//     #[error("While getting component by uid {uid}: \n {err:?}")]
//     BBGetError { uid: Uid, err: BBGetError<C> },
// }
//
// impl<C: Component + Debug> From<UidRegistryError> for BBUidGetError<C> {
//     fn from(value: UidRegistryError) -> Self {
//         Self::UidRegistryError {
//             err: value,
//             pd: PhantomData::<C>,
//         }
//     }
// }

#[derive(Error, Debug)]
pub enum BBGetError<C: Component + Debug> {
    #[error("Component {pd:?} not on entity {entity:?}. ObjectType: {object_type:?} \n  Current components are {components:#?}.")]
    MissingComponent {
        entity: Entity,
        components: Vec<String>,
        name: Option<Name>,
        object_type: Option<ObjectType>,
        pd: PhantomData<C>,
    },

    #[error("Entity {entity:?} does not exist when trying to get component {pd:?}.")]
    EntityDoesNotExist { entity: Entity, pd: PhantomData<C> },
}

pub trait BBSafeWorldExt {
    fn bb_get<C: Component + Debug>(&self, entity: Entity) -> Result<&C, BBGetError<C>>;
    // fn bb_uid_get<C: Component + Debug>(&self, uid: &Uid) -> Result<&C, BBUidGetError<C>>;
}

impl BBSafeWorldExt for World {
    fn bb_get<C: Component + Debug>(&self, entity: Entity) -> Result<&C, BBGetError<C>> {
        let v = self.get(entity);
        v.ok_or_else(|| {
            if let Some(entity_ref) = self.get_entity(entity) {
                let components: Vec<String> = entity_ref
                    .archetype()
                    .components()
                    .filter_map(|id| self.components().get_info(id))
                    .map(|info| info.name().to_string())
                    .collect();
                let object_type = self.get::<ObjectType>(entity).copied();
                let name = self.get::<Name>(entity).cloned();
                BBGetError::MissingComponent {
                    entity,
                    components,
                    name,
                    object_type,
                    pd: PhantomData::<C>,
                }
            } else {
                BBGetError::EntityDoesNotExist {
                    entity,
                    pd: PhantomData::<C>,
                }
            }
        })
    }

    // fn bb_uid_get<C: Component + Debug>(&self, uid: &Uid) -> Result<&C, BBUidGetError<C>> {
    //     let entity = uid.get_entity(self)?;
    //     self.bb_get(entity)
    //         .map_err(|err| BBUidGetError::BBGetError { uid: *uid, err })
    // }
}
