use bevy::prelude::*;

use self::sync::{execute_world_tasks_begin, execute_world_tasks_end};

mod sync;

pub struct IpcPlugin {}

impl Plugin for IpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, execute_world_tasks_begin)
            .add_systems(Last, execute_world_tasks_end);
    }
}
