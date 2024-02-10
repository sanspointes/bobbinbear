use bevy::{prelude::*, ecs::system::SystemState};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use super::{ execute_in_world, anyhow_result_to_js_result, ExecutionChannel };
use crate::{changeset::{undo_change, SpawnEntity, execute_change}, ecs::core::{DerivedMesh, DerivedMaterial}};

#[wasm_bindgen]
pub(super) struct SceneApiApi;

#[wasm_bindgen]
impl SceneApiApi {
    #[wasm_bindgen]
    pub async fn spawn_box(&self, x: f32, y: f32) -> js_sys::Promise {
        future_to_promise(execute_in_world(ExecutionChannel::FrameEnd, move |w| {
            let mut change = SpawnEntity::new_empty(None);
            change
                .with_component(DerivedMesh)
                .with_component(DerivedMaterial)
                .with_component(Transform {
                    translation: Vec3::new(x, y, 0.),
                ..Default::default()
                })
                .with_component(GlobalTransform::default())
                .with_component(Visibility::default())
                .with_component(InheritedVisibility::default())
                .with_component(ViewVisibility::default())
            ;

            anyhow_result_to_js_result(execute_change(w, change))
        }))
    }
}
