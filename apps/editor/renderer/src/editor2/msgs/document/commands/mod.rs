pub mod select_commands;
pub mod element_commands;

use std::{collections::VecDeque, fmt::Debug, sync::Arc};

use bevy::prelude::World;

use crate::{debug_log, editor2::Message};

use self::{select_commands::{handle_select_command_message, SelectCommandMessage, SelectOperation}, element_commands::{ElementOperation, ElementCommandMessage, handle_element_command_message}};

use super::{DocMessage, DocumentResource};

#[derive(Clone, Debug, Copy)]
pub enum OperationType {
    // Operations that do-not push to undo/redo stack
    Transient,
    // Operations that push to undo stack and clear redo stack
    Default,
    // Operation that moves from undo stack to redo stack
    Undo,
    // Operation that moves from redo stack to undo stack
    Redo,
}

#[derive(Clone, Debug)]
pub struct GenericCommand<T: Debug> {
    /// Document to perform operation on, if none provided, will use current
    pub document_id: Option<usize>,
    pub operation_type: OperationType,
    pub operation: T,
}

impl<T: Debug> GenericCommand<T> {
    pub fn new<T2: Debug>(operation: T2) -> GenericCommand<T2> {
        GenericCommand::<T2> {
            document_id: None,
            operation_type: OperationType::Default,
            operation,
        }
    }
    pub fn new_transient<T2: Debug>(operation: T2) -> GenericCommand<T2> {
        GenericCommand::<T2> {
            document_id: None,
            operation_type: OperationType::Transient,
            operation,
        }
    }
}

pub fn handle_command_response<'env_borrow, T: Debug>(
    prev: &GenericCommand<T>,
    responses: &mut VecDeque<Message>,
    inverse_op: Box<dyn Fn() -> Operation + 'env_borrow>,
) {
    let response = match prev.operation_type {
        OperationType::Redo | OperationType::Default => {
            Some(DocMessage::PushUndo(GenericCommand {
                document_id: prev.document_id,
                operation_type: OperationType::Undo,
                operation: inverse_op(),
            }))
        }
        OperationType::Undo => {
            Some(DocMessage::PushRedo(GenericCommand {
                document_id: prev.document_id,
                operation_type: OperationType::Redo,
                operation: inverse_op(),
            }))

        },
        OperationType::Transient => None,
    };
    if let Some(response) = response {
        responses.push_back(response.into())
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Element(ElementOperation),
    Select(SelectOperation),
}
impl From<SelectOperation> for Operation {
    fn from(value: SelectOperation) -> Self {
        Operation::Select(value)
    }
}
impl From<ElementOperation> for Operation {
    fn from(value: ElementOperation) -> Self {
        Operation::Element(value)
    }
}

pub type EmbCommand = GenericCommand<Operation>;

pub enum PostCommandMessage {
    PushUndo(EmbCommand),
    PushRedo(EmbCommand),
}

pub fn handle_command_message(
    world: &mut World,
    message: &EmbCommand,
    responses: &mut VecDeque<Message>,
) {
    if !matches!(message.operation_type, OperationType::Transient) {
        debug_log!("DocumentMessage::PerformOperation({:?})", message);
    }
    // Clears redo stack if it's a user input
    if matches!(message.operation_type, OperationType::Default)
        && !matches!(message.operation, Operation::Select(_))
    {
        if let Some(mut res) = world.get_resource_mut::<DocumentResource>() {
            res.get_active_document_mut()
                .expect(&format!("Can't perform operation without active document. Operation {message:?}"))
                .redo_stack
                .clear();
        }
    }
    match message.operation.clone() {
        Operation::Select(select_operation) => {
            let msg = SelectCommandMessage {
                operation_type: message.operation_type,
                document_id: message.document_id,
                operation: select_operation,
            };
            handle_select_command_message(world, msg, responses);
        }
        Operation::Element(element_operation) => {
            let msg = ElementCommandMessage {
                operation_type: message.operation_type,
                document_id: message.document_id,
                operation: element_operation,
            };
            handle_element_command_message(world, msg, responses);
        },
    }
}
