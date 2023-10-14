use std::{collections::VecDeque, ops::{Add, Sub}};

use bevy::{ecs::system::SystemState, prelude::*, utils::thiserror::Error};
use bevy_prototype_lyon::{
    prelude::{GeometryBuilder, ShapeBundle, Fill},
    shapes,
};

use crate::{
    components::{bbid::BBId, scene::BBObject},
    debug_log,
    msgs::{
        cmds::{
            add_remove_object_cmd::AddObjectCmd, update_path_cmd::UpdatePathCmd,
            CmdMsg,
        },
        frontend::FrontendMsg,
        Message,
    },
    plugins::input_plugin::InputMessage,
    types::BBCursor,
};

use super::ToolHandlerMessage;

//
// BOX TOOL FSM
//

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum ToolState {
    #[default]
    None,
    BuildingBox {
        bbid: BBId,
        cursor_origin_pos: Vec2,
        box_origin_pos: Vec2,
        box_extents: Vec2,
    },
}

#[derive(Error, Debug)]
enum ToolStateError {
    #[error("No valid transition.")]
    NoTransition,
}

impl ToolState {
    fn make_default_box(&mut self) -> Result<(ToolState, ToolState), ToolStateError> {
        match self {
            ToolState::None => {
                let old = self.clone();
                *self = ToolState::None;
                Ok((old, *self))
            }
            _ => Err(ToolStateError::NoTransition),
        }
    }
    /// On drag start we start building a box.
    ///
    /// * `world`:
    /// * `world_pressed_pos`:
    /// * `world_drag_offset`:
    fn start_making_box(
        &mut self,
        cursor_origin_pos: &Vec2,
        cursor_offset: &Vec2,
    ) -> Result<(ToolState, ToolState), ToolStateError> {
        match *self {
            ToolState::None => {
                let bbid = BBId::default();
                let box_origin_pos =
                    Vec2::min(*cursor_origin_pos, cursor_origin_pos.sub(*cursor_offset));
                let box_extents = Vec2::abs(*cursor_offset);

                let old = self.clone();
                *self = ToolState::BuildingBox {
                    bbid,
                    cursor_origin_pos: *cursor_origin_pos,
                    box_origin_pos,
                    box_extents,
                };

                Ok((old, *self))
            }
            _ => Err(ToolStateError::NoTransition),
        }
    }

    fn update_current_box(
        &mut self,
        cursor_offset: &Vec2,
    ) -> Result<(ToolState, ToolState), ToolStateError> {
        match *self {
            ToolState::BuildingBox {
                bbid,
                cursor_origin_pos,
                box_origin_pos,
                box_extents,
            } => {
                let box_origin_pos =
                    Vec2::min(cursor_origin_pos, cursor_origin_pos.add(*cursor_offset));
                let box_extents = Vec2::abs(*cursor_offset);

                let old = self.clone();
                *self = ToolState::BuildingBox {
                    bbid,
                    cursor_origin_pos,
                    box_origin_pos,
                    box_extents,
                };

                Ok((old, *self))
            }
            _ => Err(ToolStateError::NoTransition),
        }
    }

    fn complete_current_box(&mut self) -> Result<(ToolState, ToolState), ToolStateError> {
        match *self {
            ToolState::BuildingBox { .. } => {
                let old = self.clone();
                *self = ToolState::None;
                Ok((old, *self))
            }
            _ => Err(ToolStateError::NoTransition),
        }
    }
}

#[derive(Resource, Default)]
pub struct BoxToolResource {
    state: ToolState,
}

//
// BOX TOOL MESSAGE HANDLERS
//

pub fn msg_handler_box_tool(
    world: &mut World,
    message: &ToolHandlerMessage,
    responses: &mut VecDeque<Message>,
) {
    match message {
        ToolHandlerMessage::OnActivate => {
            debug_log!("BoxTool::OnActivate");
            responses.push_back(FrontendMsg::SetCursor(BBCursor::Box).into());
        }
        ToolHandlerMessage::OnDeactivate => {
            debug_log!("BoxTool::OnDeactivate");
        }
        ToolHandlerMessage::Input(input_message) => {
            msg_handler_box_tool_input(world, input_message, responses)
        }
    }
}

pub fn msg_handler_box_tool_input(
    world: &mut World,
    message: &InputMessage,
    responses: &mut VecDeque<Message>,
) {
    let mut sys_state: SystemState<(ResMut<BoxToolResource>,)> = SystemState::new(world);

    let mut res = sys_state.get_mut(world).0;

    match message {
        InputMessage::PointerMove { screen, world, .. } => {
            dbg!(world);
            dbg!(screen);
        }
        // On Click we try make a default box
        InputMessage::PointerClick {
            world: world_pressed,
            ..
        } => {
            let result = res.state.make_default_box();

            match result {
                Ok((ToolState::None, ToolState::None)) => {
                    let bbid = BBId::default();
                    let cmd_result = AddObjectCmd::from_builder(world, None, |entity| {
                        let shape = shapes::Rectangle {
                            origin: shapes::RectangleOrigin::TopLeft,
                            extents: Vec2::new(100., 100.), // TODO: Convert this to a setting
                        };
                        entity.insert((
                            Name::from("Box"),
                            bbid,
                            BBObject::Vector,
                            ShapeBundle {
                                path: GeometryBuilder::build_as(&shape),
                                transform: Transform {
                                    translation: Vec3::new(world_pressed.x, world_pressed.y, 0.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            Fill::color(Color::rgb_u8(50, 50, 50))
                        ));
                    });

                    match cmd_result {
                        Ok(cmd) => responses.push_back(CmdMsg::from(cmd).into()),
                        Err(reason) => error!(
                            "Error performing .start_making_box on box_tool \"{reason:?}\"."
                        ),
                    }
                }
                Ok((_, _)) => panic!("Unhandled state transition"),
                Err(ToolStateError::NoTransition) => {}
            }
        }
        // On Drag start we try to create a box that we will continue to update.
        InputMessage::DragStart {
            // screen,
            // modifiers,
            world_pressed,
            world_offset,
            ..
        } => {
            let result = res.state.start_making_box(world_pressed, world_offset);

            match result {
                Ok((
                    ToolState::None,
                    ToolState::BuildingBox {
                        bbid,
                        box_origin_pos,
                        box_extents,
                        ..
                    },
                )) => {
                    let cmd_result = AddObjectCmd::from_builder(world, None, |entity| {
                        let shape = shapes::Rectangle {
                            origin: shapes::RectangleOrigin::TopLeft,
                            extents: box_extents,
                        };
                        entity.insert((
                            Name::from("Box"),
                            bbid,
                            BBObject::Vector,
                            ShapeBundle {
                                path: GeometryBuilder::build_as(&shape),
                                transform: Transform {
                                    translation: Vec3::new(box_origin_pos.x, box_origin_pos.y, 0.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            Fill::color(Color::rgb_u8(50, 50, 50))
                        ));
                    });

                    match cmd_result {
                        Ok(cmd) => responses.push_back(CmdMsg::from(cmd).into()),
                        Err(reason) => error!(
                            "Error performing .start_making_box on box_tool \"{reason:?}\"."
                        ),
                    }
                }
                Ok((_, _)) => panic!("Unhandled state transition"),
                Err(ToolStateError::NoTransition) => {}
            }
        }

        InputMessage::DragMove { world_offset, .. } => {
            let result = res.state.update_current_box(world_offset);

            match result {
                Ok((
                    ToolState::BuildingBox { .. },
                    ToolState::BuildingBox {
                        bbid,
                        box_extents,
                        ..
                    },
                )) => {
                    let shape = shapes::Rectangle {
                        origin: shapes::RectangleOrigin::TopLeft,
                        extents: box_extents,
                    };
                    let path = GeometryBuilder::build_as(&shape).0;

                    let cmd = UpdatePathCmd::new(bbid, path);

                    responses.push_back(CmdMsg::from(cmd).into());
                }
                Ok((_, _)) => panic!("Unhandled state transition"),
                Err(ToolStateError::NoTransition) => {}
            }
        }

        InputMessage::DragEnd { .. } => {
            let result = res.state.complete_current_box();

            match result {
                Ok((
                    ToolState::BuildingBox {
                        bbid,
                        box_extents,
                        box_origin_pos,
                        ..
                    },
                    ToolState::None,
                )) => {
                    let shape = shapes::Rectangle {
                        origin: shapes::RectangleOrigin::TopLeft,
                        extents: box_extents,
                    };
                    let path = GeometryBuilder::build_as(&shape).0;

                    let cmd = UpdatePathCmd::new(bbid, path);

                    responses.push_back(CmdMsg::from(cmd).into());
                }
                Ok((_, _)) => panic!("Unhandled state transition"),
                Err(ToolStateError::NoTransition) => {}
            }
        }
        _ => {}
    };
}

//
// BOX TOOL HELPERS
//
