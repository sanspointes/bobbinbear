use anyhow::anyhow;

use bevy::prelude::*;
use bevy_spts_changeset::prelude::WorldChangesetExt;
use bevy_spts_fragments::prelude::Uid;
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

use crate::{
    ecs::{InternalObject, ObjectType, Position, ProxiedPosition},
    plugins::{
        inspecting::Inspected,
        selected::Selected,
        undoredo::{UndoRedoApi, UndoRedoResult},
    },
};

#[allow(unused_imports)]
pub use self::definitions::*;

#[allow(non_snake_case)]
mod definitions {
    use bevy::math::Vec2;
    use bevy_spts_uid::Uid;
    use serde::{Deserialize, Serialize};
    use tsify::Tsify;
    use wasm_bindgen::prelude::*;

    use crate::ecs::object::ObjectType;
    #[wasm_bindgen(typescript_custom_section)]
    const TS_APPEND_CONTENT: &'static str = r#"
export type Vec2 = [number, number]; 
export type Uid = string; 
    "#;

    #[derive(Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub struct DetailedObject {
        pub ty: ObjectType,
        pub uid: Uid,
        pub parent: Option<Uid>,
        pub name: Option<String>,
        pub visible: bool,
        pub selected: bool,
        pub inspected: bool,
        pub position: Vec2,
        pub children: Option<Vec<Uid>>,
    }
}

#[derive(Clone, Copy)]
pub struct SceneApi;

#[allow(dead_code)]
#[bevy_wasm_api]
impl SceneApi {
    /// JS ONLY:
    /// Returns a DescribedObject for a single object if it exists.
    /// * `world`:
    pub fn describe_object(
        world: &mut World,
        uid: Uid,
    ) -> Result<Option<DetailedObject>, anyhow::Error> {
        let Some(entity) = uid.entity(world) else {
            return Ok(None);
        };
        let parent = world.query::<&Parent>().get(world, entity).ok();
        let parent = parent.and_then(|parent| world.get::<Uid>(**parent).copied());
        let children = world
            .query::<&Children>()
            .get(world, entity)
            .ok()
            .map(|c| c.to_vec());
        let children = children.map(|children| {
            let mut q_uids = world.query::<&Uid>();
            children
                .iter()
                .filter_map(|e| q_uids.get(world, *e).ok().copied())
                .collect::<Vec<_>>()
        });

        Ok(world
            .query_filtered::<(
                &Uid,
                &ObjectType,
                Option<&Name>,
                &Visibility,
                &Transform,
                &Selected,
                Option<&Inspected>,
            ), Without<InternalObject>>()
            .get(world, entity)
            .ok()
            .map(
                |(uid, ty, name, visibility, transform, selected, inspected)| DetailedObject {
                    uid: *uid,
                    ty: *ty,
                    name: name.map(|name| name.to_string()),

                    parent,
                    children,

                    visible: matches!(visibility, Visibility::Inherited),
                    selected: matches!(selected, Selected::Selected),
                    inspected: inspected.is_some(),

                    position: transform.translation.xy(),
                },
            ))
    }

    pub fn log_scene(world: &mut World) -> String {
        let entities: Vec<Entity> = world.query::<Entity>().iter(world).collect();

        let info: Vec<_> = entities
            .iter()
            .map(|e| {
                let v = world.inspect_entity(*e);
                let info: Vec<_> = v.iter().map(|info| info.name()).collect();
                (e, info)
            })
            .collect();

        format!("{info:?}").to_string()
    }

    pub fn set_visible(
        world: &mut World,
        uid: Uid,
        visible: bool,
    ) -> Result<UndoRedoResult, anyhow::Error> {
        let visible = if matches!(visible, true) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };

        let mut builder = world.changeset();
        builder.entity(uid).apply(visible);

        let changeset = builder.build();

        UndoRedoApi::execute(world, changeset)
    }

    pub fn set_name(
        world: &mut World,
        uid: Uid,
        name: String,
    ) -> Result<UndoRedoResult, anyhow::Error> {
        let mut builder = world.changeset();
        builder.entity(uid).apply(Name::from(name));

        let changeset = builder.build();

        let result = UndoRedoApi::execute(world, changeset)?;

        Ok(result)
    }

    pub fn set_position(
        world: &mut World,
        uid: Uid,
        x: f32,
        y: f32,
    ) -> Result<UndoRedoResult, anyhow::Error> {
        let entity = uid.entity(world).unwrap();
        let target = match world.get::<ProxiedPosition>(entity) {
            Some(proxy) => *proxy.target(),
            None => uid,
        };

        let mut builder = world.changeset();
        builder.entity(target).apply(Position::new((x, y)));
        let changeset = builder.build();

        let result = UndoRedoApi::execute(world, changeset)?;

        Ok(result)
    }

    /// Inspects an object, uninspects the current inspected object if it has to.
    pub fn inspect(world: &mut World, uid: Uid) -> Result<UndoRedoResult, anyhow::Error> {
        let prev_inspected = world
            .query_filtered::<&Uid, With<Inspected>>()
            .get_single(world)
            .copied();

        let mut builder = world.changeset();

        if let Ok(uid) = prev_inspected {
            builder.entity(uid).remove::<Inspected>();
        }

        builder.entity(uid).insert(Inspected);
        let changeset = builder.build();

        UndoRedoApi::execute(world, changeset)
    }

    /// Uninspects the currently inspected object (if there is one).
    pub fn uninspect(world: &mut World) -> Result<UndoRedoResult, anyhow::Error> {
        let prev_inspected = world
            .query_filtered::<&Uid, With<Inspected>>()
            .get_single(world)
            .copied();

        let mut builder = world.changeset();

        if let Ok(uid) = prev_inspected {
            builder.entity(uid).remove::<Inspected>();
        }

        let changeset = builder.build();

        UndoRedoApi::execute(world, changeset)
    }

    pub fn delete(world: &mut World, uid: Uid) -> Result<UndoRedoResult, anyhow::Error> {
        let entity = uid.entity(world).unwrap();
        let is_inspected = world.get::<Inspected>(entity).is_some();
        let mut builder = world.changeset();
        if is_inspected {
            builder.entity(uid).remove::<Inspected>();
        }
        builder.entity(uid).despawn_recursive();
        let changeset = builder.build();

        UndoRedoApi::execute(world, changeset)
    }
}
