//! Plugin creates an entity with the Viewport component attached to it.  Children of this entity
//! will be position in screenspace.

mod debug;
mod systems;

use bevy::{ecs::reflect::ReflectComponent, prelude::*};

use crate::plugins::viewport::systems::*;

use self::debug::{sys_setup_viewport_debug, sys_update_viewport_debug_positions};

pub struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BobbinViewport>()
            .add_systems(
                Startup,
                (sys_setup_viewport, sys_setup_viewport_debug).chain(),
            )
            .add_systems(PreUpdate, sys_update_viewport_on_window_resize)
            // .add_systems(PostUpdate, sys_update_camera_from_viewport)
            .add_systems(PostUpdate, sys_update_viewport_debug_positions);
    }
}

#[derive(Component, Reflect, Default, Debug, Copy, Clone, PartialEq)]
#[reflect(Component)]
pub struct BobbinViewport {
    pub logical_size: Vec2,
    pub physical_size: Vec2,
    pub target_position: Vec2,
}

#[allow(dead_code)]
impl BobbinViewport {
    pub fn set_zoom(&mut self, zoom: f32) {
        todo!();
    }
    pub fn get_zoom(&mut self) -> f32 {
        todo!();
    }
    pub fn set_position(&mut self, position: Vec2) {
        self.target_position = position;
    }

    pub fn ndc_to_viewport(&self, ndc: Vec2) -> Vec2 {
        ndc * self.logical_size / 2.
    }
}
