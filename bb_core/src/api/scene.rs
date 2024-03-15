use anyhow::anyhow;
use bevy::prelude::*;
use bevy_spts_changeset::prelude::WorldChangesetExt;
use bevy_spts_fragments::prelude::Uid;
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

use crate::undoredo::{UndoRedoApi, UndoRedoResult};

#[allow(unused_imports)]
pub use self::definitions::*;

#[allow(non_snake_case)]
mod definitions {
    use bevy::math::Vec2;
    use serde::{Deserialize, Serialize};
    use tsify::Tsify;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r#"
    export type Vec2 = [number, number]; 
    "#;


    #[derive(Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub struct DescribedObject {
        pub uid: String,
        pub name: Option<String>,
        pub visible: bool,
    }

    #[derive(Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub struct DetailedObject {
        pub uid: String,
        pub name: Option<String>,
        pub visible: bool,
        pub position: Vec2,
    }
}

#[derive(Clone, Copy)]
pub struct SceneApi;

#[allow(dead_code)]
#[bevy_wasm_api]
impl SceneApi {
    /// JS ONLY:
    /// Returns a Vec<DescribedObject> for all the objects within a scene.
    /// * `world`:
    pub fn describe_document(world: &mut World) -> Vec<DescribedObject> {
        world
            .query::<(&Uid, Option<&Name>, &Visibility)>()
            .iter(world)
            .map(|(uid, name, visibility)| DescribedObject {
                uid: uid.into(),
                name: name.map(|name| name.to_string()),
                visible: matches!(visibility, Visibility::Visible),
            })
            .collect()
    }

    /// JS ONLY:
    /// Returns a DescribedObject for a single object if it exists.
    /// * `world`:
    pub fn describe_object(
        world: &mut World,
        uid: String,
    ) -> Result<Option<DetailedObject>, anyhow::Error> {
        let uid = Uid::try_from(&uid)?;
        let Some(entity) = uid.entity(world) else {
            return Ok(None);
        };
        Ok(world
            .query::<(&Uid, Option<&Name>, &Visibility, &Transform)>()
            .get(world, entity)
            .ok()
            .map(|(uid, name, visibility, transform)| DetailedObject {
                uid: uid.into(),
                name: name.map(|name| name.to_string()),
                visible: matches!(visibility, Visibility::Visible),
                position: transform.translation.xy(),
            }))
    }

    pub fn log_scene(world: &mut World) -> String {
        let e: Vec<_> = world.query::<Entity>().iter(world).collect();

        let info: Vec<_> = e
            .iter()
            .map(|e| {
                let v = world.inspect_entity(*e);
                let info: Vec<_> = v.iter().map(|info| info.name()).collect();
                (e, info)
            })
            .collect();

        format!("{info:?}").to_string()
    }

    pub fn set_visible(world: &mut World, uid: String, visible: bool) -> Result<(), anyhow::Error> {
        let uid: Uid = (&uid).try_into()?;
        let entity = uid
            .entity(world)
            .ok_or_else(|| anyhow!("No entity for uid {uid}."))?;
        let mut visibility = world
            .get_mut::<Visibility>(entity)
            .ok_or_else(|| anyhow!("No `Visibility` component on entity with uid {uid}."))?;
        match visible {
            true => *visibility = Visibility::Visible,
            false => *visibility = Visibility::Hidden,
        }
        Ok(())
    }

    pub fn set_name(
        world: &mut World,
        uid: String,
        name: String,
    ) -> Result<UndoRedoResult, anyhow::Error> {
        let uid = Uid::try_from(&uid)?;

        let mut builder = world.changeset();
        builder.entity(uid).apply(Name::from(name));

        let changeset = builder.build();

        UndoRedoApi::execute(world, changeset)
    }

    pub fn set_position(
        world: &mut World,
        uid: String,
        x: f32,
        y: f32,
    ) -> Result<UndoRedoResult, anyhow::Error> {
        let uid = Uid::try_from(&uid)?;
        let entity = uid
            .entity(world)
            .ok_or_else(|| anyhow!("No entity for uid {uid}."))?;

        let mut transform = *world
            .get::<Transform>(entity)
            .ok_or_else(|| anyhow!("No `Transform` component on entity with uid {uid}."))?;
        transform.translation.x = x;
        transform.translation.y = y;

        let mut builder = world.changeset();
        builder.entity(uid).apply(transform);

        let changeset = builder.build();

        UndoRedoApi::execute(world, changeset)
    }
}
