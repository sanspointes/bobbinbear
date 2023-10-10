use bevy::prelude::Resource;
use crossbeam::channel::{Receiver, Sender};
use serde::{Serialize, Deserialize};

use crate::types::Cursors;

use super::{msgs::DocumentMetaData, Message, msgs::Tool};

#[derive(Clone, Debug, Serialize, Deserialize /*, specta::Type */)]
pub enum FrontendMessageTypes {
    Init,
    DocumentsUpdated,
}

#[derive(Clone, Debug, Serialize, Deserialize /*, specta::Type */)]
pub struct DocumentsUpdatedModel {
    msg_type: FrontendMessageTypes,
    active_document: Option<usize>,
    documents: Vec<DocumentMetaData>,
}
impl DocumentsUpdatedModel {
    pub fn new(documents: Vec<DocumentMetaData>, active_document: Option<usize>) -> Self {
        Self {
            msg_type: FrontendMessageTypes::DocumentsUpdated,
            active_document,
            documents,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize /*, specta::Type */)]
pub struct InitModel;

#[derive(Clone, Debug, Serialize, Deserialize /*, specta::Type */)]
pub enum FrontendMessage {
    Log(String),
    Warn(String),
    Error(String),
    Init(InitModel),
    DocumentsUpdated(DocumentsUpdatedModel),
    SetCursor(Cursors),
    SetCurrentTool(Tool),
}

#[derive(Resource, Debug)]
// Can receive messages from the javascript layer
pub struct FrontendReceiver(pub Receiver<Message>);

#[derive(Resource)]
/// Can send messages to the javascript layer
pub struct FrontendSender(pub Sender<FrontendMessage>);

