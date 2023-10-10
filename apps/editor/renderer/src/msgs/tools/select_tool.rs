use std::collections::VecDeque;

use bevy::{input::ButtonState, prelude::*, utils::HashSet};

use crate::{
    msgs::{frontend::FrontendMsg, Message},
    plugins::input_plugin::InputMessage,
    types::BBCursor,
};

use super::ToolHandlerMessage;

type DragStartModel = Vec<(Entity, Vec2)>;

#[derive(Resource, Default, Debug)]
pub struct SelectToolRes {
    drag_model: Option<DragStartModel>,
    prev_hovers: HashSet<Entity>,
}

pub fn msg_handler_select_tool(
    world: &mut World,
    message: &ToolHandlerMessage,
    responses: &mut VecDeque<Message>,
) {
    // let mut select_sys_state = SystemState::<(
    //     // Selectables
    //     Query<
    //         (
    //             Entity,
    //             &Parent,
    //             &SelectedState,
    //             &mut HoveredState,
    //             &Transform,
    //         ),
    //         With<SelectableTag>,
    //     >,
    //     // Raycaster
    //     Query<&RaycastSource<RaycastSelectable>>,
    //     // Prev hovers
    //     ResMut<SelectToolRes>,
    // )>::new(world);

    match message {
        ToolHandlerMessage::OnActivate => {
            println!("BoxTool::OnActivate");
            responses.push_back(FrontendMsg::SetCursor(BBCursor::Default).into());
        }
        ToolHandlerMessage::OnDeactivate => {
            println!("BoxTool::OnDeactivate");
        }
        ToolHandlerMessage::Input(input_message) => {
            match input_message {
                InputMessage::PointerMove { .. } => {
                    // let (mut selectables, rc_source, mut res) = select_sys_state.get_mut(world);
                    // let src = rc_source.single();
                    //
                    // let cur_hovers: HashSet<Entity> =
                    //     src.intersections().iter().map(|v| v.0).collect();
                    // let (to_over, to_exit) = {
                    //     let to_over: Vec<_> = cur_hovers
                    //         .iter()
                    //         .filter(|v| !res.prev_hovers.contains(*v))
                    //         .collect();
                    //     let to_exit: Vec<_> = res
                    //         .prev_hovers
                    //         .iter()
                    //         .filter(|v| !cur_hovers.contains(*v))
                    //         .cloned()
                    //         .collect();
                    //     (to_over, to_exit)
                    // };
                    // for to_hover_exit in to_exit {
                    //     let get_result = selectables.get_mut(to_hover_exit);
                    //     match get_result {
                    //         Ok((_, _, _, mut hovered_state, _)) => {
                    //             hovered_state.set_if_neq(HoveredState::Unhovered);
                    //         }
                    //         Err(reason) => {
                    //             println!(
                    //                 "DBG: Unable to hover over {:?}.  Reason: {}",
                    //                 to_hover_exit, reason
                    //             );
                    //         }
                    //     }
                    //     res.prev_hovers.remove(&to_hover_exit);
                    // }
                    // for to_hover_over in to_over {
                    //     let get_result = selectables.get_mut(*to_hover_over);
                    //     match get_result {
                    //         Ok((_, _, _, mut hovered_state, _)) => {
                    //             hovered_state.set_if_neq(HoveredState::Unhovered);
                    //         }
                    //         Err(reason) => {
                    //             println!(
                    //                 "DBG: Unable to hover exit {:?}.  Reason: {}",
                    //                 to_hover_over, reason
                    //             );
                    //         }
                    //     }
                    //     res.prev_hovers.insert(*to_hover_over);
                    // }
                    //
                    // res.prev_hovers = cur_hovers;
                }
                InputMessage::PointerDown { modifiers, .. } => {
                    // let (selectables, rc_source, _) = select_sys_state.get_mut(world);
                    // let src = rc_source.single();
                    // let selected_entities: Vec<_> = selectables
                    //     .iter()
                    //     .filter(|(_, _, state, _, _)| matches!(state, SelectedState::Selected))
                    //     .map(|x| x.0)
                    //     .collect();
                    //
                    // // if let Some((entity, _)) = src.get_nearest_intersection() {
                    // //     // If the user is selecting an unselected object and shift isn't
                    // //     // pressed, deselect all.
                    // //     if !selected_entities.iter().any(|e| *e == entity) {
                    // //         // Deselect all existing entities unless shift is pressed
                    // //         let to_deselect = if matches!(modifiers.shift, ButtonState::Released) {
                    // //             selected_entities
                    // //         } else {
                    // //             vec![]
                    // //         };
                    // //         let operation = ElementOperation::ChangeSelection {
                    // //             to_select: vec![entity],
                    // //             to_deselect,
                    // //         };
                    // //         responses.push_back(
                    // //             DocMessage::PerformOperation(EmbCommand::new(operation.into()))
                    // //                 .into(),
                    // //         );
                    // //     }
                    // // } else {
                    // //     let operation = ElementOperation::ChangeSelection {
                    // //         to_select: vec![],
                    // //         to_deselect: selected_entities,
                    // //     };
                    // //     responses.push_back(
                    // //         DocMessage::PerformOperation(EmbCommand::new(operation.into())).into(),
                    // //     );
                    // // }
                }

                // Drag to Move
                InputMessage::DragStart { .. } => {
                    // let (selectables, _, mut res) = select_sys_state.get_mut(world);
                    // let selected_entities: Vec<_> = selectables
                    //     .iter()
                    //     .filter(|(_, _, state, _, _)| matches!(state, SelectedState::Selected))
                    //     .collect();
                    // // Filter out entities that are children of other entities in the list.
                    // // Otherwise the transform will double up
                    // let to_move: Vec<_> = selected_entities
                    //     .iter()
                    //     .filter(|(_, parent, _, _, _)| {
                    //         !selected_entities
                    //             .iter()
                    //             .any(|(entity2, _, _, _, _)| parent.get() == *entity2)
                    //     })
                    //     .map(|(entity, _, _, _, transform)| {
                    //         (entity.clone(), transform.translation.xy())
                    //     })
                    //     .collect();
                    //
                    // res.drag_model = Some(to_move);
                }
                drag @ InputMessage::DragMove { world_offset, .. }
                | drag @ InputMessage::DragEnd { world_offset, .. } => {
                    // let (_, _, mut res) = select_sys_state.get_mut(world);
                    // if let Some(model) = res.drag_model.clone() {
                    //     let model: Vec<_> = model
                    //         .iter()
                    //         .map(|(entity, start_pos)| (*entity, world_offset.xy() + *start_pos))
                    //         .collect();
                    //
                    //     let is_final = matches!(drag, InputMessage::DragEnd { .. });
                    //
                    //     let operation = ElementOperation::TranslateObjects(model);
                    //     let command = if is_final {
                    //         EmbCommand::new(operation.into())
                    //     } else {
                    //         EmbCommand::new_transient(operation.into())
                    //     };
                    //     responses.push_back(DocMessage::PerformOperation(command).into());
                    //
                    //     if matches!(drag, InputMessage::DragEnd { .. }) {
                    //         res.drag_model = None;
                    //     }
                    // }
                }

                InputMessage::Keyboard {
                    pressed,
                    key,
                    modifiers,
                } => match (pressed, key) {
                    (ButtonState::Released, KeyCode::Delete)
                    | (ButtonState::Released, KeyCode::Back) => {
                        // let (selectables, _, mut res) = select_sys_state.get_mut(world);
                        // let selected_entities: Vec<_> = selectables
                        //     .iter()
                        //     .filter(|(_, _, state, _, _)| matches!(state, SelectedState::Selected))
                        //     .map(|v| v.0)
                        //     .collect();
                        //
                        // let operation = ElementOperation::Delete(selected_entities);
                        // let command = EmbCommand::new(operation.into());
                        // responses.push_back(DocMessage::PerformOperation(command).into());
                    }
                    (_, _) => {}
                },
                _ => {}
            }
        }
    }
}
