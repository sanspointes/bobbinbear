//! Contains the API for executing/undoing/redoing changesets.

use bevy::{ecs::world::World, log::warn, time::Time};
use bevy_spts_changeset::prelude::{Changeset, ChangesetResource};
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

use super::{UndoRedoResource, UndoRedoTag};

#[allow(non_snake_case, clippy::empty_docs)]
mod definitions {
    use serde::{Deserialize, Serialize};
    use tsify::Tsify;

    #[derive(Debug, Clone, Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub enum UndoRedoResult {
        NothingToDo,
        PerformedChange,
    }
}

pub use definitions::*;

#[derive(Clone, Copy)]
pub struct UndoRedoApi;

impl UndoRedoApi {
    pub fn execute(
        world: &mut World,
        changeset: Changeset,
    ) -> Result<UndoRedoResult, anyhow::Error> {
        ChangesetResource::<UndoRedoTag>::context_scope(world, |world, cx| {
            let time = world.resource::<Time>();
            let current_seconds = time.elapsed_seconds_f64();

            let mut res = world.resource_mut::<UndoRedoResource>();
            let since_last_execute_seconds = current_seconds - res.last_execute_seconds;

            let prev_changeset = if since_last_execute_seconds < 0.5 {
                res.undo_stack.pop()
            } else {
                None
            };

            let inverse = if let Some(prev_changeset) = prev_changeset {
                match changeset.try_apply_repeatable(world, cx, &prev_changeset) {
                    Ok(inverse) => {
                        inverse
                    }
                    Err(_) => {
                        let mut res = world.resource_mut::<UndoRedoResource>();
                        res.undo_stack.push(prev_changeset);

                        warn!("Executingchangeset {changeset:?}");
                        changeset.apply(world, cx)?
                    }
                }
            } else {
                changeset.apply(world, cx)?
            };

            let mut res = world.resource_mut::<UndoRedoResource>();
            res.last_execute_seconds = current_seconds;

            res.undo_stack.push(inverse);
            res.redo_stack.clear();

            Ok(UndoRedoResult::PerformedChange)
        })
    }
}

#[allow(dead_code)]
#[bevy_wasm_api]
impl UndoRedoApi {
    pub fn undo(world: &mut World) -> Result<UndoRedoResult, anyhow::Error> {
        ChangesetResource::<UndoRedoTag>::context_scope(world, |world, cx| {
            let mut res = world.resource_mut::<UndoRedoResource>();
            let prev = res.undo_stack.pop();
            match prev {
                Some(changeset) => {
                    let inverse = changeset.apply(world, cx)?;

                    warn!("Applying undo changeset {changeset:?}");
                    let mut res = world.resource_mut::<UndoRedoResource>();
                    res.redo_stack.push(inverse);
                    Ok(UndoRedoResult::PerformedChange)
                }
                None => Ok(UndoRedoResult::NothingToDo),
            }
        })
    }

    pub fn redo(world: &mut World) -> Result<UndoRedoResult, anyhow::Error> {
        ChangesetResource::<UndoRedoTag>::context_scope(world, |world, cx| {
            let mut res = world.resource_mut::<UndoRedoResource>();
            let prev = res.redo_stack.pop();
            match prev {
                Some(change) => {
                    let inverse = change.apply(world, cx)?;

                    let mut res = world.resource_mut::<UndoRedoResource>();
                    res.undo_stack.push(inverse);
                    Ok(UndoRedoResult::PerformedChange)
                }
                None => Ok(UndoRedoResult::NothingToDo),
            }
        })
    }
}
