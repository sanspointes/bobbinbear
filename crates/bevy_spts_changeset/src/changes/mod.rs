mod heirarchy;
mod insert;
mod spawn;

use std::{fmt::Debug, sync::Arc};

use bevy_ecs::world::World;

use crate::resource::ChangesetContext;

pub use self::{heirarchy::*, insert::*, spawn::*};

pub trait Change: Debug + Send + Sync {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn apply(
        &self,
        world: &mut World,
        context: &mut ChangesetContext,
    ) -> Result<Arc<dyn Change>, anyhow::Error>;
}
