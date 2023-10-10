use std::{collections::VecDeque, ops::Mul, sync::Arc};

use bevy::{ecs::system::SystemState, math::Vec3Swizzles, prelude::*};
use bevy_prototype_lyon::{path::PathBuilder, prelude::*};
use debug_panic::debug_panic;

use crate::{
    debug_log,
    editor2::{
        entities::{
            vector::{Ordered, PathSegment, VecNodeTag, VectorObjectSpawner, VectorObjectTag},
            ActiveDocumentTag, Bounded,
        },
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

#[derive(Component, Debug)]
pub struct ActivePenObjectTag;

fn get_child_node_with_index(
    world: &mut World,
    children: &Vec<Entity>,
    index: i32,
) -> Option<Entity> {
    let mut q_nodes = world.query_filtered::<(Entity, &Ordered), With<VecNodeTag>>();
    for (entity, order) in q_nodes.iter_many(world, children) {
        if order.0 as i32 == index {
            return Some(entity)
        }
    }
    return None;
}

pub fn handle_pen_tool_message(
    world: &mut World,
    message: &ToolHandlerMessage,
    responses: &mut VecDeque<Message>,
) {
    match message {
        ToolHandlerMessage::OnActivate => {
            debug_log!("PenTool::OnActivate");
            responses.push_back(FrontendMessage::SetCursor(Cursors::Pen).into());
        }
        ToolHandlerMessage::OnDeactivate => {
            debug_log!("PenTool::OnDeactivate");
        }
        ToolHandlerMessage::Input(input_message) => match input_message {
            InputMessage::DragStart { world_offset, .. } => {
                let parent_result = {
                    let mut q_active_obj =
                        world.query_filtered::<(Entity, &Children), With<ActivePenObjectTag>>();
                    q_active_obj
                        .get_single(world)
                        .map(|(e, c)| (e, c.iter().cloned().collect::<Vec<_>>()))
                };
                if let Ok((parent, children)) = parent_result {
                    let active_node_result = {
                        let mut q_active_order_index =
                            world.query::<(&Ordered, &VecNodeTag, &PathSegment)>();
                        let active = q_active_order_index
                            .iter_many(world, &children)
                            .find(|(_, tag, _)| matches!(tag, VecNodeTag::Active));
                        active.map(|(order, _, path_seg)| (order.0, path_seg.clone()))
                    };

                    if let Some((index, path_seg)) = active_node_result {
                        let point = path_seg.get_point();
                        let index: i32 = index
                            .try_into()
                            .expect("PenTool: Cannot convert the active node index to an i32.");
                        let offset_xy = *point + world_offset.xy();
                        let inv_offset_xy = *point + world_offset.xy().mul(Vec2::NEG_ONE);

                        debug_log!("Placing before {offset_xy:?} and after {inv_offset_xy:?} near original {point:?} with {world_offset:?}");

                        let mut operations = Vec::<EmbCommand>::new();
                        operations.push(EmbCommand::new_transient(
                            ElementOperation::InsertVecNode {
                                parent,
                                index: index + 1,
                                position: offset_xy,
                                path_seg: PathSegment::Control(offset_xy),
                            }
                            .into(),
                        ));
                        if index > 0 {
                            operations.push(EmbCommand::new_transient(
                                ElementOperation::InsertVecNode {
                                    parent,
                                    index,
                                    position: inv_offset_xy,
                                    path_seg: PathSegment::Control(inv_offset_xy),
                                }
                                .into(),
                            ));
                        }

                        let message = DocMessage::PerformBatchOperation(Arc::new(operations));
                        responses.push_back(message.into());
                    }
                }
            }
            InputMessage::DragMove { world_offset, .. } => {
                let parent_result = {
                    let mut q_active_obj =
                        world.query_filtered::<&Children, With<ActivePenObjectTag>>();
                    q_active_obj
                        .get_single(world)
                        .map(|c| c.iter().cloned().collect::<Vec<_>>())
                };
                if let Ok(children) = parent_result {
                    let active_node_result = {
                        let mut q_active_order_index = world.query_filtered::<(
                            &Ordered,
                            &VecNodeTag,
                            &PathSegment,
                        ), With<VecNodeTag>>(
                        );
                        let active = q_active_order_index
                            .iter_many(world, &children)
                            .find(|(_, tag, _)| matches!(tag, VecNodeTag::Active));
                        active.map(|(order, _, path_seg)| (order.0, path_seg.get_point().clone()))
                    };

                    if let Some((index, point)) = active_node_result {
                        let mut to_translate = Vec::<(Entity, Vec2)>::new();

                        let inv_offset_xy = point + world_offset.xy().mul(Vec2::NEG_ONE);
                        if let Some(before) = get_child_node_with_index(world, &children, index as i32 - 1) {
                            to_translate.push((before, inv_offset_xy));
                        }

                        let offset_xy = point + world_offset.xy();
                        if let Some(after) = get_child_node_with_index(world, &children, index as i32 + 1) {
                            to_translate.push((after, offset_xy));
                        }

                        if to_translate.len() > 0 {
                            let command = EmbCommand::new_transient(
                                ElementOperation::TranslateObjects(to_translate).into(),
                            );
                            let message = DocMessage::PerformOperation(command);
                            responses.push_back(message.into());
                        } else {
                            debug_panic!("PenTool:DragMove on active node but can't find post node to translate.")
                        }
                    } else {
                        debug_panic!("PenTool: No active node while dragging.");
                    }
                } else {
                    debug_panic!("PenTool: No active pen object while dragging.");
                }
            }
            InputMessage::PointerDown {
                world: world_pressed,
                ..
            } => {
                let mut click_sys_state = SystemState::<(
                    Query<Entity, With<ActiveDocumentTag>>,
                    // Parent
                    Query<(Entity, &Children, &Bounded), With<ActivePenObjectTag>>,
                    // Child nodes
                    Query<(&Ordered, &VecNodeTag, &PathSegment)>,
                )>::new(world);
                let (q_active_doc, q_active_object, q_all_nodes) = click_sys_state.get_mut(world);
                let active_doc_entity = q_active_doc.single();

                // If there is a VectorObjectTag with ActivePenObjectTag component, add to it.
                if let Ok((parent, children, bounds)) = q_active_object.get_single() {
                    // Index is either index of active node + 1 or added to end of node list.
                    let mut ordered_children: Vec<_> = q_all_nodes.iter_many(children).collect();
                    ordered_children.sort_by(|(a, _, _), (b, _, _)| a.0.partial_cmp(&b.0).unwrap());

                    // Calculate the index to insert the next point into.  Either the next spot
                    // after the last active VecNodeTag (skipping control points.) or the end of
                    // the list
                    let index: usize = ordered_children.iter().fold(0, |acc, (order, _, _)| acc.max(order.0)) + 1;

                    match bounds {
                        Bounded::Calculated { min, .. } => {
                            let pos = world_pressed.xy() - *min;
                            debug_log!("Adding new node {min:?} {world_pressed:?} -> {pos:?}");
                            let new_node_op = ElementOperation::InsertVecNode {
                                parent,
                                index: index.try_into().expect(
                                    "PenTool: Cannot convert the active node index to an i32.",
                                ),
                                position: pos,
                                path_seg: PathSegment::Point(pos),
                            };
                            let make_active_op = ElementOperation::SetActiveVecNode {
                                parent,
                                new_active: index,
                            };

                            responses.push_back(
                                DocMessage::PerformBatchOperation(Arc::new(vec![
                                    EmbCommand::new(new_node_op.into()),
                                    EmbCommand::new(make_active_op.into()),
                                ]))
                                .into(),
                            );
                        }
                        Bounded::NeedsCalculate => {
                            debug_log!("handle_pen_tool_message: Attempted to add new node to {parent:?} but bounds has not been calculated yet.  Ignoring.");
                        }
                    }

                // Else add a new vector object.
                } else {
                    debug_log!("Adding new object {world_pressed:?}");
                    // Create path with a single vector node
                    let mut pb = PathBuilder::new();
                    pb.move_to(Vec2::ZERO);

                    let spawner = VectorObjectSpawner::new()
                        .with_origin(world_pressed.xy())
                        .with_z_index(1.)
                        .with_name("Shape".to_string())
                        .with_path(pb.build().0)
                        .with_fill(Fill::color(Color::GRAY))
                        .with_stroke(Stroke::new(Color::BLACK, 1.))
                        .with_shape_editable(true)
                        .with_selectable(true)
                        .with_movable(true)
                        .with_parent(active_doc_entity)
                        .with_extra(|builder| {
                            builder.insert((VectorObjectTag, ActivePenObjectTag));
                        })
                        .with_extra_for_nodes(|builder| {
                            builder.insert(VecNodeTag::Active);
                        });

                    let operation = ElementOperation::CreateVectorObject(Arc::new(spawner));
                    let command = EmbCommand::new(operation.into());
                    responses.push_back(DocMessage::PerformOperation(command).into());
                }
            }
            _ => {}
        },
    }
}

/// Checks the path segment type at the given order index
///
/// * `world`:
/// * `vec_nodes_to_check`:
/// * `order_index`:
fn check_vec_obj_path_node(
    world: &mut World,
    vec_nodes_to_check: &Vec<Entity>,
    order_index: usize,
) -> Result<PathSegment, ()> {
    let mut query = world.query_filtered::<(&Ordered, &PathSegment), With<VecNodeTag>>();
    for (order, path_seg) in query.iter_many(world, vec_nodes_to_check) {
        if order.0 == order_index {
            return Ok(path_seg.clone());
        }
    }
    Err(())
}
