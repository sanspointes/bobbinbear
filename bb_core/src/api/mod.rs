mod scene;
mod debug;

use bevy::{app::AppExit, prelude::*};
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

pub struct AppApi;

#[bevy_wasm_api]
impl AppApi {
    pub async fn exit(world: &mut World) {
        let mut exit_events = world.resource_mut::<Events<AppExit>>();
        exit_events.send(AppExit);
    }
}
