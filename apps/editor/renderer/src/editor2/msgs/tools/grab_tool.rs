use std::{collections::VecDeque, println};

use bevy::{ecs::system::SystemState, prelude::*, window::PrimaryWindow, math::Vec3Swizzles};

use crate::{
    debug_log,
    editor2::{
        camera::{CameraMessage, MyCameraTag},
        frontend::FrontendMessage,
        input::InputMessage, msgs::Message,
    },
    types::Cursors,
};

use super::ToolHandlerMessage;

pub fn handle_grab_tool_message(
    world: &mut World,
    message: &ToolHandlerMessage,
    responses: &mut VecDeque<Message>,
) {
    let mut grab_sys_state: SystemState<(
        // Current Camera
        Query<(&MyCameraTag, &Transform, &OrthographicProjection)>,
        // Camera event writer
        EventWriter<CameraMessage>,
    )> = SystemState::new(world);

    let (q_camera, mut camera_writer) = grab_sys_state.get_mut(world);

    match message {
        ToolHandlerMessage::OnActivate => {
            debug_log!("GrabTool::OnActivate");
            responses.push_back(FrontendMessage::SetCursor(Cursors::Grab).into());
        }
        ToolHandlerMessage::OnDeactivate => {
            debug_log!("GrabTool::OnDeactivate");
        }
        ToolHandlerMessage::Input(input_message) => {
            match input_message {
                InputMessage::DragStart { .. } => {
                    responses.push_back(FrontendMessage::SetCursor(Cursors::Grabbing).into());
                }
                InputMessage::DragMove {
                    world_offset,
                    ..
                } => {
                    for (cam, transform, projection) in &q_camera {
                        let proj_size = projection.area.size();

                        // The proposed new camera position
                        let delta_world = world_offset.xy();
                        let mut proposed_cam_transform =
                            transform.translation - delta_world.extend(0.);

                        // Check whether the proposed camera movement would be within the provided boundaries, override it if we
                        // need to do so to stay within bounds.
                        if let Some(min_x_boundary) = cam.min_x {
                            let min_safe_cam_x = min_x_boundary + proj_size.x / 2.;
                            proposed_cam_transform.x = proposed_cam_transform.x.max(min_safe_cam_x);
                        }
                        if let Some(max_x_boundary) = cam.max_x {
                            let max_safe_cam_x = max_x_boundary - proj_size.x / 2.;
                            proposed_cam_transform.x = proposed_cam_transform.x.min(max_safe_cam_x);
                        }
                        if let Some(min_y_boundary) = cam.min_y {
                            let min_safe_cam_y = min_y_boundary + proj_size.y / 2.;
                            proposed_cam_transform.y = proposed_cam_transform.y.max(min_safe_cam_y);
                        }
                        if let Some(max_y_boundary) = cam.max_y {
                            let max_safe_cam_y = max_y_boundary - proj_size.y / 2.;
                            proposed_cam_transform.y = proposed_cam_transform.y.min(max_safe_cam_y);
                        }

                        camera_writer.send(CameraMessage::SetTranslate {
                            pos: proposed_cam_transform,
                        })
                    }
                }
                InputMessage::DragEnd {
                    screen_offset: offset,
                    ..
                } => {
                    // if let Some(_) = data {
                    //     responses.push_back(Message::Document(DocumentMessage::Camera(
                    //         DocumentCameraMessage::SetTranslate(
                    //             self.initial_translate + offset,
                    //         ),
                    //     )))
                    // }
                    responses.push_back(FrontendMessage::SetCursor(Cursors::Grab).into());
                }
                _ => {}
            }
        }
    }
}
