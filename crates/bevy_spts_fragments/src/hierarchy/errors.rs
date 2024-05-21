use bevy_spts_uid::UidRegistryError;
use thiserror::Error;

use super::ComponentReflectError;

#[derive(Debug, Clone, Error)]
pub enum HierarchySpawnWithParentUidError {
    #[error("Cannot spawn HierarchyFragment in world with parent because entity was not registered in the UidRegistry: \n {err} ")]
    UidRegistryError { err: UidRegistryError },
    #[error("Cannot spawn HierarchyFragment in world because there was an error extracting one of the components:\n {err}")]
    ComponentReflecError { err: ComponentReflectError },
}

impl From<UidRegistryError> for HierarchySpawnWithParentUidError {
    fn from(value: UidRegistryError) -> Self {
        Self::UidRegistryError { err: value }
    }
}
impl From<ComponentReflectError> for HierarchySpawnWithParentUidError {
    fn from(value: ComponentReflectError) -> Self {
        Self::ComponentReflecError { err: value }
    }
}
