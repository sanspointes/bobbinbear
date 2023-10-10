use std::collections::VecDeque;

use bevy::{ecs::system::SystemState, prelude::*};

use crate::editor2::{
    entities::{SelectableTag, SelectedState},
    Message,
};

use super::{GenericCommand, handle_command_response, Operation};

#[derive(Debug, Clone)]
pub enum SelectOperation {
    SelectObjects { entities: Vec<Entity> },
    DeselectObjects { entities: Vec<Entity> },
}
pub(crate) type SelectCommandMessage = GenericCommand<SelectOperation>;

pub fn handle_select_command_message(
    world: &mut World,
    message: SelectCommandMessage,
    responses: &mut VecDeque<Message>,
) {
    let mut selectables_sys_state: SystemState<Query<&mut SelectedState, With<SelectableTag>>> =
        SystemState::new(world);
    match &message.operation {
        SelectOperation::SelectObjects { entities } => {
            let mut selectables = selectables_sys_state.get_mut(world);
            for entity in entities.iter() {
                if let Ok(mut state) = selectables.get_mut(*entity) {
                    *state = SelectedState::Selected;
                }
            }

            handle_command_response(
                &message,
                responses,
                Box::new(move || {
                    SelectOperation::DeselectObjects {
                        entities: entities.clone(),
                    }.into()
                }),
            );
        }
        SelectOperation::DeselectObjects { entities } => {
            let mut selectables = selectables_sys_state.get_mut(world);
            for entity in entities.iter() {
                if let Ok(mut state) = selectables.get_mut(*entity) {
                    *state = SelectedState::Unselected;
                }
            }

            handle_command_response(
                &message,
                responses,
                Box::new(move || {
                    SelectOperation::DeselectObjects {
                        entities: entities.clone(),
                    }.into()
                }),
            );
        }
    }
}
