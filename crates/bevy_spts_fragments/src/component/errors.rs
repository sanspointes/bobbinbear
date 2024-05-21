use std::any::TypeId;

use bevy_ecs::entity::Entity;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ComponentReflectError {
    #[error("Component {type_path} does not have represented type info.")]
    NoRepresentedType { type_path: String },
    #[error("Component {type_path} is unregistered in the type registry (but is reflected).")]
    UnregisteredButReflectedType { type_path: String },
    #[error("Component {type_path} does not derive ReflectComponent.")]
    UnregisteredComponent { type_path: String },
}

#[derive(Debug, Clone, Error)]
pub enum ComponentFromTypeIdError {
    #[error("Cannot create ComponentFragment from component's TypeId ({type_id:?}) because the type is not in the type_registry.")]
    NotRegistered { type_id: TypeId },
    #[error("Cannot create ComponentFragment from component's TypeId ({type_id:?}) because the type does not derive ReflectComponent.")]
    NotReflectComponent { type_id: TypeId },
    #[error("Cannot create ComponentFragment from component's TypeId ({type_id:?}) because the Entity ({entity:?}) does not contain that component.")]
    NotContainedByEntity { entity: Entity, type_id: TypeId },
}

#[derive(Debug, Clone, Error)]
pub enum ComponentApplyError {
    #[error("Error reflecting component: {0}")]
    ReflectionError(ComponentReflectError),
    #[error("The component that fragment represents is missing on the entity.  Type_path: {type_path}")]
    ComponentMissingOnEntity { type_path: String },
}

impl From<ComponentReflectError> for ComponentApplyError {
    fn from(value: ComponentReflectError) -> Self {
        Self::ReflectionError(value)
    }
}
