use bevy_ecs::entity::Entity;
use bevy_spts_uid::UidRegistryError;
use thiserror::Error;

use crate::{bundle::BundleFromEntityError, component::ComponentFromTypeIdError};

#[derive(Debug, Clone, Error)]
pub enum EntityFromWorldError {
    #[error("Cannot create EntityFragment from world because entity was not registered in the UidRegistry:\n {err}")]
    UidRegistry { err: UidRegistryError },
    #[error("Cannot create EntityFragment from entity because entity ({entity:?}) does not exist in world.")]
    EntityNotExist { entity: Entity },
    #[error("Cannot create EntityFragment from world because there was an error extracting one of the components:\n {err}")]
    ComponentFromTypeId { err: ComponentFromTypeIdError },
}

impl From<UidRegistryError> for EntityFromWorldError {
    fn from(value: UidRegistryError) -> Self {
        Self::UidRegistry { err: value }
    }
}

impl From<ComponentFromTypeIdError> for EntityFromWorldError {
    fn from(value: ComponentFromTypeIdError) -> Self {
        Self::ComponentFromTypeId { err: value }
    }
}

impl From<BundleFromEntityError> for EntityFromWorldError {
    fn from(value: BundleFromEntityError) -> Self {
        match value {
            BundleFromEntityError::EntityNotExist { entity } => Self::EntityNotExist { entity },
            BundleFromEntityError::ComponentFromTypeId { err } => Self::ComponentFromTypeId { err },
        }
    }
}
