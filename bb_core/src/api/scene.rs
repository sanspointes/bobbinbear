use anyhow::Context;
use bevy::{prelude::*, ecs::system::SystemState};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use super::{ execute_in_world, anyhow_result_to_js_result, ExecutionChannel };
use crate::changeset::{undo_change, redo_change, SpawnEntity};

#[wasm_bindgen]
pub(super) struct SceneApiApi;

#[wasm_bindgen]
impl SceneApiApi {
    #[wasm_bindgen]
    pub async fn spawn_box(&self, x: f32, y: f32) -> js_sys::Promise {
        future_to_promise(execute_in_world(ExecutionChannel::FrameEnd, |w| {
            let mut sys_state = SystemState::<(
                ResMut<Assets<Mesh>>,
                ResMut<Assets<ColorMaterial>>,
            )>::new(w);
            let (mut meshes, mut materials) = sys_state.get_mut(w);

            let mesh = meshes.add(shape::Quad::new(Vec2::new(10., 10.)).into());
            let material = materials.add(ColorMaterial::from(Color::WHITE));

            let build_change = || -> Result<_, anyhow::Error> {
                Ok(SpawnEntity::new_empty(None)
                    .with_component(mesh)?
                    .with_component(material)?
                    .with_component(Transform {
                        translation: Vec3::new(x, y, 0.),
                    ..Default::default()
                    })?
                    .with_component(GlobalTransform::default())?
                    .with_component(Visibility::default())?
                    .with_component(InheritedVisibility::default())?
                    .with_component(ViewVisibility::default())?
                )
            };

            let result = build_change().and_then(|change| change.apply(w));
            anyhow_result_to_js_result(undo_change(w))
        }))
    }
}
