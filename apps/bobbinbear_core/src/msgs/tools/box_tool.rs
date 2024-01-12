use std::ops::{Add, Sub};

use bevy::{ecs::system::SystemState, prelude::*, math::vec2};

use crate::{
    components::{bbid::BBId, scene::BBObject},
    debug_log,
    msgs::{
        api::ApiEffectMsg,
        cmds::{add_remove_object_cmd::AddObjectCmd, update_vector_graph_cmd::UpdateVectorGraphCmd, CmdMsg},
        MsgQue,
    },
    plugins::{
        bounds_2d_plugin::GlobalBounds2D, input_plugin::InputMessage,
        selection_plugin::SelectableBundle, vector_graph_plugin::Fill,
    },
    types::BBCursor,
    utils::{vector::BBObjectVectorBundle, vector_graph::build_vector_graph_box},
};

use super::{ToolFsmError, ToolFsmResult, ToolHandlerMessage};

//
// BOX TOOL FSM
//

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum BoxFsm {
    #[default]
    Default,
    PointerDown {
        cursor_origin_pos: Vec2,
    },
    BuildingBox {
        bbid: BBId,
        cursor_origin_pos: Vec2,
        box_origin_pos: Vec2,
        box_extents: Vec2,
    },
}

impl BoxFsm {
    fn handle_pointer_down(mut self, cursor_position: &Vec2) -> ToolFsmResult<BoxFsm> {
        let old = self;
        match self {
            BoxFsm::Default => {
                self = BoxFsm::PointerDown {
                    cursor_origin_pos: *cursor_position,
                };
                Ok(self)
            }
            _ => Err(ToolFsmError::NoTransition),
        }
        .map(|new| (old, new))
    }

    /// Occurs after pointer down, resetting state to default
    fn handle_pointer_click(mut self) -> ToolFsmResult<BoxFsm> {
        let old = self;
        match self {
            BoxFsm::PointerDown { .. } => {
                self = BoxFsm::Default;
                Ok(self)
            }
            _ => Err(ToolFsmError::NoTransition),
        }
        .map(|new| (old, new))
    }
    /// On drag start we start building a box.
    ///
    /// * `world`:
    /// * `world_pressed_pos`:
    /// * `world_drag_offset`:
    fn handle_drag_start(mut self, cursor_offset: &Vec2) -> ToolFsmResult<BoxFsm> {
        let old = self;
        match self {
            BoxFsm::PointerDown { cursor_origin_pos } => {
                let bbid = BBId::default();
                let box_origin_pos =
                    Vec2::min(cursor_origin_pos, cursor_origin_pos.sub(*cursor_offset));
                let box_extents = Vec2::abs(*cursor_offset);

                self = BoxFsm::BuildingBox {
                    bbid,
                    cursor_origin_pos,
                    box_origin_pos,
                    box_extents,
                };

                Ok(self)
            }
            _ => Err(ToolFsmError::NoTransition),
        }
        .map(|new| (old, new))
    }

    fn handle_drag_move(mut self, cursor_offset: &Vec2) -> ToolFsmResult<BoxFsm> {
        let old = self;
        match self {
            BoxFsm::BuildingBox {
                cursor_origin_pos,
                ref mut box_origin_pos,
                ref mut box_extents,
                ..
            } => {
                *box_extents = Vec2::abs(*cursor_offset);
                *box_origin_pos =
                    Vec2::min(cursor_origin_pos, cursor_origin_pos.add(*cursor_offset));
                Ok(self)
            }
            _ => Err(ToolFsmError::NoTransition),
        }
        .map(|new| (old, new))
    }

    fn handle_drag_end(mut self) -> ToolFsmResult<BoxFsm> {
        let old = self;
        match self {
            BoxFsm::BuildingBox { .. } => {
                self = BoxFsm::Default;
                Ok(self)
            }
            _ => Err(ToolFsmError::NoTransition),
        }
        .map(|new| (old, new))
    }
}

#[derive(Resource, Default)]
pub struct BoxToolRes {
    state: BoxFsm,
}

//
// BOX TOOL MESSAGE HANDLERS
//

pub fn msg_handler_box_tool(
    world: &mut World,
    message: &ToolHandlerMessage,
    responder: &mut MsgQue,
) {
    let _span = debug_span!("msg_handler_box_tool").entered();

    match message {
        ToolHandlerMessage::OnActivate => {
            debug_log!("BoxTool::OnActivate");
            responder.notify_effect(ApiEffectMsg::SetCursor(BBCursor::Box));
        }
        ToolHandlerMessage::OnDeactivate => {
            debug_log!("BoxTool::OnDeactivate");
        }
        ToolHandlerMessage::Input(input_message) => {
            msg_handler_box_tool_input(world, input_message, responder)
        }
    }
}

pub fn msg_handler_box_tool_input(
    world: &mut World,
    message: &InputMessage,
    responder: &mut MsgQue,
) {
    let mut sys_state: SystemState<(ResMut<BoxToolRes>,)> = SystemState::new(world);

    let mut res = sys_state.get_mut(world).0;

    let result = match message {
        InputMessage::PointerDown {
            world: world_pos, ..
        } => res.state.handle_pointer_down(world_pos),
        // On Click we try make a default box
        InputMessage::PointerClick { .. } => res.state.handle_pointer_click(),
        // On Drag start we try to create a box that we will continue to update.
        InputMessage::DragStart { world_offset, .. } => res.state.handle_drag_start(world_offset),
        InputMessage::DragMove { world_offset, .. } => res.state.handle_drag_move(world_offset),
        InputMessage::DragEnd { .. } => res.state.handle_drag_end(),
        _ => Err(ToolFsmError::NoTransition),
    };

    // Save the new state back in the resource.
    if let Ok((_, new_state)) = result {
        res.state = new_state;
    }

    // Handle state transitions
    match result {
        //
        // Default -> Pointer down is preparing to either make box manually or via a click event
        Ok((BoxFsm::Default, BoxFsm::PointerDown { .. })) => {}
        //
        // PointerDown -> Default, create a default box by click event.
        Ok((BoxFsm::PointerDown { cursor_origin_pos }, BoxFsm::Default)) => {
            let bbid = BBId::default();
            let cmd_result = AddObjectCmd::from_builder(world, None, |entity| {
                let vector_graph = build_vector_graph_box(vec2(0., 0.), vec2(10., 10.));

                entity.insert((
                    Name::from("Box"),
                    bbid,
                    BBObject::Vector,
                    BBObjectVectorBundle::from_vector_graph(vector_graph).with_transform(Transform {
                        translation: Vec3::new(cursor_origin_pos.x, cursor_origin_pos.y, 0.),
                        ..Default::default()
                    }),
                    GlobalBounds2D::default(),
                    SelectableBundle::default(),
                    Fill::color(Color::rgb_u8(50, 50, 50)),
                ));
            });

            match cmd_result {
                Ok(cmd) => responder.push_internal(CmdMsg::from(cmd)),
                Err(reason) => {
                    error!("Error performing .start_making_box on box_tool \"{reason:?}\".")
                }
            }
        }
        //
        // PointerDown -> BuildingBox, Creates a new box with no path
        Ok((
            BoxFsm::PointerDown { .. },
            BoxFsm::BuildingBox {
                bbid,
                box_origin_pos,
                box_extents,
                ..
            },
        )) => {
            let cmd_result = AddObjectCmd::from_builder(world, None, |entity| {
                let vector_graph = build_vector_graph_box(vec2(0., 0.), box_extents);

                entity.insert((
                    Name::from("Box"),
                    bbid,
                    BBObject::Vector,
                    BBObjectVectorBundle::from_vector_graph(vector_graph).with_transform(Transform {
                        translation: Vec3::new(box_origin_pos.x, box_origin_pos.y, 0.),
                        ..Default::default()
                    }),
                    GlobalBounds2D::default(),
                    SelectableBundle::default(),
                    Fill::color(Color::rgb_u8(50, 50, 50)),
                ));
            });

            match cmd_result {
                Ok(cmd) => responder.push_internal(CmdMsg::from(cmd)),
                Err(reason) => {
                    error!("Error performing .start_making_box on box_tool \"{reason:?}\".")
                }
            }
        }
        //
        // BuildingBox -> BuildingBox, Updates the currently building box
        Ok((
            BoxFsm::BuildingBox { .. },
            BoxFsm::BuildingBox {
                bbid, box_extents, ..
            },
        )) => {
            let vector_graph = build_vector_graph_box(vec2(0., 0.), box_extents);
            let cmd = UpdateVectorGraphCmd::new(bbid, vector_graph);

            responder.push_internal(CmdMsg::from(cmd));
        }
        //
        // BuildingBox -> Default, Finish building the box.
        Ok((
            BoxFsm::BuildingBox {
                bbid, box_extents, ..
            },
            BoxFsm::Default,
        )) => {
            let vector_graph = build_vector_graph_box(vec2(0., 0.), box_extents);
            let cmd = UpdateVectorGraphCmd::new(bbid, vector_graph);

            responder.push_internal(CmdMsg::from(cmd));
        }
        Ok((arg1, arg2)) => panic!("BoxTool: Unhandled state transition from {arg1:?} to {arg2:?}"),
        Err(ToolFsmError::NoTransition) => {}
        Err(ToolFsmError::TransitionError(error)) => {
            panic!("BoxTool: Error during transition. Reason {error:?}.")
        }
    }
}

//
// BOX TOOL HELPERS
//
