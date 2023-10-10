use bevy::prelude::{Resource, Event};
use crossbeam_channel::{Receiver, Sender};
use serde::{Serialize, Deserialize};

use crate::types::Cursors;

use super::{Message, msgs::Tool};

#[derive(Clone, Debug, Serialize, Deserialize /*, specta::Type */)]
pub enum FrontendMessageTypes {
    Init,
    DocumentsUpdated,
}

#[derive(Clone, Debug, Serialize, Deserialize /*, specta::Type */)]
pub struct InitModel;

#[derive(Event, Clone, Debug, Serialize, Deserialize /*, specta::Type */)]
pub enum FrontendMessage {
    Log(String),
    Warn(String),
    Error(String),
    Init(InitModel),
    SetCursor(Cursors),
    SetCurrentTool(Tool),
}

#[derive(Resource, Debug)]
// Can receive messages from the javascript layer
pub struct FrontendReceiver(pub Receiver<Message>);

#[derive(Resource)]
/// Can send messages to the javascript layer
pub struct FrontendSender(pub Sender<FrontendMessage>);

