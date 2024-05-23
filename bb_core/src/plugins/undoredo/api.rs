//! Contains the API for executing/undoing/redoing changesets.

use bevy::ecs::world::World;
use bevy_spts_changeset::prelude::{Changeset, ChangesetResource};
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

use super::{UndoRedoResource, UndoRedoTag};
use crate::plugins::undoredo::changeset_ext::CollapseChangeset;

#[allow(non_snake_case)]
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
            let mut res = world.resource_mut::<UndoRedoResource>();

            // Early check if changeset has same signature as previous and can be merged.
            if let Some(mut prev_changeset) = res.undo_stack.pop() {
                if let Ok(merged_changeset) = prev_changeset.merge_and_apply_mergable(&changeset, world, cx) {
                    let mut res = world.resource_mut::<UndoRedoResource>();

                    res.undo_stack.push(merged_changeset);
                    res.redo_stack.clear();

                    return Ok(UndoRedoResult::PerformedChange);
                }
            }

            let inverse = changeset.apply(world, cx)?;
            let mut res = world.resource_mut::<UndoRedoResource>();
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
                Some(change) => {
                    let inverse = change.apply(world, cx)?;

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
