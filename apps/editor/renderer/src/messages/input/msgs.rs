use bevy::{prelude::*, input::ButtonState};

#[derive(Debug, Copy, Clone)]
pub struct ModifiersState {
    pub command: ButtonState,
    pub alt: ButtonState,
    pub shift: ButtonState,
}

impl Default for ModifiersState {
    fn default() -> Self {
        Self {
            command: ButtonState::Released,
            alt: ButtonState::Released,
            shift: ButtonState::Released,
        }
    }
}


pub struct KeyboardInputModel {
    key: KeyCode,
    pressed: ButtonState,
}

// Raw Input messages passed from winit
#[derive(Event, Debug, Clone)]
pub enum RawInputMessage {
    PointerMove(Vec2),
    PointerInput {
        pressed: ButtonState,
        button: MouseButton,
    },
    KeyboardInput {
        pressed: ButtonState,
        key: KeyCode,
    },
}

// Processed / abstracted input events for common behaviour like click
#[derive(Event, Debug, Clone, Copy)]
pub enum InputMessage {
    PointerDown {
        screen: Vec2,
        world: Vec3,
        modifiers: ModifiersState,
    },
    PointerMove {
        screen: Vec2,
        world: Vec3,
        modifiers: ModifiersState,
    },
    PointerClick {
        screen: Vec2,
        world: Vec3,
        modifiers: ModifiersState,
    },
    PointerOptionClick {
        screen: Vec2,
        world: Vec3,
        modifiers: ModifiersState,
    },
    DragStart {
        screen: Vec2,
        screen_offset: Vec2,
        screen_pressed: Vec2,
        world: Vec3,
        world_offset: Vec3,
        world_pressed: Vec3,
        modifiers: ModifiersState,
    },
    DragMove {
        screen: Vec2,
        screen_offset: Vec2,
        screen_pressed: Vec2,
        world: Vec3,
        world_offset: Vec3,
        world_pressed: Vec3,
        modifiers: ModifiersState,
    },
    DragEnd {
        screen: Vec2,
        screen_offset: Vec2,
        screen_pressed: Vec2,
        world: Vec3,
        world_offset: Vec3,
        world_pressed: Vec3,
        modifiers: ModifiersState,
    },
    Keyboard {
        pressed: ButtonState,
        key: KeyCode,
        modifiers: ModifiersState,
    },
    ModifiersChanged {
        state: ModifiersState,
    },
}

