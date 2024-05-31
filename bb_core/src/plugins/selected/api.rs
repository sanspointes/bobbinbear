use bevy::ecs::{query::Without, world::World};
use bevy_spts_changeset::commands_ext::WorldChangesetExt;
use bevy_spts_fragments::prelude::Uid;
use bevy_spts_uid::{UidRegistry, UidRegistryError};
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

use crate::{ecs::ProxiedComponent, plugins::undoredo::UndoRedoApi};

use super::{Hovered, ProxiedHovered, ProxiedSelected, Selected};

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

#[allow(dead_code)]
#[bevy_wasm_api]
impl SelectedApi {
    fn query_selected_uids(world: &mut World) -> Vec<Uid> {
        let selected: Vec<_> = world
            .query_filtered::<(&Uid, &Selected), Without<ProxiedSelected>>()
            .iter(world)
            .filter_map(|(uid, selected)| match selected {
                Selected::Selected => Some(*uid),
                Selected::Deselected => None,
            })
            .collect();
        selected
    }

    fn query_hovered_uids(world: &mut World) -> Vec<Uid> {
        let to_deselect: Vec<_> = world
            .query_filtered::<(&Uid, &Hovered), Without<ProxiedHovered>>()
            .iter(world)
            .filter_map(|(uid, hovered)| match hovered {
                Hovered::Hovered => Some(*uid),
                Hovered::Unhovered => None,
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
        let entity = uid.entity(world).unwrap();
        let target = match world.get::<ProxiedComponent<Selected>>(entity) {
            Some(proxy) => *proxy.target(),
            None => uid,
        };

        let mut changeset = world.changeset();
        changeset.entity(target).apply(selected);

        let changeset = changeset.build();
        UndoRedoApi::execute(world, changeset)?;

        Ok(())
    }

    pub fn deselect_all_set_object_selected(
        world: &mut World,
        uid: Uid,
        selected: Selected,
    ) -> Result<(), anyhow::Error> {
        let entity = uid.entity(world).unwrap();
        let target = match world.get::<ProxiedComponent<Selected>>(entity) {
            Some(proxy) => *proxy.target(),
            None => uid,
        };

        let to_deselect = SelectedApi::query_selected_uids(world);

        let mut changeset = world.changeset();
        for uid in to_deselect.iter() {
            changeset.entity(*uid).apply(Selected::Deselected);
        }

        changeset.entity(target).apply(selected);

        let changeset = changeset.build();
        UndoRedoApi::execute(world, changeset)?;

        Ok(())
    }

    pub fn unhover_all(world: &mut World) -> Result<(), anyhow::Error> {
        let curr_hovered = Self::query_hovered_uids(world);
        for uid in curr_hovered {
            if let Some(mut entity_mut) = uid.entity(world).and_then(|e| world.get_entity_mut(e)) {
                entity_mut.insert(Hovered::Unhovered);
            }
        }

        Ok(())
    }

    pub fn set_object_hovered(
        world: &mut World,
        uid: Uid,
        hovered: Hovered,
    ) -> Result<(), anyhow::Error> {
        let entity = uid.entity(world).unwrap();
        let target = match world.get::<ProxiedComponent<Hovered>>(entity) {
            Some(proxy) => *proxy.target(),
            None => uid,
        };

        if let Some(mut entity_mut) = target.entity(world).and_then(|e| world.get_entity_mut(e)) {
            entity_mut.insert(hovered);
        }
        Ok(())
    }

    pub fn unhover_all_set_object_hovered(
        world: &mut World,
        uid: Uid,
        hovered: Hovered,
    ) -> Result<(), anyhow::Error> {
        Self::unhover_all(world)?;
        Self::set_object_hovered(world, uid, hovered)?;
        Ok(())
    }
}
