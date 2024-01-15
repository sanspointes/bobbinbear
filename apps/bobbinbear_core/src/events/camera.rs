use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub enum CameraEvent {
    Moved { translation: Vec2, delta: Vec2 },
    Zoomed { zoom_level: f32, delta: f32 },
}

impl CameraEvent {
    pub fn send(self, world: &mut World) {
        if let Some(mut events) = world.get_resource_mut::<Events<CameraEvent>>() {
            events.send(self);
        }
    }
}
