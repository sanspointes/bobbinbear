mod types;

use std::ops::Sub;

use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_mod_raycast::{RaycastSource, RaycastMesh};
use bevy_prototype_lyon::{shapes, prelude::{GeometryBuilder, ShapeBundle}};

use crate::systems::camera::sys_setup_camera;

pub use self::types::{RawInputMessage, InputMessage, ModifiersState};

#[derive(Debug, Clone, Reflect)]
pub struct RaycastRawInput;

const DRAG_THRESHOLD: f32 = 10.;
const BG_HIT_Z_INDEX: f32 = 50.;

/// The input processor plugin processes raw input (mouse down/up, move, etc)
/// into more useful events like Click, DragStart, move, etc.
///
/// These events can be listened to Via the EventReceiver<InputMessage> type.
pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RawInputResource::default())
            .add_event::<RawInputMessage>()
            .add_event::<InputMessage>()
            // Hit plane creation and movement
            .add_systems(Startup, sys_setup_bg_hit_plane.before(sys_setup_camera))
            .add_systems(PreUpdate, sys_move_bg_hit_plane)
            // Input events
            .add_systems(
                PreUpdate,
                // Convert raw inputs into
                (
                    // These systems take raw input events and pass them to the processor.
                    (
                        sys_mouse_button_input,
                        sys_mouse_movement_input,
                        sys_keyboard_input,
                    ),
                    // The processor will then 
                    sys_raw_input_processor,
                )
                    .chain(),
            );
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

/// Processes the Raw input events into more useful app events.
///
/// * `res`: 
/// * `ev_reader`: 
/// * `ev_writer`: 
/// * `bg_hit_query`: 
pub fn sys_raw_input_processor(
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
            // debug_log!("Warn: Input system cannot get world position of mouse!.");
        }
    } else {
        // debug_log!("Warn: Input system cannot get background entity.");
    }

    for msg in ev_reader.iter() {
        match msg {
            RawInputMessage::PointerMove(move_model) => {
                res.cur_pos.x = move_model.x as f32;
                res.cur_pos.y = move_model.y as f32;

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
                    })
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
                KeyCode::ControlLeft
                | KeyCode::ControlRight
                | KeyCode::SuperLeft
                | KeyCode::SuperRight => {
                    res.modifiers.command = *pressed;
                    ev_writer.send(InputMessage::ModifiersChanged {
                        state: res.modifiers.clone(),
                    });
                }
                KeyCode::AltLeft | KeyCode::AltRight => {
                    res.modifiers.alt = *pressed;
                    ev_writer.send(InputMessage::ModifiersChanged {
                        state: res.modifiers.clone(),
                    });
                }
                KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                    res.modifiers.shift = *pressed;
                    ev_writer.send(InputMessage::ModifiersChanged {
                        state: res.modifiers.clone(),
                    });
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

fn sys_mouse_button_input(
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

fn sys_mouse_movement_input(
    mut mousebtn_events: EventReader<CursorMoved>,
    mut message_writer: EventWriter<RawInputMessage>,
) {
    for ev in mousebtn_events.iter() {
        message_writer.send(RawInputMessage::PointerMove(ev.position));
    }
}

fn sys_keyboard_input(
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

// BG Hit plane, responsible for proving the background color +
// mapping mouse events to world coordinates

#[derive(Component)]
pub struct BgHitPlane;

fn sys_setup_bg_hit_plane(mut commands: Commands) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(10000., 10000.),
        ..Default::default()
    };
    commands.spawn((
        Name::from("BgHitPlane"),
        BgHitPlane {},
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform {
                translation: Vec3::new(0., 0., BG_HIT_Z_INDEX),
                ..Default::default()
            },
            ..Default::default()
        },
        RaycastMesh::<RaycastRawInput>::default(),
    ));
}
fn sys_move_bg_hit_plane(
    cam: Query<&Transform, (With<Camera2d>, Without<BgHitPlane>)>,
    mut bg_hit_plane: Query<&mut Transform, (With<BgHitPlane>, Without<Camera2d>)>) {
    if let (Ok(cam_transform), Ok(mut bg_hit_transform)) = (cam.get_single(), bg_hit_plane.get_single_mut()) {
        bg_hit_transform.translation.x = cam_transform.translation.x;
        bg_hit_transform.translation.y = cam_transform.translation.y;
    }
}
