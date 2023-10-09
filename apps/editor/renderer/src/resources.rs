use bevy::prelude::Resource;
use crossbeam_channel::{Receiver, Sender};

use crate::messages::{Msg, frontend::FrontendMsg};

#[derive(Resource)]
pub struct FrontendReceiver(pub Receiver<Msg>);

#[derive(Resource)]
pub struct FrontendSender(pub Sender<FrontendMsg>);
