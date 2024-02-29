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
        app.register_type::<Viewport>()
            .add_systems(
                Startup,
                (sys_setup_viewport, sys_setup_viewport_debug).chain(),
            )
            .add_systems(PreUpdate, sys_update_viewport_on_window_resize)
            .add_systems(PostUpdate, sys_update_camera_from_viewport)
            .add_systems(PostUpdate, sys_update_viewport_debug_positions);
    }
}

#[derive(Component, Reflect, Default, Debug, Copy, Clone, PartialEq)]
#[reflect(Component)]
pub struct Viewport {
    pub window_size: Vec2,
    pub target_position: Vec2,
    pub target_size: Vec2,
}

#[allow(dead_code)]
impl Viewport {
    pub fn set_zoom(&mut self, zoom: f32) {
        self.target_size = self.window_size * zoom;
    }
    pub fn get_zoom(&mut self) -> f32 {
        self.target_size.length() / self.window_size.length()
    }
    pub fn set_position(&mut self, position: Vec2) {
        self.target_position = position;
    }

    pub fn half_size(&self) -> Vec2 {
        self.target_size / 2.
    }

    pub fn view_rect(&self) -> Rect {
        let half_size = self.target_size / 2.;
        Rect::new(
            self.target_position.x - half_size.x,
            self.target_position.y - half_size.y,
            self.target_position.x + half_size.x,
            self.target_position.y + half_size.y,
        )
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let norm_pos = screen_pos / self.window_size;
        self.normalized_screen_to_world(norm_pos)
    }
    pub fn normalized_screen_to_world(&self, norm_pos: Vec2) -> Vec2 {
        norm_pos.mul_add(self.target_size, self.target_position - self.half_size())
    }
}

#[derive(Component)]
pub struct WorldToScreen {
    world_pos: Vec2,
}
