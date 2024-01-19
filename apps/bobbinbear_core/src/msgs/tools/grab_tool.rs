use std::ops::{Add, Mul, Sub};

use anyhow::anyhow;
use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    msgs::{api::ApiEffectMsg, MsgQue, effect::EffectMsg},
    plugins::{input_plugin::InputMessage, screen_space_root_plugin::ScreenSpaceRoot},
    systems::camera::CameraTag,
    types::BBCursor,
    utils::coordinates,
};

use super::{ToolHandlerMessage, ToolHandler};

#[derive(Resource, Clone)]
pub enum GrabToolState {
    None {
        // Stores the current translation of the camera
        translation: Vec2,
    },
    Moving {
        // Stores the current translation of the camera
        translation: Vec2,
        // Stores the initial translation position when moving started
        initial_translation: Vec2,
        // Stores the initial position of the mouse in the world.
        initial_mouse_pos: Vec2,
    },
}
impl Default for GrabToolState {
    fn default() -> Self {
        Self::None {
            translation: Vec2::ZERO,
        }
    }
}

const VEC2_INVERSE_Y: Vec2 = Vec2::new(1., -1.);

impl GrabToolState {
    /// Returns the drag end or reset of this [`GrabToolState`].
    fn drag_end_or_reset(&self) -> Self {
        match self {
            Self::None { translation } => Self::None {
                translation: *translation,
            },
            Self::Moving { translation, .. } => Self::None {
                translation: *translation,
            },
        }
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn drag_start(&self, initial_mouse_pos: &Vec2) -> Result<Self, anyhow::Error> {
        match self {
            Self::None { translation } => Ok(Self::Moving {
                translation: *translation,
                initial_translation: *translation,
                initial_mouse_pos: *initial_mouse_pos,
            }),
            _ => Err(anyhow!("Invalid state transition")),
        }
    }

    fn drag_move(&self, current_mouse_pos: &Vec2) -> Result<Self, anyhow::Error> {
        match self {
            Self::None { .. } => Err(anyhow!("Invalid state transition")),
            Self::Moving {
                initial_mouse_pos,
                initial_translation,
                ..
            } => Ok(Self::Moving {
                initial_translation: *initial_translation,
                translation: initial_translation.add(
                    initial_mouse_pos
                        .sub(*current_mouse_pos)
                        .mul(VEC2_INVERSE_Y),
                ),
                initial_mouse_pos: *initial_mouse_pos,
            }),
        }
    }
}

pub struct GrabTool;

impl ToolHandler for GrabTool {
    fn setup(world: &mut World) {
        
    }
    fn handle_msg(world: &mut World, msg: &ToolHandlerMessage, responder: &mut MsgQue) {
        let _span = debug_span!("msg_handler_select_tool").entered();

        let mut grab_sys_state: SystemState<(
            ResMut<GrabToolState>,
            // Current Camera
            Query<&mut Transform, With<CameraTag>>,
            // Primary window query
            Query<&ScreenSpaceRoot>,
        )> = SystemState::new(world);

        let (mut grab_state, mut q_camera, q_ss_root) = grab_sys_state.get_mut(world);

        let mut effects = vec![];

        match msg {
            ToolHandlerMessage::OnActivate => {
                debug!("GrabTool::OnActivate");
                responder.notify_effect(ApiEffectMsg::SetCursor(BBCursor::Grab));
            }
            ToolHandlerMessage::OnDeactivate => {
                debug!("GrabTool::OnDeactivate");
            }
            ToolHandlerMessage::Input(input_message) => {
                let mut transform = q_camera.single_mut();
                let ss_root = q_ss_root.single();

                match input_message {
                    InputMessage::DragStart { screen_pressed, .. } => {
                        let world_pos_2d = coordinates::screen_to_world(
                            *screen_pressed,
                            ss_root.window_size(),
                            ss_root.projection_area(),
                        );
                        let v = grab_state.drag_start(&world_pos_2d);

                        match &v {
                            Ok(new_state) => {
                                if let GrabToolState::Moving { translation, .. } = new_state {
                                    responder
                                        .notify_effect(ApiEffectMsg::SetCursor(BBCursor::Grabbing));
                                    let delta = translation.xy() - transform.translation.xy();
                                    transform.translation.x = translation.x;
                                    transform.translation.y = translation.y;

                                    effects.push(EffectMsg::CameraMoved {
                                        translation: transform.translation.xy(),
                                        delta,
                                    });
                                }
                                *grab_state = new_state.clone();
                            }
                            Err(_) => {}
                        }
                    }
                    InputMessage::DragMove { screen, .. } => {
                        let world_pos_2d = coordinates::screen_to_world(
                            *screen,
                            ss_root.window_size(),
                            ss_root.projection_area(),
                        );
                        let v = grab_state.drag_move(&world_pos_2d);

                        match &v {
                            Ok(new_state) => {
                                if let GrabToolState::Moving { translation, .. } = new_state {
                                    let delta = translation.xy() - transform.translation.xy();
                                    transform.translation.x = translation.x;
                                    transform.translation.y = translation.y;

                                    effects.push(EffectMsg::CameraMoved {
                                        translation: transform.translation.xy(),
                                        delta,
                                    });
                                }
                                *grab_state = new_state.clone();
                            }
                            Err(_) => {}
                        }
                    }
                    InputMessage::DragEnd { screen, .. } => {
                        let world_pos_2d = coordinates::screen_to_world(
                            *screen,
                            ss_root.window_size(),
                            ss_root.projection_area(),
                        );
                        let v = grab_state.drag_move(&world_pos_2d);

                        match &v {
                            Ok(new_state) => {
                                if let GrabToolState::Moving { translation, .. } = new_state {
                                    let delta = translation.xy() - transform.translation.xy();
                                    transform.translation.x = translation.x;
                                    transform.translation.y = translation.y;

                                    effects.push(EffectMsg::CameraMoved {
                                        translation: transform.translation.xy(),
                                        delta,
                                    });
                                }
                                *grab_state = grab_state.drag_end_or_reset();
                            }
                            Err(_) => {}
                        }
                        responder.notify_effect(ApiEffectMsg::SetCursor(BBCursor::Grab));
                    }
                    _ => {}
                }

                for ev in effects {
                    ev.send(world);
                }
            }
        }
    }
    fn handle_effects(world: &mut World, event: &EffectMsg) {
        
    }
}
