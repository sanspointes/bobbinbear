//! Contains an extension to the bevy_spts_changeset crate that will allow you to "collapse"
//! changesets into previous changesets.  This is for "deduping" changesets in the undo/redo stacks
//! so that if a user moves the same element 3 times it only causes a single changeset in the undo
//! stack.

use bevy::{ecs::world::World,utils::thiserror::Error};
use bevy_spts_changeset::{
    builder::Changeset,
    changes::{ApplyChange, Change},
    resource::ChangesetContext,
};
use core::panic;
use std::{any::{Any, TypeId}, sync::Arc};


struct AnyWrapper<'a> {
    pub any: &'a dyn Any,
}
/// Creates a Wrapped &dyn Any of T.  Wrapping is useful to get around static lifetimes
///
/// Example:
/// let wrapper = my_trait_object.as_any_wrapped();
/// if let Some(any_ref) = wrapper.any.downcast_ref::<MyStruct>() {
trait AsAnyWrapped {
    fn as_any_wrapped(&self) -> AnyWrapper<'_>;
    fn as_any_mut_wrapped(&mut self) -> AnyWrapper<'_>;
}

impl<T: Any> AsAnyWrapped for T {
    fn as_any_wrapped(&self) -> AnyWrapper<'_> {
        AnyWrapper { any: self }   
    }
    fn as_any_mut_wrapped(&mut self) -> AnyWrapper<'_> {
        AnyWrapper { any: self }   
    }
}

#[derive(Debug, Error)]
pub enum CollapseChangeError {
    #[error(
        "We have not implemented the merge for this change.  Typically this means the change applies a non-repeatable mutation that we should ignore."
    )]
    NoCollapseImplemented,
    #[error("The two changes have different signatures and are therefore not mergable.")]
    DifferentSignature,
}

trait CollapseChange: Change {
    fn can_merge(&self, _other: &dyn Change) -> Result<(), CollapseChangeError> {
        Err(CollapseChangeError::NoCollapseImplemented)
    }
    /// Checks whether this
    ///
    /// * `_other`:
    fn merge(&self, _other: &dyn Change) -> Result<Arc<dyn Change>, CollapseChangeError> {
        Err(CollapseChangeError::NoCollapseImplemented)
    }
}

impl<T: Change + ? Sized> CollapseChange for T {
    fn can_merge(&self, other: &dyn Change) -> Result<(), CollapseChangeError> {
        let self_ty = self.type_id();
        if self_ty != other.type_id() {
            return Err(CollapseChangeError::DifferentSignature)
        }

        if self.type_id() == TypeId::of::<ApplyChange>() {
            return Ok(())
        }

        Err(CollapseChangeError::NoCollapseImplemented)
    }

    fn merge(&self, other: &dyn Change) -> Result<Arc<dyn Change>, CollapseChangeError> {
        let self_ty = self.type_id();
        if self_ty != other.type_id() {
            return Err(CollapseChangeError::DifferentSignature)
        }
        if self.type_id() == TypeId::of::<ApplyChange>() {
            let self_wrapper = ( *self ).as_any_wrapped();
            let other_wrapper = ( *other ).as_any_wrapped();
            let self_change = self_wrapper.any.downcast_ref::<ApplyChange>().unwrap();
            let other_change = other_wrapper.any.downcast_ref::<ApplyChange>().unwrap();
            merge_apply_change(self_change, other_change);
        }

        Err(CollapseChangeError::NoCollapseImplemented)
    }
}



fn merge_apply_change(a: &ApplyChange, b: &ApplyChange) -> Result<Arc<dyn Change>, CollapseChangeError> {
    if a.target() != b.target() {
        return Err(CollapseChangeError::NoCollapseImplemented);
    }

    let to_compare = a.components().iter().zip(b.components());

    for (self_component, other_component) in to_compare {
        match (self_component.try_type_id(), other_component.try_type_id()) {
            (Ok(self_id), Ok(other_id)) => {
                if self_id != other_id {
                    return Err(CollapseChangeError::DifferentSignature);
                }
            }
            _ => return Err(CollapseChangeError::DifferentSignature),
        }
    }

    Ok(Arc::new(b.clone()))
}

pub trait CollapseChangeset {
    fn merge_and_apply_mergable(&mut self, other: &Self, world: &mut World, cx: &mut ChangesetContext) -> Result<Changeset, CollapseChangeError>;
}

impl CollapseChangeset for Changeset {
    /// Tries to merge a changeset with another changeset, throws an error if it's not mergable.
    /// Returns the updated changeset if it did merge changes.
    ///
    /// * `other`:
    fn merge_and_apply_mergable(&mut self, other: &Self, world: &mut World, cx: &mut ChangesetContext) -> Result<Changeset, CollapseChangeError> {
        if self.changes().len() != other.changes().len() {
            return Err(CollapseChangeError::DifferentSignature);
        }

        let to_compare = self.changes().iter().zip(other.changes());
        for (self_change, other_change) in to_compare {
            let self_change = &**self_change;
            let other_change = &**other_change;

            match self_change.can_merge(other_change) {
                Err(CollapseChangeError::DifferentSignature) => return Err(CollapseChangeError::DifferentSignature),
                _ => (),
            }
        }

        let to_compare = self.changes().iter().zip(other.changes());
        let mut changes = vec![];
        for (self_change, other_change) in to_compare {
            match ( &**self_change ).merge(&**other_change) {
                Ok(_) => {
                    if let Ok(inverse) = self_change.apply(world, cx) {
                        changes.push(inverse);
                    }
                },
                Err(CollapseChangeError::NoCollapseImplemented) => changes.push(*self_change),
                // Safety: Covered by `can_merge` above.
                Err(CollapseChangeError::DifferentSignature) => panic!("Impossible"),
            }
        }

        Ok(Changeset::new(changes))
    }
}
