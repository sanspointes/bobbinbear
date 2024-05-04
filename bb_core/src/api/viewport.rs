use bevy::{prelude::*, window::PrimaryWindow};
use bevy_wasm_api::bevy_wasm_api;
use wasm_bindgen::prelude::*;

use crate::plugins::viewport::BobbinViewport;

pub struct ViewportApi;

#[bevy_wasm_api]
#[allow(dead_code)]
impl ViewportApi {
    pub fn set_resolution(world: &mut World, physical_width: u32, physical_height: u32) {
        let mut window = world.query_filtered::<&mut Window, With<PrimaryWindow>>().single_mut(world);
        window.resolution.set_physical_resolution(physical_width, physical_height);
    }

    pub fn set_zoom(world: &mut World, zoom_level: f32)  {
        let mut viewport = world.query::<&mut BobbinViewport>().single_mut(world);
        viewport.set_zoom(zoom_level);
    }

    pub fn get_zoom(world: &mut World) -> f32  {
        let mut viewport = world.query::<&mut BobbinViewport>().single_mut(world);
        viewport.get_zoom()
    }

    pub fn set_position(world: &mut World, x: f32, y: f32)  {
        let mut viewport = world.query::<&mut BobbinViewport>().single_mut(world);
        viewport.set_position(Vec2::new(x, y));
    }
}
