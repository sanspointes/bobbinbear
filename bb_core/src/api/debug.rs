use bevy::{app::AppExit, ecs::system::SystemState, prelude::*};
use bevy_spts_changeset::commands_ext::WorldChangesetExt;
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

use crate::undoredo::{UndoRedoApi, UndoRedoResult};

pub struct DebugApi;

#[bevy_wasm_api]
impl DebugApi {
    pub fn spawn_circle(world: &mut World) -> Result<UndoRedoResult, anyhow::Error> {
        let mut sys_state = SystemState::<(
            ResMut<Assets<Mesh>>,
            ResMut<Assets<ColorMaterial>>,
        )>::new(world);

        let (mut meshes, mut materials) = sys_state.get_mut(world);

        let mesh = meshes.add(shape::Circle::new(25.).into());
        let material = materials.add(Color::RED.into());

        let mut builder = world.changeset();
        let mut entity = builder.spawn_empty();
        entity
            .insert(Name::from("Debug Circle"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .insert(mesh)
            .insert(material)
            .insert(Visibility::default())
            .insert(ViewVisibility::default())
            .insert(InheritedVisibility::default());
        let changeset = builder.build();

        UndoRedoApi::execute(world, changeset)
    }
}

