use anyhow::anyhow;
use bevy::{prelude::*, ecs::world};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use super::{ execute_in_world, anyhow_result_to_js_result, ExecutionChannel };
use crate::{
    changeset::{SpawnEntity, execute_change},
    ecs::{node::Node, core::{DerivedMesh, DerivedMaterial}}, index::Idx
};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct SceneApi;

#[wasm_bindgen]
impl SceneApi {
    #[wasm_bindgen]
    pub async fn list_objects(&self) -> js_sys::Promise {
        future_to_promise(execute_in_world(ExecutionChannel::FrameEnd, move |w| {
            let mut q_idx = w.query::<&Idx>();
            let iter_idx = q_idx.iter(w);
            let array = js_sys::Array::new_with_length(iter_idx.len() as u32);
            for idx in iter_idx {
                array.push(&serde_wasm_bindgen::to_value(idx).unwrap());
            }
            if false {
                return Err(JsValue::from_str("Never"));
            }
            Ok(array.into())
        }))
    }

    #[wasm_bindgen]
    pub async fn spawn_node(&self, x: f32, y: f32) -> js_sys::Promise {
        future_to_promise(execute_in_world(ExecutionChannel::FrameEnd, move |w| {
            let mut change = SpawnEntity::new_empty(None);
            change
                .with_component(Node::Endpoint)
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
