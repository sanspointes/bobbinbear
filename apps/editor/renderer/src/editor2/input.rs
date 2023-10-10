use std::ops::Sub;

use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_mod_raycast::RaycastSource;

use super::{
    camera::{BgHitPlane, RaycastRawInput},
    msgs::{Tool, ToolMessage},
    Message,
};

// use super::{Message, document::DocumentMessage, tools::ToolMessage, camera::{BgHitPlane, RaycastRawInput}};

const DRAG_THRESHOLD: f32 = 10.;

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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSets {
    ReceiveInput,
    ProcessInput,
}

/// The input processor plugin processes raw input (mouse down/up, move, etc)
/// into more useful events like Click, DragStart, move, etc.
pub struct InputProcessorPlugin;

impl Plugin for InputProcessorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RawInputResource::default())
            .add_event::<RawInputMessage>()
            .add_event::<InputMessage>()
            // Fetch background hit entity if needed
            .add_state::<BgHitEntityFetchState>()
            .add_systems(PreUpdate, fetch_bg_hit_entity.run_if(in_state(BgHitEntityFetchState::Needed)))
            // Input events
            .configure_set(PreUpdate, InputSets::ReceiveInput)
            .configure_set(Update, InputSets::ProcessInput.after(InputSets::ReceiveInput))
            .add_systems(PreUpdate,
                (
                    mouse_button_input_system,
                    mouse_movement_input_system,
                    keyboard_input_system,
                ).in_set(InputSets::ReceiveInput)
            )
            .add_systems(Update, (
                    handle_raw_input_message_system,
                    tool_preprocess_system,
                ).chain().in_set(InputSets::ProcessInput)
            )
            // .add_system(handle_raw_input_message_system.in_set(EditorSet::ProcessRawInput))
        ;
    }
}

#[derive(States, Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum BgHitEntityFetchState {
    #[default]
    Needed,
    Fetched,
}

/// Fetches the BgHitPlane entity for use in raw input events
fn fetch_bg_hit_entity(
    mut res: ResMut<RawInputResource>,
    bg_hit_plane: Query<Entity, With<BgHitPlane>>,
) {
    let first = bg_hit_plane.single();
    res.bg_hit_entity = Some(first);
}

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

#[derive(Resource, Default)]
pub struct RawInputResource {
    bg_hit_entity: Option<Entity>,
    is_dragging: bool,
    left_pressed: bool,
    // right_pressed: bool,
    cur_pos: Vec2,
    down_pos: Vec2,
    down_pos_world: Vec3,
    modifiers: ModifiersState,
}

// INITIAL INPUT EVENTS
fn mouse_button_input_system(
    mut mousebtn_events: EventReader<MouseButtonInput>,
    mut message_writer: EventWriter<RawInputMessage>,
) {
    for ev in mousebtn_events.iter() {
        message_writer.send(RawInputMessage::PointerInput {
            pressed: ev.state,
            button: ev.button,
        })
    }
}

fn mouse_movement_input_system(
    mut mousebtn_events: EventReader<CursorMoved>,
    mut message_writer: EventWriter<RawInputMessage>,
) {
    for ev in mousebtn_events.iter() {
        message_writer.send(RawInputMessage::PointerMove(ev.position));
    }
}

fn keyboard_input_system(
    mut key_evr: EventReader<KeyboardInput>,
    mut message_writer: EventWriter<RawInputMessage>,
) {
    for ev in key_evr.iter() {
        match (ev.key_code, ev.state) {
            (Some(key_code), state) => {
                message_writer.send(RawInputMessage::KeyboardInput {
                    pressed: state,
                    key: key_code,
                });
            }
            (None, _) => {}
        }
    }
}

// RAW INPUT PROCESSED INTO MORE USEFUL INPUT EVENTS

pub fn handle_raw_input_message_system(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut res: ResMut<RawInputResource>,
    mut ev_reader: EventReader<RawInputMessage>,
    mut ev_writer: EventWriter<InputMessage>,
    bg_hit_query: Query<&RaycastSource<RaycastRawInput>>,
) {
    let mut world_point = Vec3::new(0., 0., 0.);
    if let Some(bg_hit_entity) = res.bg_hit_entity {
        let intersections = bg_hit_query.single().intersections();

        if let Some((_, data)) = intersections
            .iter()
            .find(|(entity, _)| *entity == bg_hit_entity)
        {
            world_point = data.position();
        } else {
            warn!("Warn: Input system cannot get world position of mouse!.");
        }
    } else {
        warn!("Warn: Input system cannot get background entity.");
    }

    let win = primary_window.single();

    let evts: Vec<_> = ev_reader.iter().enumerate().collect();

    let last_move_position = evts
        .iter()
        .rev()
        .find(|(_, ev)| matches!(ev, RawInputMessage::PointerMove(_)))
        .map(|(pos, _)| pos.clone());
    let evts = evts
        .into_iter()
        .filter(|(idx, ev)| {
            if matches!(ev, RawInputMessage::PointerMove(_)) {
                // Filter out PointerMove events that aren't the last one in the frame.
                return last_move_position
                    .map(|last_move_idx| last_move_idx == *idx)
                    .unwrap_or(true);
            } else {
                return true;
            }
        });

    // Dedupe multiple of the same types of events, taking only the last.
    for (i, msg) in evts {
        match msg {
            RawInputMessage::PointerMove(move_model) => {
                res.cur_pos.x = move_model.x;
                res.cur_pos.y = win.height() - move_model.y;

                if res.left_pressed
                    && !res.is_dragging
                    && res.cur_pos.distance(res.down_pos) > DRAG_THRESHOLD
                {
                    res.is_dragging = true;
                    ev_writer.send(InputMessage::DragStart {
                        screen: res.cur_pos,
                        screen_pressed: res.down_pos,
                        screen_offset: res.cur_pos.sub(res.down_pos),
                        world: world_point,
                        world_pressed: res.down_pos_world,
                        world_offset: world_point.sub(res.down_pos_world),
                        modifiers: res.modifiers.clone(),
                    });
                } else if res.is_dragging {
                    ev_writer.send(InputMessage::DragMove {
                        screen: res.cur_pos,
                        screen_pressed: res.down_pos,
                        screen_offset: res.cur_pos.sub(res.down_pos),
                        world: world_point,
                        world_pressed: res.down_pos_world,
                        world_offset: world_point.sub(res.down_pos_world),
                        modifiers: res.modifiers.clone(),
                    });
                } else {
                    ev_writer.send(InputMessage::PointerMove {
                        screen: res.cur_pos.clone(),
                        world: world_point,
                        modifiers: res.modifiers.clone(),
                    });
                }

                // screen_print!("mouse: \ncur_pos {:?}\ndown_pos {:?}\noffset {:?} offset\nleft_pressed {:?}\nis_dragging {:?}", world_point, res.down_pos_world, world_point.sub(res.down_pos_world), res.left_pressed, res.is_dragging);
            }
            RawInputMessage::PointerInput { pressed, button } => match (button, pressed) {
                (MouseButton::Left, ButtonState::Pressed) => {
                    if !res.left_pressed {
                        res.left_pressed = true;
                        res.down_pos = res.cur_pos.clone();
                        res.down_pos_world = world_point.clone();
                        ev_writer.send(InputMessage::PointerDown {
                            screen: res.cur_pos.clone(),
                            world: world_point,
                            modifiers: res.modifiers.clone(),
                        });
                    }
                }
                (MouseButton::Left, ButtonState::Released) => {
                    res.left_pressed = false;
                    if res.is_dragging {
                        res.is_dragging = false;
                        ev_writer.send(InputMessage::DragEnd {
                            screen: res.cur_pos,
                            screen_pressed: res.down_pos,
                            screen_offset: res.cur_pos.sub(res.down_pos),
                            world: world_point,
                            world_pressed: res.down_pos_world,
                            world_offset: world_point.sub(res.down_pos_world),
                            modifiers: res.modifiers.clone(),
                        });
                    } else {
                        ev_writer.send(InputMessage::PointerClick {
                            screen: res.cur_pos.clone(),
                            world: world_point,
                            modifiers: res.modifiers.clone(),
                        });
                    }
                }
                (_, _) => {}
            },
            RawInputMessage::KeyboardInput { pressed, key } => match key {
                KeyCode::ControlLeft | KeyCode::ControlRight | KeyCode::SuperLeft | KeyCode::SuperRight => {
                    res.modifiers.command = *pressed;
                    ev_writer.send(
                        InputMessage::ModifiersChanged {
                            state: res.modifiers.clone(),
                        }
                        .into(),
                    );
                }
                KeyCode::AltLeft | KeyCode::AltRight => {
                    res.modifiers.alt = *pressed;
                    ev_writer.send(
                        InputMessage::ModifiersChanged {
                            state: res.modifiers.clone(),
                        }
                        .into(),
                    );
                }
                KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                    res.modifiers.shift = *pressed;
                    ev_writer.send(
                        InputMessage::ModifiersChanged {
                            state: res.modifiers.clone(),
                        }
                        .into(),
                    );
                }
                key => {
                    ev_writer.send(InputMessage::Keyboard {
                        pressed: *pressed,
                        key: *key,
                        modifiers: res.modifiers.clone(),
                    });
                }
            },
        }
    }
}

/// TODO: Refactor this into a seperate plugin that manages keymaps
/// Pre-processes input before passing it to the tool handler in the next system.
/// This enables hotkeys like switching tools etc.
///
/// * `selected_tool`:
/// * `input_reader`:
/// * `tool_writer`:
pub fn tool_preprocess_system(
    selected_tool: Res<State<Tool>>,
    mut input_reader: EventReader<InputMessage>,
    mut msg_writer: EventWriter<Message>,
) {
    for msg in input_reader.iter() {
        // If this event is not handled by a hotkey handler, it gets passed to the tool
        let mut should_pass_through = false;
        let cur_tool = selected_tool.get();
        match msg {
            InputMessage::Keyboard {
                pressed,
                key,
                modifiers,
            } => match (pressed, key, modifiers.command, modifiers.shift) {
                // Click to drag around viewport with space key pressed
                (ButtonState::Pressed, KeyCode::Space, _, _) => {
                    if *cur_tool != Tool::Grab {
                        msg_writer.send(ToolMessage::PushTool(Tool::Grab).into());
                    }
                }
                (ButtonState::Released, KeyCode::Space, _, _) => {
                    if *cur_tool == Tool::Grab {
                        msg_writer.send(ToolMessage::ResetToRootTool.into());
                    }
                }
                (ButtonState::Released, KeyCode::Key1, _, _) => {
                    msg_writer.send(ToolMessage::SwitchTool(Tool::Select).into());
                }
                (ButtonState::Released, KeyCode::Key2, _, _) => {
                    msg_writer.send(ToolMessage::SwitchTool(Tool::Box).into());
                }
                (_, _, _, _) => {
                    should_pass_through = true;
                }
            },
            _ => {
                should_pass_through = true;
            }
        }

        if should_pass_through {
            msg_writer.send(ToolMessage::Input(msg.clone()).into());
        }
    }
}
