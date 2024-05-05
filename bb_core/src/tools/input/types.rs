use bevy::{prelude::*, input::ButtonState};


#[derive(Debug, Copy, Clone, Reflect)]
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
#[derive(Event, Reflect, Debug, Clone, Copy)]
pub enum InputMessage {
    PointerDown {
        screen_pos: Vec2,
        world_pos: Vec2,
        modifiers: ModifiersState,
    },
    PointerMove {
        screen_pos: Vec2,
        world_pos: Vec2,
        modifiers: ModifiersState,
    },
    PointerClick {
        screen_pos: Vec2,
        world_pos: Vec2,
        modifiers: ModifiersState,
    },
    DoubleClick {
        screen_pos: Vec2,
        world_pos: Vec2,
        modifiers: ModifiersState,
    },
    PointerOptionClick {
        screen_pos: Vec2,
        world_pos: Vec2,
        modifiers: ModifiersState,
    },
    DragStart {
        screen_pos: Vec2,
        screen_delta_pos: Vec2,
        screen_start_pos: Vec2,
        world_pos: Vec2,
        world_delta_pos: Vec2,
        world_start_pos: Vec2,
        modifiers: ModifiersState,
    },
    DragMove {
        screen_pos: Vec2,
        screen_delta_pos: Vec2,
        screen_start_pos: Vec2,
        world_pos: Vec2,
        world_delta_pos: Vec2,
        world_start_pos: Vec2,
        modifiers: ModifiersState,
    },
    DragEnd {
        screen_pos: Vec2,
        screen_delta_pos: Vec2,
        screen_start_pos: Vec2,
        world_pos: Vec2,
        world_delta_pos: Vec2,
        world_start_pos: Vec2,
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
