//! 

use bevy_spts_fragments::prelude::Uid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComponentMismatchError {
    #[error("The entity is not parented to {0}.")]
    MissingParent(Uid),
    #[error("The entity expected to have parent {0} but instead found {1}.")]
    DifferentParent(Uid, Uid),
    #[error("The entity expected to have parent {0} but the parent entity does not have a Uid.")]
    NoUidOnParent(Uid),
    #[error("The entity expected to have component {0} but it didn't exist on the struct.")]
    MissingComponent(String),
}

#[derive(Error, Debug)]
pub enum ChangeError {
    #[error("No entities with uid {0} exist.")]
    NoEntity(Uid),
    #[error("The components on entity {0} are unexpected.  {1}")]
    ComponentMismatch(Uid, ComponentMismatchError)
}

impl ChangeError {
    pub fn component_mismatch_missing_parent(child: Uid, parent: Uid) -> Self {
        Self::ComponentMismatch(child, ComponentMismatchError::MissingParent(parent))
    }
    pub fn component_mismatch_different_parent(child: Uid, expected: Uid, found: Uid) -> Self {
        Self::ComponentMismatch(child, ComponentMismatchError::DifferentParent(expected, found))
    }
    pub fn component_mismatch_no_uid_on_parent(child: Uid, parent: Uid) -> Self {
        Self::ComponentMismatch(child, ComponentMismatchError::NoUidOnParent(parent))
    }
    pub fn component_mismatch_missing_component(child: Uid, component_name: String) -> Self {
        Self::ComponentMismatch(child, ComponentMismatchError::MissingComponent(component_name))
    }
}
