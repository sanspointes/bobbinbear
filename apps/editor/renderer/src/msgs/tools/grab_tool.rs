use std::{collections::VecDeque, println};

use bevy::{ecs::system::SystemState, math::Vec3Swizzles, prelude::*};

use crate::{
    msgs::{frontend::FrontendMsg, Message},
    plugins::input_plugin::InputMessage,
    systems::camera::CameraTag,
    types::BBCursor,
};

use super::ToolHandlerMessage;

pub fn msg_handler_grab_tool(
    world: &mut World,
    message: &ToolHandlerMessage,
    responses: &mut VecDeque<Message>,
) {
    let mut grab_sys_state: SystemState<(
        // Current Camera
        Query<(&CameraTag, &mut Transform, &OrthographicProjection)>,
    )> = SystemState::new(world);

    let mut q_camera = grab_sys_state.get_mut(world).0;

    match message {
        ToolHandlerMessage::OnActivate => {
            println!("GrabTool::OnActivate");
            responses.push_back(FrontendMsg::SetCursor(BBCursor::Grab).into());
        }
        ToolHandlerMessage::OnDeactivate => {
            println!("GrabTool::OnDeactivate");
        }
        ToolHandlerMessage::Input(input_message) => {
            match input_message {
                InputMessage::DragStart { .. } => {
                    responses.push_back(FrontendMsg::SetCursor(BBCursor::Grabbing).into());
                }
                InputMessage::DragMove { world_offset, .. } => {
                    let (cam, mut transform, projection) = q_camera.single_mut();
                    let proj_size = projection.area.size();

                    // The proposed new camera position
                    let delta_world = world_offset.xy();
                    let mut proposed_cam_transform = transform.translation - delta_world.extend(0.);

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

                    transform.translation = proposed_cam_transform;
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
                    responses.push_back(FrontendMsg::SetCursor(BBCursor::Grab).into());
                }
                _ => {}
            }
        }
    }
}
