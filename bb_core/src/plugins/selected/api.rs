use bevy::ecs::world::World;
use bevy_spts_changeset::commands_ext::WorldChangesetExt;
use bevy_spts_fragments::prelude::Uid;
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

use crate::plugins::undoredo::UndoRedoApi;

use super::Selected;

#[allow(non_snake_case)]
mod definitions {
    // use serde::{Deserialize, Serialize};
    // use tsify::Tsify;
    //
    // #[derive(Tsify, Serialize, Deserialize)]
    // #[tsify(into_wasm_abi, from_wasm_abi)]
    // pub enum UndoRedoResult {
    //     NothingToDo,
    //     PerformedChange,
    // }
}

#[derive(Clone, Copy)]
pub struct SelectedApi;

impl SelectedApi {
    fn query_selected_uids(world: &mut World) -> Vec<Uid> {
        let to_deselect: Vec<_> = world
            .query::<(&Uid, &Selected)>()
            .iter(world)
            .filter_map(|(uid, selected)| match selected {
                Selected::Selected => Some(*uid),
                Selected::Deselected => None,
            })
            .collect();
        to_deselect
    }
    /// Deselects all objects in scene
    ///
    /// * `world`:
    pub fn deselect_all(world: &mut World) -> Result<(), anyhow::Error> {
        let to_deselect = SelectedApi::query_selected_uids(world);

        let mut changeset = world.changeset();
        for uid in to_deselect.iter() {
            changeset.entity(*uid).apply(Selected::Deselected);
        }

        let changeset = changeset.build();
        UndoRedoApi::execute(world, changeset)?;

        Ok(())
    }

    pub fn set_object_selected(
        world: &mut World,
        uid: Uid,
        selected: Selected,
    ) -> Result<(), anyhow::Error> {
        let mut changeset = world.changeset();
        changeset.entity(uid).apply(selected);

        let changeset = changeset.build();
        UndoRedoApi::execute(world, changeset)?;

        let selected = SelectedApi::query_selected_uids(world);

        Ok(())
    }

    pub fn deselect_all_set_object_selected(
        world: &mut World,
        uid: Uid,
        selected: Selected,
    ) -> Result<(), anyhow::Error> {
        let to_deselect = SelectedApi::query_selected_uids(world);

        let mut changeset = world.changeset();
        for uid in to_deselect.iter() {
            changeset.entity(*uid).apply(Selected::Deselected);
        }
        changeset.entity(uid).apply(selected);

        let changeset = changeset.build();
        UndoRedoApi::execute(world, changeset)?;

        let selected = SelectedApi::query_selected_uids(world);

        Ok(())
    }
}

#[allow(dead_code)]
#[bevy_wasm_api]
impl SelectedApi {
    pub fn deselect_all_js(world: &mut World) -> Result<(), anyhow::Error> {
        SelectedApi::deselect_all(world)
    }

    pub fn set_object_selected_js(
        world: &mut World,
        uid: String,
        selected: bool,
    ) -> Result<(), anyhow::Error> {
        SelectedApi::set_object_selected(world, Uid::try_from(&uid)?, selected.into())
    }

    pub fn deselect_all_set_object_selected_js(
        world: &mut World,
        uid: String,
        selected: bool,
    ) -> Result<(), anyhow::Error> {
        SelectedApi::deselect_all_set_object_selected(world, Uid::try_from(&uid)?, selected.into())
    }
}
