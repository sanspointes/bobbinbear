mod types;

use std::{mem::discriminant, ops::Sub};

use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput, ButtonState},
    math::Vec3Swizzles,
    prelude::*,
    sprite::MaterialMesh2dBundle,
    utils::HashSet,
    window::PrimaryWindow,
};
use bevy_mod_raycast::prelude::{
    DeferredRaycastingPlugin, RaycastMesh, RaycastMethod, RaycastSource, RaycastSystem,
};

use crate::plugins::viewport::{sys_setup_viewport, BobbinViewport};

pub use self::types::{InputMessage, ModifiersState, RawInputMessage};

#[derive(Debug, Clone, Reflect)]
pub struct RaycastRawInput;

const DRAG_THRESHOLD: f32 = 2.;
const BG_HIT_Z_INDEX: f32 = -100.;

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub enum InputSet {
    ConvertInputMessages,
    HandleInputMessages,
}

/// The input processor plugin processes raw input (mouse down/up, move, etc)
/// into more useful events like Click, DragStart, move, etc.
///
/// These events can be listened to Via the EventReceiver<InputMessage> type.
pub struct BobbinInputPlugin;
impl Plugin for BobbinInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RawInputResource::default())
            .add_event::<RawInputMessage>()
            .add_event::<InputMessage>()
            .add_plugins(DeferredRaycastingPlugin::<RaycastRawInput>::default())
            // Hit plane creation and movement
            .add_systems(Startup, sys_setup_input_plugin.after(sys_setup_viewport))
            // Input events
            .configure_sets(
                First,
                InputSet::ConvertInputMessages.before(InputSet::HandleInputMessages),
            )
            .configure_sets(
                First,
                InputSet::HandleInputMessages.before(RaycastSystem::BuildRays::<RaycastRawInput>),
            )
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
                    .in_set(InputSet::ConvertInputMessages),
            )
            .add_systems(
                First,
                sys_raw_input_processor.in_set(InputSet::HandleInputMessages),
            );
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
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
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
    let window_size = {
        let w = q_window.single();
        Vec2::new(w.width(), w.height())
    };

    for msg in ev_reader.read() {
        match msg {
            RawInputMessage::PointerMove(pos) => {
                res.cur_pos.x = pos.x;
                res.cur_pos.y = window_size.y - pos.y;

                if res.left_pressed
                    && !res.is_dragging
                    && res.cur_pos.distance(res.down_pos) > DRAG_THRESHOLD
                {
                    res.is_dragging = true;
                    to_send.push(InputMessage::DragStart {
                        screen_pos: res.cur_pos,
                        screen_start_pos: res.down_pos,
                        screen_delta_pos: res.cur_pos.sub(res.down_pos),
                        world_pos: world_point,
                        world_start_pos: res.down_pos_world,
                        world_delta_pos: world_point.sub(res.down_pos_world),
                        modifiers: res.modifiers,
                    })
                } else if res.is_dragging {
                    to_send.push(InputMessage::DragMove {
                        screen_pos: res.cur_pos,
                        screen_start_pos: res.down_pos,
                        screen_delta_pos: res.cur_pos.sub(res.down_pos),
                        world_pos: world_point,
                        world_start_pos: res.down_pos_world,
                        world_delta_pos: world_point.sub(res.down_pos_world),
                        modifiers: res.modifiers,
                    });
                } else {
                    to_send.push(InputMessage::PointerMove {
                        screen_pos: res.cur_pos,
                        world_pos: world_point,
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
                        to_send.push(InputMessage::PointerDown {
                            screen_pos: res.cur_pos,
                            world_pos: world_point,
                            modifiers: res.modifiers,
                        });
                    }
                }
                (MouseButton::Left, ButtonState::Released) => {
                    res.left_pressed = false;
                    if res.is_dragging {
                        res.is_dragging = false;
                        to_send.push(InputMessage::DragEnd {
                            screen_pos: res.cur_pos,
                            screen_start_pos: res.down_pos,
                            screen_delta_pos: res.cur_pos.sub(res.down_pos),
                            world_pos: world_point,
                            world_start_pos: res.down_pos_world,
                            world_delta_pos: world_point.sub(res.down_pos_world),
                            modifiers: res.modifiers,
                        });
                    } else {
                        let curr_time = time.elapsed_seconds();

                        if curr_time < res.last_click_time + res.double_click_timeout {
                            res.last_click_time = 0f32;

                            info!(
                                "Double clicking {curr_time} {} {}",
                                res.last_click_time, res.double_click_timeout
                            );
                            to_send.push(InputMessage::DoubleClick {
                                screen_pos: res.cur_pos,
                                world_pos: world_point,
                                modifiers: res.modifiers,
                            });
                        } else {
                            res.last_click_time = time.elapsed_seconds();

                            to_send.push(InputMessage::PointerClick {
                                screen_pos: res.cur_pos,
                                world_pos: world_point,
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
                    to_send.push(InputMessage::ModifiersChanged {
                        state: res.modifiers,
                    });
                }
                KeyCode::AltLeft | KeyCode::AltRight => {
                    res.modifiers.alt = *pressed;
                    to_send.push(InputMessage::ModifiersChanged {
                        state: res.modifiers,
                    });
                }
                KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                    res.modifiers.shift = *pressed;
                    to_send.push(InputMessage::ModifiersChanged {
                        state: res.modifiers,
                    });
                }
                key => {
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
    for ev in mousebtn_events.read() {
        message_writer.send(RawInputMessage::PointerInput {
            pressed: ev.state,
            button: ev.button,
        });
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
    let mut maybe_source = q_raycast_source.get_single_mut();
    for ev in mousebtn_events.read() {
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
    for ev in key_evr.read() {
        message_writer.send(RawInputMessage::KeyboardInput {
            pressed: ev.state,
            key: ev.key_code,
        });
    }
}

// BG Hit plane, responsible for proving the background color +
// mapping mouse events to world coordinates

#[derive(Component)]
pub struct InputHitPlaneTag;

/// Spawns a Raycaster hit plane for world coordinate mouse inputs + attaches the raycast
/// source to the camera.
///
fn sys_setup_input_plugin(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_camera: Query<Entity, With<BobbinViewport>>,
) {
    let handle = meshes.add(Mesh::from(Rectangle::new(10000., 10000.)));
    commands.spawn((
        Name::from("BgHitPlane"),
        InputHitPlaneTag,
        MaterialMesh2dBundle {
            mesh: handle.into(),
            transform: Transform {
                translation: Vec3::new(0., 0., BG_HIT_Z_INDEX),
                ..Default::default()
            },
            material: materials.add(ColorMaterial::from(Color::rgb(0.8, 0.8, 0.8))),
            ..Default::default()
        },
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
    cam: Query<&Transform, (With<BobbinViewport>, Without<InputHitPlaneTag>)>,
    mut bg_hit_plane: Query<&mut Transform, (With<InputHitPlaneTag>, Without<Camera2d>)>,
) {
    if let (Ok(cam_transform), Ok(mut bg_hit_transform)) =
        (cam.get_single(), bg_hit_plane.get_single_mut())
    {
        bg_hit_transform.translation.x = cam_transform.translation.x;
        bg_hit_transform.translation.y = cam_transform.translation.y;
    }
}
