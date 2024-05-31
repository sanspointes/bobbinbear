mod heirarchy;
mod insert;
mod spawn;

use std::{any::Any, fmt::Debug, sync::Arc};

use bevy_ecs::world::World;
use thiserror::Error;

use crate::resource::ChangesetContext;

pub use self::{heirarchy::*, insert::*, spawn::*};

#[derive(Error, Debug)]
pub enum NotRepeatableReason {
    #[error("These two changes represent a different type.  Comparing {0} to {1}.")]
    DifferentType(&'static str, &'static str),
    #[error("These two changes are the same type but have different content.")]
    DifferentContent,
    #[error("This change modifies the World layout and therefore cannot be repeated (but is safe to be ignored when applying repeatable).")]
    ChangesWorldLayout,
}

pub trait Change: AsAnyArc + Debug + Any + Send + Sync {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn apply(
        &self,
        world: &mut World,
        context: &mut ChangesetContext,
    ) -> Result<Arc<dyn Change>, anyhow::Error>;

    fn is_repeatable(&self, other: Arc<dyn Change>) -> Result<(), NotRepeatableReason>;
}

pub trait AsAnyArc {
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any>;
}

impl<T: 'static> AsAnyArc for T {
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any> { self }
}

