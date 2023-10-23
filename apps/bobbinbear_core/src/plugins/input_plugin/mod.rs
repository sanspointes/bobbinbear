mod types;

use std::{default, mem::discriminant, ops::Sub};

use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState},
    math::Vec3Swizzles,
    prelude::*,
    utils::HashSet,
};
use bevy_mod_raycast::{
    DefaultRaycastingPlugin, RaycastMesh, RaycastMethod, RaycastSource, RaycastSystem,
};
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes,
};
use js_sys::Reflect::construct_with_new_target;

use crate::{editor::EditorSet, systems::camera::CameraTag};

pub use self::types::{InputMessage, ModifiersState, RawInputMessage};

#[derive(Debug, Clone, Reflect)]
pub struct RaycastRawInput;

const DRAG_THRESHOLD: f32 = 3.;
const BG_HIT_Z_INDEX: f32 = -100.;

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
            .add_plugins(DefaultRaycastingPlugin::<RaycastRawInput>::default())
            // Hit plane creation and movement
            .add_systems(Startup, sys_setup_input_plugin)
            // Input events
            .add_systems(
                First,
                (
                    // Hit plane follows camera
                    sys_move_bg_hit_plane,
                    // These systems take raw input events and pass them to the processor.
                    sys_mouse_button_input,
                    sys_mouse_movement_input, // This also updates the raycast ray
                    sys_keyboard_input,
                )
                    .before(RaycastSystem::BuildRays::<RaycastRawInput>),
            )
            .add_systems(Update, sys_raw_input_processor.in_set(EditorSet::PreMsgs));
    }
}

#[derive(Resource)]
pub struct RawInputResource {
    is_dragging: bool,
    left_pressed: bool,
    // right_pressed: bool,
    cur_pos: Vec2,
    down_pos: Vec2,
    down_pos_world: Vec2,
    modifiers: ModifiersState,

    double_click_timeout: f32, // TODO: Move to settings resource.
    last_click_time: f32,
}
impl Default for RawInputResource {
    fn default() -> Self {
        Self {
            is_dragging: false,
            left_pressed: false,
            cur_pos: Vec2::default(),
            down_pos: Vec2::default(),
            down_pos_world: Vec2::default(),
            modifiers: ModifiersState::default(),
            double_click_timeout: 0.3f32,
            last_click_time: 0.0f32,
        }
    }
}

/// Processes the Raw input events into more useful app events.
///
/// * `res`:
/// * `ev_reader`:
/// * `ev_writer`:
/// * `bg_hit_query`:
pub fn sys_raw_input_processor(
    time: Res<Time>,
    mut res: ResMut<RawInputResource>,
    mut ev_reader: EventReader<RawInputMessage>,
    mut ev_writer: EventWriter<InputMessage>,
    bg_hit_query: Query<&RaycastMesh<RaycastRawInput>, With<InputHitPlaneTag>>,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_raw_input_processor").entered();

    let mut world_point = Vec2::new(0., 0.);

    if let Ok(raycast_source) = bg_hit_query.get_single() {
        let intersections = raycast_source.intersections();

        if let Some((_, data)) = intersections.first() {
            world_point = data.position().xy();
        } else {
            warn!("Warn: Input system cannot get world position of mouse!.");
        }
    } else {
        warn!("Warn: Input system cannot get background entity.");
    }

    let mut to_send = Vec::<InputMessage>::with_capacity(8);

    for msg in ev_reader.iter() {
        match msg {
            RawInputMessage::PointerMove(move_model) => {
                res.cur_pos.x = move_model.x;
                res.cur_pos.y = move_model.y;

                if res.left_pressed
                    && !res.is_dragging
                    && res.cur_pos.distance(res.down_pos) > DRAG_THRESHOLD
                {
                    #[cfg(feature = "debug_trace")]
                    debug!(
                        "Sending DragStart: screen: {:?}, world: {:?}",
                        res.cur_pos, world_point
                    );

                    res.is_dragging = true;
                    to_send.push(InputMessage::DragStart {
                        screen: res.cur_pos,
                        screen_pressed: res.down_pos,
                        screen_offset: res.cur_pos.sub(res.down_pos),
                        world: world_point,
                        world_pressed: res.down_pos_world,
                        world_offset: world_point.sub(res.down_pos_world),
                        modifiers: res.modifiers,
                    })
                } else if res.is_dragging {
                    #[cfg(feature = "debug_trace")]
                    debug!(
                        "Sending DragMove: screen: {:?}, world: {:?}",
                        res.cur_pos, world_point
                    );

                    to_send.push(InputMessage::DragMove {
                        screen: res.cur_pos,
                        screen_pressed: res.down_pos,
                        screen_offset: res.cur_pos.sub(res.down_pos),
                        world: world_point,
                        world_pressed: res.down_pos_world,
                        world_offset: world_point.sub(res.down_pos_world),
                        modifiers: res.modifiers,
                    });
                } else {
                    #[cfg(feature = "debug_trace")]
                    debug!(
                        "Sending PointerMove: screen: {:?}, world: {:?}",
                        res.cur_pos, world_point
                    );

                    to_send.push(InputMessage::PointerMove {
                        screen: res.cur_pos,
                        world: world_point,
                        modifiers: res.modifiers,
                    });
                }
            }
            RawInputMessage::PointerInput { pressed, button } => match (button, pressed) {
                (MouseButton::Left, ButtonState::Pressed) => {
                    if !res.left_pressed {
                        res.left_pressed = true;
                        res.down_pos = res.cur_pos;
                        res.down_pos_world = world_point;

                        #[cfg(feature = "debug_trace")]
                        debug!(
                            "Sending PointerDown: screen: {:?}, world: {:?}",
                            res.cur_pos, world_point
                        );

                        to_send.push(InputMessage::PointerDown {
                            screen: res.cur_pos,
                            world: world_point,
                            modifiers: res.modifiers,
                        });
                    }
                }
                (MouseButton::Left, ButtonState::Released) => {
                    res.left_pressed = false;
                    if res.is_dragging {
                        res.is_dragging = false;

                        #[cfg(feature = "debug_trace")]
                        debug!(
                            "Sending DragEnd: screen: {:?}, world: {:?}",
                            res.cur_pos, world_point
                        );

                        to_send.push(InputMessage::DragEnd {
                            screen: res.cur_pos,
                            screen_pressed: res.down_pos,
                            screen_offset: res.cur_pos.sub(res.down_pos),
                            world: world_point,
                            world_pressed: res.down_pos_world,
                            world_offset: world_point.sub(res.down_pos_world),
                            modifiers: res.modifiers,
                        });
                    } else {
                        #[cfg(feature = "debug_trace")]
                        debug!(
                            "Sending PointerClick: screen: {:?}, world: {:?}",
                            res.cur_pos, world_point
                        );
                        let curr_time = time.elapsed_seconds();

                        if curr_time < res.last_click_time + res.double_click_timeout {
                            res.last_click_time = 0f32;

                            info!("Double clicking {curr_time} {} {}", res.last_click_time, res.double_click_timeout);
                            to_send.push(InputMessage::DoubleClick {
                                screen: res.cur_pos,
                                world: world_point,
                                modifiers: res.modifiers,
                            });

                        } else {
                            res.last_click_time = time.elapsed_seconds();

                            to_send.push(InputMessage::PointerClick {
                                screen: res.cur_pos,
                                world: world_point,
                                modifiers: res.modifiers,
                            });
                        }
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

                    #[cfg(feature = "debug_trace")]
                    debug!("Sending ModifiersChanged: modifiers: {:?}", res.modifiers);

                    to_send.push(InputMessage::ModifiersChanged {
                        state: res.modifiers,
                    });
                }
                KeyCode::AltLeft | KeyCode::AltRight => {
                    res.modifiers.alt = *pressed;

                    #[cfg(feature = "debug_trace")]
                    debug!("Sending ModifiersChanged: modifiers: {:?}", res.modifiers);

                    to_send.push(InputMessage::ModifiersChanged {
                        state: res.modifiers,
                    });
                }
                KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                    res.modifiers.shift = *pressed;

                    #[cfg(feature = "debug_trace")]
                    debug!("Sending ModifiersChanged: modifiers: {:?}", res.modifiers);

                    to_send.push(InputMessage::ModifiersChanged {
                        state: res.modifiers,
                    });
                }
                key => {
                    #[cfg(feature = "debug_trace")]
                    debug!("Sending Keyboard: key: {:?}, pressed: {:?}", key, pressed);

                    to_send.push(InputMessage::Keyboard {
                        pressed: *pressed,
                        key: *key,
                        modifiers: res.modifiers,
                    });
                }
            },
        }
    }

    let mut seen_variants = HashSet::new();
    let filtered: Vec<_> = to_send
        .into_iter()
        .rev()
        .filter(|variant| match variant {
            InputMessage::Keyboard { .. } => true,
            variant => seen_variants.insert(discriminant(variant)),
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    for ev in filtered {
        ev_writer.send(ev);
    }
}

fn sys_mouse_button_input(
    mut mousebtn_events: EventReader<MouseButtonInput>,
    mut message_writer: EventWriter<RawInputMessage>,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_mouse_button_input").entered();

    for ev in mousebtn_events.iter() {
        message_writer.send(RawInputMessage::PointerInput {
            pressed: ev.state,
            button: ev.button,
        })
    }
}

/// Passes raw input events into the processor + updates the raycast source with current mouse
/// position.
///
fn sys_mouse_movement_input(
    mut q_raycast_source: Query<&mut RaycastSource<RaycastRawInput>>,
    mut mousebtn_events: EventReader<CursorMoved>,
    mut message_writer: EventWriter<RawInputMessage>,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_mouse_movement_input").entered();

    let mut maybe_source = q_raycast_source.get_single_mut();
    for ev in mousebtn_events.iter() {
        if let Ok(ref mut source) = maybe_source {
            source.cast_method = RaycastMethod::Screenspace(ev.position)
        }
        message_writer.send(RawInputMessage::PointerMove(ev.position));
    }
}

fn sys_keyboard_input(
    mut key_evr: EventReader<KeyboardInput>,
    mut message_writer: EventWriter<RawInputMessage>,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_keyboard_input").entered();

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
pub struct InputHitPlaneTag;

/// Spawns a Raycaster hit plane for world coordinate mouse inputs + attaches the raycast
/// source to the camera.
///
fn sys_setup_input_plugin(mut commands: Commands, q_camera: Query<Entity, With<CameraTag>>) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_setup_input_plugin").entered();

    let shape = shapes::Rectangle {
        extents: Vec2::new(10000., 10000.),
        ..Default::default()
    };
    commands.spawn((
        Name::from("BgHitPlane"),
        InputHitPlaneTag,
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform {
                translation: Vec3::new(0., 0., BG_HIT_Z_INDEX),
                ..Default::default()
            },

            ..Default::default()
        },
        Fill::color(Color::rgb_u8(230, 230, 230)),
        RaycastMesh::<RaycastRawInput>::default(),
    ));

    let e_camera = q_camera
        .get_single()
        .expect("sys_setup_input_plugin: Cannot get camera.");
    let mut camera_commands = commands
        .get_entity(e_camera)
        .expect("sys_setup_input_plugin: Cannot get commands handle for camera.");
    camera_commands.insert(RaycastSource::<RaycastRawInput>::default());
}
fn sys_move_bg_hit_plane(
    cam: Query<&Transform, (With<Camera2d>, Without<InputHitPlaneTag>)>,
    mut bg_hit_plane: Query<&mut Transform, (With<InputHitPlaneTag>, Without<Camera2d>)>,
) {
    #[cfg(feature = "debug_trace")]
    let _span = info_span!("sys_move_bg_hit_plane").entered();

    if let (Ok(cam_transform), Ok(mut bg_hit_transform)) =
        (cam.get_single(), bg_hit_plane.get_single_mut())
    {
        bg_hit_transform.translation.x = cam_transform.translation.x;
        bg_hit_transform.translation.y = cam_transform.translation.y;
    }
}
