mod heirarchy;
mod insert;
mod spawn;

use std::{any::Any, fmt::Debug, sync::Arc};

use bevy_ecs::world::World;

use crate::resource::ChangesetContext;

pub use self::{heirarchy::*, insert::*, spawn::*};

pub trait Change: Debug + Any + Send + Sync {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn apply(
        &self,
        world: &mut World,
        context: &mut ChangesetContext,
    ) -> Result<Arc<dyn Change>, anyhow::Error>;
}
