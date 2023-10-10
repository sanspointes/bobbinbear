use std::{collections::VecDeque, matches, sync::Arc};

use bevy::{input::ButtonState, math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::{prelude::*, shapes};

use crate::{
    debug_log,
    editor2::{
        entities::{vector::{VectorObjectSpawner, VectorObjectTag}, ActiveDocumentTag},
        frontend::FrontendMessage,
        input::InputMessage,
        msgs::{
            document::commands::{element_commands::ElementOperation, EmbCommand},
            DocMessage,
        },
        Message,
    },
    types::Cursors,
};

use super::ToolHandlerMessage;

pub fn build_box_path(
    shift_state: ButtonState,
    alt_state: ButtonState,
    world_offset: &Vec3,
) -> tess::path::Path {
    let mut extents = match shift_state {
        ButtonState::Released => world_offset.xy(),
        ButtonState::Pressed => {
            let max = world_offset.xy().max_element();
            Vec2::new(max, max)
        }
    };
    let origin = match alt_state {
        ButtonState::Pressed => {
            extents = extents * 2.;
            RectangleOrigin::Center
        }
        ButtonState::Released => RectangleOrigin::BottomLeft,
    };
    let rect = shapes::Rectangle {
        extents,
        origin,
        ..shapes::Rectangle::default()
    };

    GeometryBuilder::build_as(&rect).0
}

pub fn handle_box_tool_message(
    world: &mut World,
    message: &ToolHandlerMessage,
    responses: &mut VecDeque<Message>,
) {
    match message {
        ToolHandlerMessage::OnActivate => {
            debug_log!("BoxTool::OnActivate");
            responses.push_back(FrontendMessage::SetCursor(Cursors::Box).into());
        }
        ToolHandlerMessage::OnDeactivate => {
            debug_log!("BoxTool::OnDeactivate");
        }
        ToolHandlerMessage::Input(input_message) => match input_message {
            drag @ InputMessage::DragMove {
                world_pressed,
                world_offset,
                modifiers,
                ..
            }
            | drag @ InputMessage::DragEnd {
                world_pressed,
                world_offset,
                modifiers,
                ..
            } => {
                let active_doc_entity = world.query_filtered::<Entity, With<ActiveDocumentTag>>().single(world);

                let is_final = matches!(drag, InputMessage::DragEnd { .. });
                let name = if is_final { "Box" } else { "TransientBox" };
                let path = build_box_path(modifiers.shift, modifiers.alt, world_offset);

                let spawner = VectorObjectSpawner::new()
                    .with_origin(world_pressed.xy())
                    .with_z_index(1.)
                    .with_name(name.to_string())
                    .with_fill(Fill::color(Color::GRAY))
                    .with_transient(!is_final)
                    .with_shape_editable(is_final)
                    .with_selectable(is_final)
                    .with_movable(true)
                    .with_parent(active_doc_entity)
                    .with_path(path)
                    .with_extra(|builder| {
                        builder.insert(VectorObjectTag);
                    });

                let operation = ElementOperation::CreateVectorObject(Arc::new(spawner));
                let command = if is_final {
                    EmbCommand::new(operation.into())
                } else {
                    EmbCommand::new_transient(operation.into())
                };
                responses.push_back(DocMessage::PerformOperation(command).into());
            }
            InputMessage::PointerClick { world, .. } => {}
            _ => {}
        },
    }
}
