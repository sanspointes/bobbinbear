
use bevy_ecs::entity::Entity;
use thiserror::Error;

use crate::component::ComponentFromTypeIdError;

#[derive(Debug, Clone, Error)]
pub enum BundleFromEntityError {
    #[error("Cannot create BundleFragment from entity because entity ({entity:?}) does not exist in world.")]
    EntityNotExist { entity: Entity },
    #[error("Cannot create BundleFragment from world because there was an error creating ComponentFragment:\n {err}")]
    ComponentFromTypeId { err: ComponentFromTypeIdError },
}
impl From<ComponentFromTypeIdError> for BundleFromEntityError {
    fn from(value: ComponentFromTypeIdError) -> Self {
        Self::ComponentFromTypeId { err: value }
    }
}
