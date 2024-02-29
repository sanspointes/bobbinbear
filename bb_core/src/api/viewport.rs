use bevy::prelude::*;
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

use crate::plugins::viewport::Viewport;

pub struct ViewportApi;

#[bevy_wasm_api]
#[allow(dead_code)]
impl ViewportApi {
    pub fn set_zoom(world: &mut World, zoom_level: f32)  {
        let mut viewport = world.query::<&mut Viewport>().single_mut(world);
        viewport.set_zoom(zoom_level);
    }

    pub fn get_zoom(world: &mut World) -> f32  {
        let mut viewport = world.query::<&mut Viewport>().single_mut(world);
        viewport.get_zoom()
    }

    pub fn set_position(world: &mut World, x: f32, y: f32)  {
        let mut viewport = world.query::<&mut Viewport>().single_mut(world);
        viewport.set_position(Vec2::new(x, y));
    }
}
