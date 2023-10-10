pub mod commands;

use std::{collections::VecDeque, sync::Arc};

use bevy::{ecs::system::SystemState, prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::{prelude::*, shapes};
use serde::{Deserialize, Serialize};

use crate::editor2::{
    camera::CameraMessage,
    constants::CANVAS_Z_INDEX,
    entities::{vector::VectorObjectSpawner, ActiveDocumentTag, DocumentTag, NeedsDelete},
    frontend::{DocumentsUpdatedModel, FrontendMessage},
    Message,
};

use self::commands::{handle_command_message, EmbCommand};

#[derive(Clone, Debug)]
pub enum DocMessage {
    SetActive(usize),
    Create { size: Vec2, name: String },
    Delete(usize),
    Resize { id: usize, new_size: Vec2 },

    PerformOperation(EmbCommand),
    PerformBatchOperation(Arc<Vec<EmbCommand>>),
    PerformUndo,
    PerformRedo,
    PushUndo(EmbCommand),
    PushRedo(EmbCommand),
}

/// The document struct contains all of the data to render
///
/// * `metadata`:
/// * `camera`:
pub struct Document {
    pub entity: Entity,
    pub metadata: DocumentMetaData,

    pub undo_stack: Vec<EmbCommand>,
    pub redo_stack: Vec<EmbCommand>,
}

#[derive(Component, Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentMetaData {
    pub size: Vec2,
    pub name: String,
    pub id: usize,
}

#[derive(Resource, Default)]
pub struct DocumentResource {
    next_document_id: usize,
    documents: Vec<Document>,
    active_document: Option<usize>,
}

impl DocumentResource {
    pub fn get_active_document(&self) -> Option<&Document> {
        if let Some(active_document) = self.active_document {
            return self
                .documents
                .iter()
                .find(|v| v.metadata.id == active_document);
        };
        None
    }
    pub fn get_active_document_mut(&mut self) -> Option<&mut Document> {
        if let Some(active_document) = self.active_document {
            return self
                .documents
                .iter_mut()
                .find(|v| v.metadata.id == active_document);
        };
        None
    }

    pub fn update_frontend(&mut self, responses: &mut VecDeque<Message>) {
        let msg = Message::Frontend(FrontendMessage::DocumentsUpdated(
            DocumentsUpdatedModel::new(
                self.documents.iter().map(|d| d.metadata.clone()).collect(),
                self.active_document,
            ),
        ));

        // Notify frontend of changes to documents
        responses.push_back(msg);
    }
}

pub struct DocumentPlugin;

impl Plugin for DocumentPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DocumentResource::default())
            // .add_plugin(DocumentSelectPlugin)
            // .add_plugin(DocumentElementsPlugin)
        ;
    }
}

pub fn handle_doc_message(
    world: &mut World,
    message: &DocMessage,
    responses: &mut VecDeque<Message>,
) {
    let mut create_sys_state: SystemState<ResMut<DocumentResource>> = SystemState::new(world);
    let mut window_sys_state: SystemState<(
        Query<&Window, With<PrimaryWindow>>,
        EventWriter<CameraMessage>,
    )> = SystemState::new(world);
    match message {
        DocMessage::Create { size, name } => {
            let shape = shapes::Rectangle {
                extents: *size,
                ..Default::default()
            };
            // Unset the previous active document
            if let Ok(active_doc_entity) = world
                .query_filtered::<Entity, With<ActiveDocumentTag>>()
                .get_single(world)
            {
                world.entity_mut(active_doc_entity).remove::<ActiveDocumentTag>();
            }

            let entity = VectorObjectSpawner::new()
                .with_z_index(CANVAS_Z_INDEX)
                .with_shape_editable(false)
                .with_name(name.to_string())
                .with_fill(Fill::color(Color::WHITE))
                .with_shape_editable(false)
                .with_path(GeometryBuilder::build_as(&shape).0)
                .with_extra(|builder| {
                    builder.insert((DocumentTag, ActiveDocumentTag));
                })
                .spawn_with_world(world);

            // Add to store
            let mut res = create_sys_state.get_mut(world);
            let id = res.next_document_id;
            res.next_document_id += 1;
            res.active_document = Some(id);

            let new_document = Document {
                entity,
                metadata: DocumentMetaData {
                    size: size.clone(),
                    name: name.clone(),
                    id,
                },
                undo_stack: vec![],
                redo_stack: vec![],
            };
            res.documents.push(new_document);
        }
        DocMessage::Delete(id) => {
            let idx = {
                let res = create_sys_state.get_mut(world);
                res.documents
                    .iter()
                    .position(|view| view.metadata.id == *id)
            };
            if let Some(idx) = idx {
                let commands = world
                    .get_resource::<DocumentResource>()
                    .and_then(|doc_resource| doc_resource.documents.get(idx))
                    .and_then(|doc| Some(doc.entity))
                    .and_then(|entity| world.get_entity_mut(entity));
                if let Some(mut commands) = commands {
                    commands.insert(NeedsDelete);

                    let mut res = world.get_resource_mut::<DocumentResource>()
                        .expect("DocMessage::Delete() Could not get DocumentResource. This should never happen.");
                    res.documents.remove(idx);
                    res.update_frontend(responses);
                }
            }
        }
        DocMessage::Resize { id, new_size } => {
            let mut res = create_sys_state.get_mut(world);

            let vp = res.documents.iter_mut().find(|vp| vp.metadata.id == *id);
            if let Some(vp) = vp {
                vp.metadata.size = *new_size;
            }
            res.update_frontend(responses);
        }
        DocMessage::SetActive(id) => {
            let mut res = create_sys_state.get_mut(world);

            res.active_document = Some(*id);
            res.update_frontend(responses);
            // TODO: Also needs to be re-run on canvas resize.
            if let Some(doc_size) = res
                .get_active_document()
                .map(|doc| doc.metadata.size.clone())
            {
                let (q_window, mut camera_writer) = window_sys_state.get_mut(world);
                let window = q_window.single();
                let padding = Vec2::new(window.width(), window.height()) * 0.8;
                camera_writer.send(CameraMessage::UpdateBounds {
                    rect: Rect::from_center_size(Vec2::ZERO, doc_size),
                    padding,
                });
            }
        }
        DocMessage::PerformUndo => {
            let maybe_command = world
                .get_resource_mut::<DocumentResource>()
                .and_then(|mut res| {
                    res.get_active_document_mut()
                        .expect(&format!("Can't perform operation without active document. Operation {message:?}"))
                        .undo_stack
                        .pop()
                });
            if let Some(command) = maybe_command {
                responses.push_back(DocMessage::PerformOperation(command).into());
            }
        }
        DocMessage::PerformRedo => {
            let maybe_command = world
                .get_resource_mut::<DocumentResource>()
                .and_then(|mut res| {
                    res.get_active_document_mut()
                        .expect(&format!("Can't perform operation without active document. Operation {message:?}"))
                        .redo_stack
                        .pop()
                });
            if let Some(command) = maybe_command {
                responses.push_back(DocMessage::PerformOperation(command).into());
            }
        }
        DocMessage::PerformOperation(command) => {
            handle_command_message(world, command, responses);
        },
        DocMessage::PerformBatchOperation(commands) => {
            println!("Performing batch operation...");
            for command in commands.iter() {
                handle_command_message(world, command, responses);
            }
        }
        DocMessage::PushUndo(command) => {
            let mut res = world
                .get_resource_mut::<DocumentResource>()
                .expect("Can't push undo because can't get world.");
            res.get_active_document_mut()
                .expect("Can't push undo without active document.")
                .undo_stack
                .push(command.clone());
        }
        DocMessage::PushRedo(command) => {
            let mut res = world
                .get_resource_mut::<DocumentResource>()
                .expect("Can't push redo because can't get world.");
            res.get_active_document_mut()
                .expect("Can't push redo without active document.")
                .redo_stack
                .push(command.clone());
        }
    }
}
