//! Contains the logic for all the different fragments that can be reflected.

pub mod component;
pub mod bundle;
pub mod entity;
pub mod hierarchy;

pub mod prelude {
    pub use crate::component::{ComponentFragment, ComponentFragmentError, ComponentToFragment};
    pub use crate::bundle::{BundleFragment, BundleToFragment};
    pub use crate::entity::{EntityFragment, EntityFragmentNewError, EntityFragmentSpawnError};
    pub use crate::hierarchy::{HierarchyFragment, HierarchyFragmentSpawnError, HierarchyFragmentNewError};
    pub use bevy_spts_uid::Uid;
}
